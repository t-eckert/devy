#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use axum::{
    routing::{delete, get, post},
    Router,
};
use shuttle_secrets::SecretStore;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
mod api {
    pub mod auth {
        use crate::auth::Session;
        use crate::entities::{Profile, User};
        use crate::store::Store;
        use axum::{
            extract::{Query, State},
            response::Redirect,
        };
        use jsonwebtoken::{encode, EncodingKey, Header};
        use std::collections::HashMap;
        pub async fn login(State(store): State<Store>) -> Redirect {
            Redirect::permanent(&store.auth_client.login_url())
        }
        /// callback is the endpoint that GitHub redirects to after a successful login
        /// It exchanges the code for a token and then fetches the user from GitHub.
        /// If the user doesn't exist in the database, it creates a new user.
        /// It then creates a session for the user and returns a JWT.
        pub async fn callback(
            State(store): State<Store>,
            Query(params): Query<HashMap<String, String>>,
        ) -> Redirect {
            let code = match params.get("code") {
                Some(code) => code,
                None => return Redirect::to(&store.auth_client.post_auth_redirect_uri()),
            };
            let access_token = match store
                .auth_client
                .exchange_code(code.to_string())
                .await
            {
                Ok(token) => token,
                Err(err) => {
                    {
                        ::std::io::_eprint(
                            format_args!("Failed to exchange code: {0}\n", err),
                        );
                    };
                    return Redirect::to(&store.auth_client.post_auth_redirect_uri());
                }
            };
            let github_user = match store
                .auth_client
                .fetch_github_user(access_token.clone())
                .await
            {
                Ok(user) => user,
                Err(err) => {
                    {
                        ::std::io::_eprint(
                            format_args!("Failed to fetch GitHub user: {0}\n", err),
                        );
                    };
                    return Redirect::to(&store.auth_client.post_auth_redirect_uri());
                }
            };
            let user = match User::upsert_from_github_user(
                    &store.pool,
                    github_user.clone(),
                )
                .await
            {
                Ok(user) => user,
                Err(err) => {
                    {
                        ::std::io::_eprint(
                            format_args!("Failed to upsert user: {0}\n", err),
                        );
                    };
                    return Redirect::to(&store.auth_client.post_auth_redirect_uri());
                }
            };
            let user_id = match user.id.clone() {
                Some(id) => id,
                None => {
                    {
                        ::std::io::_eprint(format_args!("User ID is None\n"));
                    };
                    return Redirect::to(&store.auth_client.post_auth_redirect_uri());
                }
            };
            let profile = match Profile::upsert_from_github_user(
                    &store.pool,
                    user_id,
                    github_user,
                )
                .await
            {
                Ok(profile) => profile,
                Err(err) => {
                    {
                        ::std::io::_eprint(
                            format_args!("Failed to upsert profile: {0}\n", err),
                        );
                    };
                    return Redirect::to(&store.auth_client.post_auth_redirect_uri());
                }
            };
            let session = Session::new(user, profile, access_token.clone());
            let jwt = match encode(
                &Header::default(),
                &session,
                &EncodingKey::from_secret(access_token.secret().as_ref()),
            ) {
                Ok(token) => token,
                Err(err) => {
                    {
                        ::std::io::_eprint(
                            format_args!("Failed to encode JWT: {0}\n", err),
                        );
                    };
                    return Redirect::to(&store.auth_client.post_auth_redirect_uri());
                }
            };
            Redirect::to(
                {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}?token={1}",
                            &store.auth_client.post_auth_redirect_uri(),
                            jwt,
                        ),
                    );
                    res
                }
                    .as_str(),
            )
        }
    }
    pub mod blogs {
        use crate::{
            entities::{error::EntityError, Blog, Post},
            store::Store,
        };
        use axum::{
            extract::{Path, State},
            http::StatusCode, Json,
        };
        /// Get a blog by its slug.
        /// `/blogs/:blog_slug`
        pub async fn get_blog_by_blog_slug(
            State(store): State<Store>,
            Path(blog_slug): Path<String>,
        ) -> Result<Json<Blog>, StatusCode> {
            Ok(Json(Blog::get_by_slug(&store.pool, blog_slug).await.map_err(map_error)?))
        }
        /// Get all posts by a blog slug.
        /// `/blogs/:blog_slug/posts`
        pub async fn get_blog_posts_by_blog_slug(
            State(store): State<Store>,
            Path(blog_slug): Path<String>,
        ) -> Result<Json<Vec<Post>>, StatusCode> {
            Ok(
                Json(
                    Post::get_by_blog_slug(&store.pool, blog_slug)
                        .await
                        .map_err(map_error)?,
                ),
            )
        }
        /// Get a post by its blog slug and post slug.
        /// `/blogs/:blog_slug/posts/:post_slug`
        pub async fn get_post_by_blog_and_post_slug(
            State(store): State<Store>,
            Path((blog_slug, post_slug)): Path<(String, String)>,
        ) -> Result<Json<Post>, StatusCode> {
            Ok(
                Json(
                    Post::get_by_blog_and_post_slug(&store.pool, blog_slug, post_slug)
                        .await
                        .map_err(map_error)?,
                ),
            )
        }
        pub async fn upsert_blog(
            State(store): State<Store>,
            Json(blog): Json<Blog>,
        ) -> Result<Json<Blog>, StatusCode> {
            Ok(Json(blog.upsert(&store.pool).await.map_err(map_error)?))
        }
        pub async fn delete_blog(
            State(store): State<Store>,
            Path(blog_slug): Path<String>,
        ) -> Result<StatusCode, StatusCode> {
            Blog::delete_by_slug(&store.pool, blog_slug).await.map_err(map_error)?;
            Ok(StatusCode::OK)
        }
        fn map_error(e: EntityError) -> StatusCode {
            match e {
                EntityError::NotFound => StatusCode::NOT_FOUND,
                EntityError::Malformed { .. } => StatusCode::BAD_REQUEST,
                EntityError::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }
    pub mod feeds {
        use crate::{
            entities::{Feed, Post},
            store::Store,
        };
        use axum::{
            extract::{Path, Query, State},
            http::StatusCode, Json,
        };
        use std::collections::HashMap;
        pub async fn get_feed_by_id(
            Path(feed_id): Path<String>,
        ) -> Result<Json<Feed>, StatusCode> {
            match feed_id.as_str() {
                "new" => {
                    Ok(
                        Json(Feed {
                            id: "new".to_string(),
                            name: "New".to_string(),
                        }),
                    )
                }
                "popular" => {
                    Ok(
                        Json(Feed {
                            id: "popular".to_string(),
                            name: "Popular".to_string(),
                        }),
                    )
                }
                _ => Err(StatusCode::NOT_FOUND),
            }
        }
        pub async fn get_feed_posts_by_id(
            State(store): State<Store>,
            Path(feed_id): Path<String>,
            Query(params): Query<HashMap<String, String>>,
        ) -> Result<Json<Vec<Post>>, StatusCode> {
            let limit = params
                .get("limit")
                .unwrap_or(&"30".to_string())
                .parse::<i64>()
                .unwrap_or(30);
            let offset = params
                .get("offset")
                .unwrap_or(&"0".to_string())
                .parse::<i64>()
                .unwrap_or(0);
            Ok(
                Json(
                    Post::get_by_feed(&store.pool, feed_id, limit, offset)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
    }
    pub mod likes {
        use crate::{entities::Like, store::Store};
        use axum::{
            extract::{Json as ExtractJson, Path, State},
            http::StatusCode, Json,
        };
        pub async fn post_like(
            State(store): State<Store>,
            ExtractJson(like): ExtractJson<Like>,
        ) -> Result<Json<Like>, StatusCode> {
            {
                ::std::io::_print(format_args!("post_like\n"));
            };
            match &like {
                tmp => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}] {2} = {3:#?}\n",
                                "src/api/likes.rs",
                                13u32,
                                "&like",
                                &tmp,
                            ),
                        );
                    };
                    tmp
                }
            };
            Ok(
                Json(
                    like
                        .upsert(&store.pool)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
        pub async fn delete_like(
            State(store): State<Store>,
            Path((post_id, profile_id)): Path<(String, String)>,
        ) -> Result<Json<Like>, StatusCode> {
            let like = Like::new(profile_id, post_id);
            {
                ::std::io::_print(format_args!("post_like\n"));
            };
            match &like {
                tmp => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}] {2} = {3:#?}\n",
                                "src/api/likes.rs",
                                28u32,
                                "&like",
                                &tmp,
                            ),
                        );
                    };
                    tmp
                }
            };
            Ok(
                Json(
                    like
                        .delete(&store.pool)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
    }
    pub mod posts {
        use crate::{
            entities::{Like, Post},
            store::Store,
        };
        use axum::{
            extract::{Path, State},
            http::{HeaderMap, StatusCode},
            Json,
        };
        pub async fn get_post_by_post_id(
            State(store): State<Store>,
            Path(post_id): Path<String>,
        ) -> Result<Json<Post>, StatusCode> {
            Ok(
                Json(
                    Post::get_by_id(&store.pool, post_id)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
        pub async fn post_like_to_post(
            headers: HeaderMap,
            State(store): State<Store>,
            Path(post_id): Path<String>,
        ) -> Result<Json<Like>, StatusCode> {
            match &headers {
                tmp => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}] {2} = {3:#?}\n",
                                "src/api/posts.rs",
                                27u32,
                                "&headers",
                                &tmp,
                            ),
                        );
                    };
                    tmp
                }
            };
            let token = headers
                .get("Authorization")
                .ok_or(StatusCode::BAD_REQUEST)?
                .to_str()
                .map_err(|_| StatusCode::BAD_REQUEST)?
                .to_string();
            match &token {
                tmp => {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "[{0}:{1}] {2} = {3:#?}\n",
                                "src/api/posts.rs",
                                36u32,
                                "&token",
                                &tmp,
                            ),
                        );
                    };
                    tmp
                }
            };
            let profile_id = String::from("e2f0fa7e-4517-4ac8-bbc6-73067d3feed4");
            {
                ::std::io::_print(format_args!("profile_id: {0}\n", profile_id));
            };
            Ok(
                Json(
                    Like::new(profile_id, post_id)
                        .upsert(&store.pool)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
    }
    pub mod profiles {
        use crate::{
            entities::{Blog, Like, Post, Profile},
            store::Store,
        };
        use axum::{
            extract::{Path, State},
            http::StatusCode, Json,
        };
        pub async fn get_profile_by_username(
            State(store): State<Store>,
            Path(username): Path<String>,
        ) -> Result<Json<Profile>, StatusCode> {
            Ok(
                Json(
                    Profile::get_by_username(&store.pool, username)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
        /// Get all blogs for a user by their username.
        /// `/profiles/:username/blogs`
        pub async fn get_blog_by_username(
            State(store): State<Store>,
            Path(username): Path<String>,
        ) -> Result<Json<Vec<Blog>>, StatusCode> {
            Ok(
                Json(
                    Blog::get_by_username(&store.pool, username)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
        pub async fn get_posts_by_username(
            State(store): State<Store>,
            Path(username): Path<String>,
        ) -> Result<Json<Vec<Post>>, StatusCode> {
            Ok(
                Json(
                    Post::get_by_username(&store.pool, username)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
        pub async fn get_liked_posts_by_username(
            State(store): State<Store>,
            Path(username): Path<String>,
        ) -> Result<Json<Vec<Post>>, StatusCode> {
            Ok(
                Json(
                    Post::get_liked_by_username(&store.pool, username)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
        pub async fn get_likes_ids_by_username(
            State(store): State<Store>,
            Path(username): Path<String>,
        ) -> Result<Json<Vec<String>>, StatusCode> {
            Ok(
                Json(
                    Like::get_post_ids_by_username(&store.pool, username)
                        .await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                ),
            )
        }
    }
    pub mod ready {
        pub async fn ready() -> axum::http::StatusCode {
            axum::http::StatusCode::OK
        }
    }
    pub mod users {
        use crate::{entities::User, store::Store};
        use axum::{
            extract::{Path, State},
            http::StatusCode, Json,
        };
        pub async fn get_user_by_username(
            State(store): State<Store>,
            Path(username): Path<String>,
        ) -> Result<Json<User>, StatusCode> {
            Ok(
                Json(
                    User::get_by_username(&store.pool, username)
                        .await
                        .map_err(|_| StatusCode::NOT_FOUND)?,
                ),
            )
        }
    }
    pub mod error {
        use crate::entities::error::EntityError;
        use http::StatusCode;
        use serde::Serialize;
        use serde_with::{serde_as, DisplayFromStr};
        use std::fmt::Debug;
        pub enum Error {
            StatusCode(
                #[serde_as(r#as = "DisplayFromStr")]
                #[serde(with = ":: serde_with :: As :: < DisplayFromStr >")]
                StatusCode,
            ),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::StatusCode(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "StatusCode",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Error {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Error::StatusCode(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Error",
                                0u32,
                                "StatusCode",
                                {
                                    #[doc(hidden)]
                                    struct __SerializeWith<'__a> {
                                        values: (&'__a StatusCode,),
                                        phantom: _serde::__private::PhantomData<Error>,
                                    }
                                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                        fn serialize<__S>(
                                            &self,
                                            __s: __S,
                                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                        where
                                            __S: _serde::Serializer,
                                        {
                                            ::serde_with::As::<
                                                DisplayFromStr,
                                            >::serialize(self.values.0, __s)
                                        }
                                    }
                                    &__SerializeWith {
                                        values: (__field0,),
                                        phantom: _serde::__private::PhantomData::<Error>,
                                    }
                                },
                            )
                        }
                    }
                }
            }
        };
        impl From<StatusCode> for Error {
            fn from(val: StatusCode) -> Self {
                Self::StatusCode(val)
            }
        }
        impl From<EntityError> for Error {
            fn from(val: EntityError) -> Self {
                match val {
                    EntityError::NotFound => Self::StatusCode(StatusCode::NOT_FOUND),
                    EntityError::Malformed { .. } => {
                        Self::StatusCode(StatusCode::BAD_REQUEST)
                    }
                    EntityError::Sqlx(_) => {
                        Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
        }
    }
}
mod auth {
    mod client {
        use oauth2::reqwest::async_http_client;
        use oauth2::TokenResponse;
        use oauth2::{
            basic::BasicClient, AccessToken, AuthUrl, AuthorizationCode, ClientId,
            ClientSecret, CsrfToken, Scope, TokenUrl,
        };
        use reqwest;
        const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
        const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
        pub struct Client {
            oauth_client: BasicClient,
            post_auth_redirect_uri: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Client {
            #[inline]
            fn clone(&self) -> Client {
                Client {
                    oauth_client: ::core::clone::Clone::clone(&self.oauth_client),
                    post_auth_redirect_uri: ::core::clone::Clone::clone(
                        &self.post_auth_redirect_uri,
                    ),
                }
            }
        }
        impl Client {
            pub fn new(
                client_id: String,
                client_secret: String,
                post_auth_redirect_uri: String,
            ) -> Self {
                Client {
                    oauth_client: BasicClient::new(
                        ClientId::new(client_id),
                        Some(ClientSecret::new(client_secret)),
                        AuthUrl::new(AUTH_URL.to_string()).expect("Invalid auth URL"),
                        Some(
                            TokenUrl::new(TOKEN_URL.to_string())
                                .expect("Invalid token URL"),
                        ),
                    ),
                    post_auth_redirect_uri,
                }
            }
            /// Returns the URL to redirect the user to for authorization on GitHub.
            pub fn login_url(&self) -> String {
                let (authorize_url, _) = self
                    .oauth_client
                    .authorize_url(CsrfToken::new_random)
                    .add_scope(Scope::new("repo".to_string()))
                    .add_scope(Scope::new("user".to_string()))
                    .url();
                authorize_url.to_string()
            }
            pub async fn exchange_code(
                &self,
                code: String,
            ) -> Result<AccessToken, anyhow::Error> {
                let token = self
                    .oauth_client
                    .exchange_code(AuthorizationCode::new(code))
                    .request_async(async_http_client)
                    .await
                    .map_err(|err| ::anyhow::Error::msg({
                        let res = ::alloc::fmt::format(
                            format_args!("Failed to exchange code: {0}", err),
                        );
                        res
                    }))?;
                Ok(token.access_token().clone())
            }
            pub fn post_auth_redirect_uri(&self) -> String {
                self.post_auth_redirect_uri.to_string()
            }
            pub async fn fetch_github_user(
                &self,
                token: AccessToken,
            ) -> Result<super::GitHubUser, reqwest::Error> {
                let req_client = reqwest::Client::new();
                let response = req_client
                    .get("https://api.github.com/user")
                    .header("User-Agent", "rust-rocket-oauth2")
                    .header("Accept", "application/json")
                    .bearer_auth(token.secret())
                    .send()
                    .await?;
                let user: super::GitHubUser = response.json().await?;
                Ok(user)
            }
        }
    }
    mod github_user {
        use serde::{Deserialize, Serialize};
        pub struct GitHubUser {
            pub login: String,
            pub id: i64,
            pub node_id: String,
            pub avatar_url: String,
            pub gravatar_id: String,
            pub url: String,
            pub html_url: String,
            pub followers_url: String,
            pub following_url: String,
            pub gists_url: String,
            pub starred_url: String,
            pub subscriptions_url: String,
            pub organizations_url: String,
            pub repos_url: String,
            pub events_url: String,
            pub received_events_url: String,
            #[serde(rename = "type")]
            pub user_type: String,
            pub site_admin: bool,
            pub name: String,
            pub company: String,
            pub blog: String,
            pub location: String,
            pub email: String,
            pub hireable: Option<bool>,
            pub bio: String,
            pub twitter_username: String,
            pub public_repos: i64,
            pub public_gists: i64,
            pub followers: i64,
            pub following: i64,
            pub created_at: String,
            pub updated_at: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for GitHubUser {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "login",
                    "id",
                    "node_id",
                    "avatar_url",
                    "gravatar_id",
                    "url",
                    "html_url",
                    "followers_url",
                    "following_url",
                    "gists_url",
                    "starred_url",
                    "subscriptions_url",
                    "organizations_url",
                    "repos_url",
                    "events_url",
                    "received_events_url",
                    "user_type",
                    "site_admin",
                    "name",
                    "company",
                    "blog",
                    "location",
                    "email",
                    "hireable",
                    "bio",
                    "twitter_username",
                    "public_repos",
                    "public_gists",
                    "followers",
                    "following",
                    "created_at",
                    "updated_at",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.login,
                    &self.id,
                    &self.node_id,
                    &self.avatar_url,
                    &self.gravatar_id,
                    &self.url,
                    &self.html_url,
                    &self.followers_url,
                    &self.following_url,
                    &self.gists_url,
                    &self.starred_url,
                    &self.subscriptions_url,
                    &self.organizations_url,
                    &self.repos_url,
                    &self.events_url,
                    &self.received_events_url,
                    &self.user_type,
                    &self.site_admin,
                    &self.name,
                    &self.company,
                    &self.blog,
                    &self.location,
                    &self.email,
                    &self.hireable,
                    &self.bio,
                    &self.twitter_username,
                    &self.public_repos,
                    &self.public_gists,
                    &self.followers,
                    &self.following,
                    &self.created_at,
                    &&self.updated_at,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "GitHubUser",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GitHubUser {
            #[inline]
            fn clone(&self) -> GitHubUser {
                GitHubUser {
                    login: ::core::clone::Clone::clone(&self.login),
                    id: ::core::clone::Clone::clone(&self.id),
                    node_id: ::core::clone::Clone::clone(&self.node_id),
                    avatar_url: ::core::clone::Clone::clone(&self.avatar_url),
                    gravatar_id: ::core::clone::Clone::clone(&self.gravatar_id),
                    url: ::core::clone::Clone::clone(&self.url),
                    html_url: ::core::clone::Clone::clone(&self.html_url),
                    followers_url: ::core::clone::Clone::clone(&self.followers_url),
                    following_url: ::core::clone::Clone::clone(&self.following_url),
                    gists_url: ::core::clone::Clone::clone(&self.gists_url),
                    starred_url: ::core::clone::Clone::clone(&self.starred_url),
                    subscriptions_url: ::core::clone::Clone::clone(
                        &self.subscriptions_url,
                    ),
                    organizations_url: ::core::clone::Clone::clone(
                        &self.organizations_url,
                    ),
                    repos_url: ::core::clone::Clone::clone(&self.repos_url),
                    events_url: ::core::clone::Clone::clone(&self.events_url),
                    received_events_url: ::core::clone::Clone::clone(
                        &self.received_events_url,
                    ),
                    user_type: ::core::clone::Clone::clone(&self.user_type),
                    site_admin: ::core::clone::Clone::clone(&self.site_admin),
                    name: ::core::clone::Clone::clone(&self.name),
                    company: ::core::clone::Clone::clone(&self.company),
                    blog: ::core::clone::Clone::clone(&self.blog),
                    location: ::core::clone::Clone::clone(&self.location),
                    email: ::core::clone::Clone::clone(&self.email),
                    hireable: ::core::clone::Clone::clone(&self.hireable),
                    bio: ::core::clone::Clone::clone(&self.bio),
                    twitter_username: ::core::clone::Clone::clone(
                        &self.twitter_username,
                    ),
                    public_repos: ::core::clone::Clone::clone(&self.public_repos),
                    public_gists: ::core::clone::Clone::clone(&self.public_gists),
                    followers: ::core::clone::Clone::clone(&self.followers),
                    following: ::core::clone::Clone::clone(&self.following),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    updated_at: ::core::clone::Clone::clone(&self.updated_at),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for GitHubUser {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __field18,
                        __field19,
                        __field20,
                        __field21,
                        __field22,
                        __field23,
                        __field24,
                        __field25,
                        __field26,
                        __field27,
                        __field28,
                        __field29,
                        __field30,
                        __field31,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                18u64 => _serde::__private::Ok(__Field::__field18),
                                19u64 => _serde::__private::Ok(__Field::__field19),
                                20u64 => _serde::__private::Ok(__Field::__field20),
                                21u64 => _serde::__private::Ok(__Field::__field21),
                                22u64 => _serde::__private::Ok(__Field::__field22),
                                23u64 => _serde::__private::Ok(__Field::__field23),
                                24u64 => _serde::__private::Ok(__Field::__field24),
                                25u64 => _serde::__private::Ok(__Field::__field25),
                                26u64 => _serde::__private::Ok(__Field::__field26),
                                27u64 => _serde::__private::Ok(__Field::__field27),
                                28u64 => _serde::__private::Ok(__Field::__field28),
                                29u64 => _serde::__private::Ok(__Field::__field29),
                                30u64 => _serde::__private::Ok(__Field::__field30),
                                31u64 => _serde::__private::Ok(__Field::__field31),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "login" => _serde::__private::Ok(__Field::__field0),
                                "id" => _serde::__private::Ok(__Field::__field1),
                                "node_id" => _serde::__private::Ok(__Field::__field2),
                                "avatar_url" => _serde::__private::Ok(__Field::__field3),
                                "gravatar_id" => _serde::__private::Ok(__Field::__field4),
                                "url" => _serde::__private::Ok(__Field::__field5),
                                "html_url" => _serde::__private::Ok(__Field::__field6),
                                "followers_url" => _serde::__private::Ok(__Field::__field7),
                                "following_url" => _serde::__private::Ok(__Field::__field8),
                                "gists_url" => _serde::__private::Ok(__Field::__field9),
                                "starred_url" => _serde::__private::Ok(__Field::__field10),
                                "subscriptions_url" => {
                                    _serde::__private::Ok(__Field::__field11)
                                }
                                "organizations_url" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                "repos_url" => _serde::__private::Ok(__Field::__field13),
                                "events_url" => _serde::__private::Ok(__Field::__field14),
                                "received_events_url" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                "type" => _serde::__private::Ok(__Field::__field16),
                                "site_admin" => _serde::__private::Ok(__Field::__field17),
                                "name" => _serde::__private::Ok(__Field::__field18),
                                "company" => _serde::__private::Ok(__Field::__field19),
                                "blog" => _serde::__private::Ok(__Field::__field20),
                                "location" => _serde::__private::Ok(__Field::__field21),
                                "email" => _serde::__private::Ok(__Field::__field22),
                                "hireable" => _serde::__private::Ok(__Field::__field23),
                                "bio" => _serde::__private::Ok(__Field::__field24),
                                "twitter_username" => {
                                    _serde::__private::Ok(__Field::__field25)
                                }
                                "public_repos" => _serde::__private::Ok(__Field::__field26),
                                "public_gists" => _serde::__private::Ok(__Field::__field27),
                                "followers" => _serde::__private::Ok(__Field::__field28),
                                "following" => _serde::__private::Ok(__Field::__field29),
                                "created_at" => _serde::__private::Ok(__Field::__field30),
                                "updated_at" => _serde::__private::Ok(__Field::__field31),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"login" => _serde::__private::Ok(__Field::__field0),
                                b"id" => _serde::__private::Ok(__Field::__field1),
                                b"node_id" => _serde::__private::Ok(__Field::__field2),
                                b"avatar_url" => _serde::__private::Ok(__Field::__field3),
                                b"gravatar_id" => _serde::__private::Ok(__Field::__field4),
                                b"url" => _serde::__private::Ok(__Field::__field5),
                                b"html_url" => _serde::__private::Ok(__Field::__field6),
                                b"followers_url" => _serde::__private::Ok(__Field::__field7),
                                b"following_url" => _serde::__private::Ok(__Field::__field8),
                                b"gists_url" => _serde::__private::Ok(__Field::__field9),
                                b"starred_url" => _serde::__private::Ok(__Field::__field10),
                                b"subscriptions_url" => {
                                    _serde::__private::Ok(__Field::__field11)
                                }
                                b"organizations_url" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                b"repos_url" => _serde::__private::Ok(__Field::__field13),
                                b"events_url" => _serde::__private::Ok(__Field::__field14),
                                b"received_events_url" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                b"type" => _serde::__private::Ok(__Field::__field16),
                                b"site_admin" => _serde::__private::Ok(__Field::__field17),
                                b"name" => _serde::__private::Ok(__Field::__field18),
                                b"company" => _serde::__private::Ok(__Field::__field19),
                                b"blog" => _serde::__private::Ok(__Field::__field20),
                                b"location" => _serde::__private::Ok(__Field::__field21),
                                b"email" => _serde::__private::Ok(__Field::__field22),
                                b"hireable" => _serde::__private::Ok(__Field::__field23),
                                b"bio" => _serde::__private::Ok(__Field::__field24),
                                b"twitter_username" => {
                                    _serde::__private::Ok(__Field::__field25)
                                }
                                b"public_repos" => _serde::__private::Ok(__Field::__field26),
                                b"public_gists" => _serde::__private::Ok(__Field::__field27),
                                b"followers" => _serde::__private::Ok(__Field::__field28),
                                b"following" => _serde::__private::Ok(__Field::__field29),
                                b"created_at" => _serde::__private::Ok(__Field::__field30),
                                b"updated_at" => _serde::__private::Ok(__Field::__field31),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<GitHubUser>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = GitHubUser;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct GitHubUser",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field13 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field14 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field15 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            15usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field16 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            16usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field17 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            17usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field18 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            18usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field19 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            19usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field20 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            20usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field21 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            21usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field22 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            22usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field23 = match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            23usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field24 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            24usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field25 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            25usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field26 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            26usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field27 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            27usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field28 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            28usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field29 = match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            29usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field30 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            30usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            let __field31 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            31usize,
                                            &"struct GitHubUser with 32 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(GitHubUser {
                                login: __field0,
                                id: __field1,
                                node_id: __field2,
                                avatar_url: __field3,
                                gravatar_id: __field4,
                                url: __field5,
                                html_url: __field6,
                                followers_url: __field7,
                                following_url: __field8,
                                gists_url: __field9,
                                starred_url: __field10,
                                subscriptions_url: __field11,
                                organizations_url: __field12,
                                repos_url: __field13,
                                events_url: __field14,
                                received_events_url: __field15,
                                user_type: __field16,
                                site_admin: __field17,
                                name: __field18,
                                company: __field19,
                                blog: __field20,
                                location: __field21,
                                email: __field22,
                                hireable: __field23,
                                bio: __field24,
                                twitter_username: __field25,
                                public_repos: __field26,
                                public_gists: __field27,
                                followers: __field28,
                                following: __field29,
                                created_at: __field30,
                                updated_at: __field31,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field13: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field14: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field15: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field16: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field17: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field18: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field19: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field20: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field21: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field22: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field23: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field24: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field25: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field26: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field27: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field28: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field29: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field30: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field31: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("login"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "node_id",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "avatar_url",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "gravatar_id",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "html_url",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "followers_url",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "following_url",
                                                ),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "gists_url",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "starred_url",
                                                ),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "subscriptions_url",
                                                ),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "organizations_url",
                                                ),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field13 => {
                                        if _serde::__private::Option::is_some(&__field13) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "repos_url",
                                                ),
                                            );
                                        }
                                        __field13 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field14 => {
                                        if _serde::__private::Option::is_some(&__field14) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "events_url",
                                                ),
                                            );
                                        }
                                        __field14 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field15 => {
                                        if _serde::__private::Option::is_some(&__field15) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "received_events_url",
                                                ),
                                            );
                                        }
                                        __field15 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field16 => {
                                        if _serde::__private::Option::is_some(&__field16) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            );
                                        }
                                        __field16 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field17 => {
                                        if _serde::__private::Option::is_some(&__field17) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "site_admin",
                                                ),
                                            );
                                        }
                                        __field17 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field18 => {
                                        if _serde::__private::Option::is_some(&__field18) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field18 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field19 => {
                                        if _serde::__private::Option::is_some(&__field19) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "company",
                                                ),
                                            );
                                        }
                                        __field19 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field20 => {
                                        if _serde::__private::Option::is_some(&__field20) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("blog"),
                                            );
                                        }
                                        __field20 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field21 => {
                                        if _serde::__private::Option::is_some(&__field21) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field21 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field22 => {
                                        if _serde::__private::Option::is_some(&__field22) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("email"),
                                            );
                                        }
                                        __field22 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field23 => {
                                        if _serde::__private::Option::is_some(&__field23) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "hireable",
                                                ),
                                            );
                                        }
                                        __field23 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field24 => {
                                        if _serde::__private::Option::is_some(&__field24) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("bio"),
                                            );
                                        }
                                        __field24 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field25 => {
                                        if _serde::__private::Option::is_some(&__field25) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "twitter_username",
                                                ),
                                            );
                                        }
                                        __field25 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field26 => {
                                        if _serde::__private::Option::is_some(&__field26) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "public_repos",
                                                ),
                                            );
                                        }
                                        __field26 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field27 => {
                                        if _serde::__private::Option::is_some(&__field27) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "public_gists",
                                                ),
                                            );
                                        }
                                        __field27 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field28 => {
                                        if _serde::__private::Option::is_some(&__field28) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "followers",
                                                ),
                                            );
                                        }
                                        __field28 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field29 => {
                                        if _serde::__private::Option::is_some(&__field29) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "following",
                                                ),
                                            );
                                        }
                                        __field29 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i64>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field30 => {
                                        if _serde::__private::Option::is_some(&__field30) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "created_at",
                                                ),
                                            );
                                        }
                                        __field30 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field31 => {
                                        if _serde::__private::Option::is_some(&__field31) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updated_at",
                                                ),
                                            );
                                        }
                                        __field31 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("login")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("node_id")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("avatar_url")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("gravatar_id")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("html_url")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("followers_url")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("following_url")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("gists_url")?
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("starred_url")?
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("subscriptions_url")?
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("organizations_url")?
                                }
                            };
                            let __field13 = match __field13 {
                                _serde::__private::Some(__field13) => __field13,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("repos_url")?
                                }
                            };
                            let __field14 = match __field14 {
                                _serde::__private::Some(__field14) => __field14,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("events_url")?
                                }
                            };
                            let __field15 = match __field15 {
                                _serde::__private::Some(__field15) => __field15,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("received_events_url")?
                                }
                            };
                            let __field16 = match __field16 {
                                _serde::__private::Some(__field16) => __field16,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("type")?
                                }
                            };
                            let __field17 = match __field17 {
                                _serde::__private::Some(__field17) => __field17,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("site_admin")?
                                }
                            };
                            let __field18 = match __field18 {
                                _serde::__private::Some(__field18) => __field18,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            let __field19 = match __field19 {
                                _serde::__private::Some(__field19) => __field19,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("company")?
                                }
                            };
                            let __field20 = match __field20 {
                                _serde::__private::Some(__field20) => __field20,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("blog")?
                                }
                            };
                            let __field21 = match __field21 {
                                _serde::__private::Some(__field21) => __field21,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field22 = match __field22 {
                                _serde::__private::Some(__field22) => __field22,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("email")?
                                }
                            };
                            let __field23 = match __field23 {
                                _serde::__private::Some(__field23) => __field23,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("hireable")?
                                }
                            };
                            let __field24 = match __field24 {
                                _serde::__private::Some(__field24) => __field24,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("bio")?
                                }
                            };
                            let __field25 = match __field25 {
                                _serde::__private::Some(__field25) => __field25,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("twitter_username")?
                                }
                            };
                            let __field26 = match __field26 {
                                _serde::__private::Some(__field26) => __field26,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("public_repos")?
                                }
                            };
                            let __field27 = match __field27 {
                                _serde::__private::Some(__field27) => __field27,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("public_gists")?
                                }
                            };
                            let __field28 = match __field28 {
                                _serde::__private::Some(__field28) => __field28,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("followers")?
                                }
                            };
                            let __field29 = match __field29 {
                                _serde::__private::Some(__field29) => __field29,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("following")?
                                }
                            };
                            let __field30 = match __field30 {
                                _serde::__private::Some(__field30) => __field30,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("created_at")?
                                }
                            };
                            let __field31 = match __field31 {
                                _serde::__private::Some(__field31) => __field31,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("updated_at")?
                                }
                            };
                            _serde::__private::Ok(GitHubUser {
                                login: __field0,
                                id: __field1,
                                node_id: __field2,
                                avatar_url: __field3,
                                gravatar_id: __field4,
                                url: __field5,
                                html_url: __field6,
                                followers_url: __field7,
                                following_url: __field8,
                                gists_url: __field9,
                                starred_url: __field10,
                                subscriptions_url: __field11,
                                organizations_url: __field12,
                                repos_url: __field13,
                                events_url: __field14,
                                received_events_url: __field15,
                                user_type: __field16,
                                site_admin: __field17,
                                name: __field18,
                                company: __field19,
                                blog: __field20,
                                location: __field21,
                                email: __field22,
                                hireable: __field23,
                                bio: __field24,
                                twitter_username: __field25,
                                public_repos: __field26,
                                public_gists: __field27,
                                followers: __field28,
                                following: __field29,
                                created_at: __field30,
                                updated_at: __field31,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "login",
                        "id",
                        "node_id",
                        "avatar_url",
                        "gravatar_id",
                        "url",
                        "html_url",
                        "followers_url",
                        "following_url",
                        "gists_url",
                        "starred_url",
                        "subscriptions_url",
                        "organizations_url",
                        "repos_url",
                        "events_url",
                        "received_events_url",
                        "type",
                        "site_admin",
                        "name",
                        "company",
                        "blog",
                        "location",
                        "email",
                        "hireable",
                        "bio",
                        "twitter_username",
                        "public_repos",
                        "public_gists",
                        "followers",
                        "following",
                        "created_at",
                        "updated_at",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "GitHubUser",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<GitHubUser>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for GitHubUser {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "GitHubUser",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                            + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                            + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "login",
                        &self.login,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "node_id",
                        &self.node_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "avatar_url",
                        &self.avatar_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gravatar_id",
                        &self.gravatar_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "html_url",
                        &self.html_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "followers_url",
                        &self.followers_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "following_url",
                        &self.following_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "gists_url",
                        &self.gists_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "starred_url",
                        &self.starred_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "subscriptions_url",
                        &self.subscriptions_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "organizations_url",
                        &self.organizations_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "repos_url",
                        &self.repos_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "events_url",
                        &self.events_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "received_events_url",
                        &self.received_events_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "type",
                        &self.user_type,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "site_admin",
                        &self.site_admin,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "company",
                        &self.company,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "blog",
                        &self.blog,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "email",
                        &self.email,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "hireable",
                        &self.hireable,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "bio",
                        &self.bio,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "twitter_username",
                        &self.twitter_username,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "public_repos",
                        &self.public_repos,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "public_gists",
                        &self.public_gists,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "followers",
                        &self.followers,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "following",
                        &self.following,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "created_at",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "updated_at",
                        &self.updated_at,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    }
    mod session {
        use crate::entities::{Profile, User};
        use oauth2::AccessToken;
        use serde::{Deserialize, Serialize};
        pub struct Session {
            user: User,
            profile: Profile,
            access_token: AccessToken,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Session {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Session",
                    "user",
                    &self.user,
                    "profile",
                    &self.profile,
                    "access_token",
                    &&self.access_token,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Session {
            #[inline]
            fn clone(&self) -> Session {
                Session {
                    user: ::core::clone::Clone::clone(&self.user),
                    profile: ::core::clone::Clone::clone(&self.profile),
                    access_token: ::core::clone::Clone::clone(&self.access_token),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Session {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "user" => _serde::__private::Ok(__Field::__field0),
                                "profile" => _serde::__private::Ok(__Field::__field1),
                                "access_token" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"user" => _serde::__private::Ok(__Field::__field0),
                                b"profile" => _serde::__private::Ok(__Field::__field1),
                                b"access_token" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Session>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Session;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Session",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                User,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Session with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Profile,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Session with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                AccessToken,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Session with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Session {
                                user: __field0,
                                profile: __field1,
                                access_token: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<User> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Profile> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<AccessToken> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("user"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<User>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "profile",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Profile>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "access_token",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                AccessToken,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("user")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("profile")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("access_token")?
                                }
                            };
                            _serde::__private::Ok(Session {
                                user: __field0,
                                profile: __field1,
                                access_token: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "user",
                        "profile",
                        "access_token",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Session",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Session>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Session {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Session",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "user",
                        &self.user,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "profile",
                        &self.profile,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "access_token",
                        &self.access_token,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        impl Session {
            pub fn new(user: User, profile: Profile, access_token: AccessToken) -> Self {
                Self {
                    user,
                    profile,
                    access_token,
                }
            }
        }
    }
    pub use client::Client;
    pub use github_user::GitHubUser;
    pub use session::Session;
}
mod entities {
    pub mod error {
        use serde::Serialize;
        use serde_with::{serde_as, DisplayFromStr};
        use std::fmt::Debug;
        pub enum EntityError {
            NotFound,
            Malformed { error: String },
            Sqlx(
                #[serde_as(r#as = "DisplayFromStr")]
                #[serde(with = ":: serde_with :: As :: < DisplayFromStr >")]
                sqlx::Error,
            ),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for EntityError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    EntityError::NotFound => {
                        ::core::fmt::Formatter::write_str(f, "NotFound")
                    }
                    EntityError::Malformed { error: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Malformed",
                            "error",
                            &__self_0,
                        )
                    }
                    EntityError::Sqlx(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Sqlx",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for EntityError {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        EntityError::NotFound => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "EntityError",
                                0u32,
                                "NotFound",
                            )
                        }
                        EntityError::Malformed { ref error } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                                __serializer,
                                "EntityError",
                                1u32,
                                "Malformed",
                                0 + 1,
                            )?;
                            _serde::ser::SerializeStructVariant::serialize_field(
                                &mut __serde_state,
                                "error",
                                error,
                            )?;
                            _serde::ser::SerializeStructVariant::end(__serde_state)
                        }
                        EntityError::Sqlx(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "EntityError",
                                2u32,
                                "Sqlx",
                                {
                                    #[doc(hidden)]
                                    struct __SerializeWith<'__a> {
                                        values: (&'__a sqlx::Error,),
                                        phantom: _serde::__private::PhantomData<EntityError>,
                                    }
                                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                        fn serialize<__S>(
                                            &self,
                                            __s: __S,
                                        ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                        where
                                            __S: _serde::Serializer,
                                        {
                                            ::serde_with::As::<
                                                DisplayFromStr,
                                            >::serialize(self.values.0, __s)
                                        }
                                    }
                                    &__SerializeWith {
                                        values: (__field0,),
                                        phantom: _serde::__private::PhantomData::<EntityError>,
                                    }
                                },
                            )
                        }
                    }
                }
            }
        };
        impl EntityError {
            pub fn malformed(error: &str) -> Self {
                Self::Malformed {
                    error: {
                        let res = ::alloc::fmt::format(format_args!("{0}", error));
                        res
                    },
                }
            }
        }
        impl From<sqlx::Error> for EntityError {
            fn from(val: sqlx::Error) -> Self {
                match val {
                    sqlx::Error::RowNotFound => Self::NotFound,
                    _ => Self::Sqlx(val),
                }
            }
        }
        impl core::fmt::Display for EntityError {
            fn fmt(
                &self,
                fmt: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                fmt.write_fmt(format_args!("{0:?}", self))
            }
        }
        impl std::error::Error for EntityError {}
    }
    mod blog {
        use serde::{Deserialize, Serialize};
        use sqlx::PgPool;
        use super::error::EntityError;
        #[serde(rename_all = "camelCase")]
        pub struct Blog {
            pub name: String,
            pub slug: String,
            pub username: String,
            pub display_name: Option<String>,
            pub description: Option<String>,
            pub created_at: Option<String>,
            pub updated_at: Option<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Blog {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "name",
                    "slug",
                    "username",
                    "display_name",
                    "description",
                    "created_at",
                    "updated_at",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.name,
                    &self.slug,
                    &self.username,
                    &self.display_name,
                    &self.description,
                    &self.created_at,
                    &&self.updated_at,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Blog",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Blog {
            #[inline]
            fn clone(&self) -> Blog {
                Blog {
                    name: ::core::clone::Clone::clone(&self.name),
                    slug: ::core::clone::Clone::clone(&self.slug),
                    username: ::core::clone::Clone::clone(&self.username),
                    display_name: ::core::clone::Clone::clone(&self.display_name),
                    description: ::core::clone::Clone::clone(&self.description),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    updated_at: ::core::clone::Clone::clone(&self.updated_at),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Blog {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Blog",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "slug",
                        &self.slug,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "username",
                        &self.username,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "displayName",
                        &self.display_name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "createdAt",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "updatedAt",
                        &self.updated_at,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Blog {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "name" => _serde::__private::Ok(__Field::__field0),
                                "slug" => _serde::__private::Ok(__Field::__field1),
                                "username" => _serde::__private::Ok(__Field::__field2),
                                "displayName" => _serde::__private::Ok(__Field::__field3),
                                "description" => _serde::__private::Ok(__Field::__field4),
                                "createdAt" => _serde::__private::Ok(__Field::__field5),
                                "updatedAt" => _serde::__private::Ok(__Field::__field6),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"name" => _serde::__private::Ok(__Field::__field0),
                                b"slug" => _serde::__private::Ok(__Field::__field1),
                                b"username" => _serde::__private::Ok(__Field::__field2),
                                b"displayName" => _serde::__private::Ok(__Field::__field3),
                                b"description" => _serde::__private::Ok(__Field::__field4),
                                b"createdAt" => _serde::__private::Ok(__Field::__field5),
                                b"updatedAt" => _serde::__private::Ok(__Field::__field6),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Blog>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Blog;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Blog",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Blog with 7 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Blog with 7 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Blog with 7 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Blog with 7 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Blog with 7 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Blog with 7 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Blog with 7 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Blog {
                                name: __field0,
                                slug: __field1,
                                username: __field2,
                                display_name: __field3,
                                description: __field4,
                                created_at: __field5,
                                updated_at: __field6,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("slug"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "username",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "displayName",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "createdAt",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updatedAt",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("slug")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("username")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("displayName")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("description")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("createdAt")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("updatedAt")?
                                }
                            };
                            _serde::__private::Ok(Blog {
                                name: __field0,
                                slug: __field1,
                                username: __field2,
                                display_name: __field3,
                                description: __field4,
                                created_at: __field5,
                                updated_at: __field6,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "name",
                        "slug",
                        "username",
                        "displayName",
                        "description",
                        "createdAt",
                        "updatedAt",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Blog",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Blog>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Blog {
            pub async fn upsert(self, pool: &PgPool) -> Result<Self, EntityError> {
                let _ = {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(self.name);
                            let arg1 = &(self.slug);
                            let arg2 = &(self.username);
                            let arg3 = &(self.description);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg1);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg2);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg3);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    4usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg1)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg2)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg3),
                                );
                            query_args.add(arg0);
                            query_args.add(arg1);
                            query_args.add(arg2);
                            query_args.add(arg3);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                "INSERT INTO \"blog\" (\"profile_id\", \"name\", \"slug\", \"description\")\nVALUES (\n\t( \n\t\tSELECT id AS profile_id FROM \"profile\" \n\t\tWHERE user_id=(SELECT id from \"user\" WHERE username=$3)\n\t),\n\t$1, $2, $4\n)\nON CONFLICT (\"slug\") DO UPDATE SET\n\t\"name\" = $1,\n\t\"description\" = $4;\n",
                                query_args,
                            )
                        }
                    }
                }
                    .fetch_one(pool)
                    .await;
                Blog::get_by_slug(pool, self.slug).await
            }
            pub async fn get_by_slug(
                pool: &PgPool,
                slug: String,
            ) -> Result<Self, EntityError> {
                Ok(
                    {
                        {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(slug);
                                if false {
                                    use ::sqlx::ty_match::{
                                        WrapSameExt as _, MatchBorrowExt as _,
                                    };
                                    let expr = ::sqlx::ty_match::dupe_value(arg0);
                                    let ty_check = ::sqlx::ty_match::WrapSame::<
                                        &str,
                                        _,
                                    >::new(&expr)
                                        .wrap_same();
                                    let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                        ty_check,
                                        &expr,
                                    );
                                    _ty_check = match_borrow.match_borrow();
                                    ::core::panicking::panic("explicit panic");
                                }
                                let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::postgres::Postgres,
                                            >::size_hint(arg0),
                                    );
                                query_args.add(arg0);
                                ::sqlx::query_with::<
                                    sqlx::postgres::Postgres,
                                    _,
                                >(
                                        "SELECT \n\tname, slug,\n\tto_char(blog.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(blog.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tusername, display_name, description\nFROM \"blog\" LEFT JOIN (\n\tSELECT \n\t\tprofile.id, display_name, username\n\tFROM \"profile\" LEFT JOIN \"user\"\n\tON user_id=\"user\".id\n) AS \"profile\" ON profile_id=\"profile\".id\nWHERE slug=$1;\n",
                                        query_args,
                                    )
                                    .try_map(|row: sqlx::postgres::PgRow| {
                                        use ::sqlx::Row as _;
                                        let sqlx_query_as_name = row
                                            .try_get_unchecked::<String, _>(0usize)?
                                            .into();
                                        let sqlx_query_as_slug = row
                                            .try_get_unchecked::<String, _>(1usize)?
                                            .into();
                                        let sqlx_query_as_created_at = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(2usize)?
                                            .into();
                                        let sqlx_query_as_updated_at = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(3usize)?
                                            .into();
                                        let sqlx_query_as_username = row
                                            .try_get_unchecked::<String, _>(4usize)?
                                            .into();
                                        let sqlx_query_as_display_name = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(5usize)?
                                            .into();
                                        let sqlx_query_as_description = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(6usize)?
                                            .into();
                                        ::std::result::Result::Ok(Self {
                                            name: sqlx_query_as_name,
                                            slug: sqlx_query_as_slug,
                                            created_at: sqlx_query_as_created_at,
                                            updated_at: sqlx_query_as_updated_at,
                                            username: sqlx_query_as_username,
                                            display_name: sqlx_query_as_display_name,
                                            description: sqlx_query_as_description,
                                        })
                                    })
                            }
                        }
                    }
                        .fetch_one(pool)
                        .await?,
                )
            }
            pub async fn get_by_username(
                pool: &PgPool,
                username: String,
            ) -> Result<Vec<Self>, EntityError> {
                Ok(
                    {
                        {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(username);
                                if false {
                                    use ::sqlx::ty_match::{
                                        WrapSameExt as _, MatchBorrowExt as _,
                                    };
                                    let expr = ::sqlx::ty_match::dupe_value(arg0);
                                    let ty_check = ::sqlx::ty_match::WrapSame::<
                                        &str,
                                        _,
                                    >::new(&expr)
                                        .wrap_same();
                                    let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                        ty_check,
                                        &expr,
                                    );
                                    _ty_check = match_borrow.match_borrow();
                                    ::core::panicking::panic("explicit panic");
                                }
                                let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::postgres::Postgres,
                                            >::size_hint(arg0),
                                    );
                                query_args.add(arg0);
                                ::sqlx::query_with::<
                                    sqlx::postgres::Postgres,
                                    _,
                                >(
                                        "SELECT \n\tname, slug,\n\tto_char(blog.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(blog.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tusername, display_name, description\nFROM \"blog\" LEFT JOIN (\n\tSELECT \n\t\tprofile.id, display_name, username\n\tFROM \"profile\" LEFT JOIN \"user\"\n\tON user_id=\"user\".id\n) AS \"profile\" ON profile_id=\"profile\".id\nWHERE username=$1;\n",
                                        query_args,
                                    )
                                    .try_map(|row: sqlx::postgres::PgRow| {
                                        use ::sqlx::Row as _;
                                        let sqlx_query_as_name = row
                                            .try_get_unchecked::<String, _>(0usize)?
                                            .into();
                                        let sqlx_query_as_slug = row
                                            .try_get_unchecked::<String, _>(1usize)?
                                            .into();
                                        let sqlx_query_as_created_at = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(2usize)?
                                            .into();
                                        let sqlx_query_as_updated_at = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(3usize)?
                                            .into();
                                        let sqlx_query_as_username = row
                                            .try_get_unchecked::<String, _>(4usize)?
                                            .into();
                                        let sqlx_query_as_display_name = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(5usize)?
                                            .into();
                                        let sqlx_query_as_description = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(6usize)?
                                            .into();
                                        ::std::result::Result::Ok(Self {
                                            name: sqlx_query_as_name,
                                            slug: sqlx_query_as_slug,
                                            created_at: sqlx_query_as_created_at,
                                            updated_at: sqlx_query_as_updated_at,
                                            username: sqlx_query_as_username,
                                            display_name: sqlx_query_as_display_name,
                                            description: sqlx_query_as_description,
                                        })
                                    })
                            }
                        }
                    }
                        .fetch_all(pool)
                        .await?,
                )
            }
            pub async fn delete_by_slug(
                pool: &PgPool,
                slug: String,
            ) -> Result<(), EntityError> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(slug);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >("DELETE FROM \"blog\"\nWHERE \"slug\" = $1\n", query_args)
                        }
                    }
                }
                    .execute(pool)
                    .await?;
                Ok(())
            }
        }
    }
    mod feed {
        use serde::{Deserialize, Serialize};
        #[serde(rename_all = "camelCase")]
        pub struct Feed {
            pub id: String,
            pub name: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Feed {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Feed",
                    "id",
                    &self.id,
                    "name",
                    &&self.name,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Feed {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Feed",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Feed {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Feed>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Feed;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Feed",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Feed with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Feed with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Feed {
                                id: __field0,
                                name: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            _serde::__private::Ok(Feed {
                                id: __field0,
                                name: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "name"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Feed",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Feed>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    mod like {
        use super::error::EntityError;
        use serde::{Deserialize, Serialize};
        use sqlx::types::Uuid;
        use sqlx::PgPool;
        #[serde(rename_all = "camelCase")]
        pub struct Like {
            pub profile_id: Option<String>,
            pub post_id: Option<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Like {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Like",
                    "profile_id",
                    &self.profile_id,
                    "post_id",
                    &&self.post_id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Like {
            #[inline]
            fn clone(&self) -> Like {
                Like {
                    profile_id: ::core::clone::Clone::clone(&self.profile_id),
                    post_id: ::core::clone::Clone::clone(&self.post_id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Like {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Like",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "profileId",
                        &self.profile_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "postId",
                        &self.post_id,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Like {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "profileId" => _serde::__private::Ok(__Field::__field0),
                                "postId" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"profileId" => _serde::__private::Ok(__Field::__field0),
                                b"postId" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Like>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Like;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Like",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Like with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Like with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Like {
                                profile_id: __field0,
                                post_id: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "profileId",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("postId"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("profileId")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("postId")?
                                }
                            };
                            _serde::__private::Ok(Like {
                                profile_id: __field0,
                                post_id: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["profileId", "postId"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Like",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Like>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Like {
            pub fn new(profile_id: String, post_id: String) -> Self {
                Like {
                    profile_id: Some(profile_id),
                    post_id: Some(post_id),
                }
            }
            pub fn profile_uuid(&self) -> Result<Uuid, EntityError> {
                match &self.profile_id {
                    Some(id) => {
                        Uuid::try_parse(&id)
                            .map_err(|_| EntityError::malformed(
                                "Like.profile_id is not UUID.",
                            ))
                    }
                    None => Err(EntityError::malformed("Like is missing profile_id.")),
                }
            }
            pub fn post_uuid(&self) -> Result<Uuid, EntityError> {
                match &self.post_id {
                    Some(id) => {
                        Uuid::try_parse(&id)
                            .map_err(|_| EntityError::malformed(
                                "Like.post_id is not UUID.",
                            ))
                    }
                    None => Err(EntityError::malformed("Like is missing post_id.")),
                }
            }
            pub async fn upsert(self, pool: &PgPool) -> Result<Self, EntityError> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(self.profile_uuid()?);
                            let arg1 = &(self.post_uuid()?);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    sqlx::types::Uuid,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg1);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    sqlx::types::Uuid,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    2usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg1),
                                );
                            query_args.add(arg0);
                            query_args.add(arg1);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "\n            INSERT INTO \"like\" (profile_id, post_id)\n            VALUES ($1, $2)\n                ON CONFLICT (profile_id, post_id)\n                DO UPDATE SET profile_id = $1, post_id = $2\n            RETURNING profile_id::TEXT, post_id::TEXT;\n            ",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_profile_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_post_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        profile_id: sqlx_query_as_profile_id,
                                        post_id: sqlx_query_as_post_id,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
                    .map_err(|x| x.into())
            }
            pub async fn get_post_ids_by_username(
                pool: &PgPool,
                username: String,
            ) -> Result<Vec<String>, EntityError> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(username);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "\n            SELECT profile_id::TEXT, post_id::TEXT\n            FROM \"like\" LEFT JOIN (\n                SELECT \n            profile.id, username\n            FROM \"profile\" LEFT JOIN \"user\"\n            ON user_id=\"user\".id\n            ) AS \"profile\" ON profile_id=\"profile\".id\n            WHERE username = $1;\n            ",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_profile_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_post_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        profile_id: sqlx_query_as_profile_id,
                                        post_id: sqlx_query_as_post_id,
                                    })
                                })
                        }
                    }
                }
                    .fetch_all(pool)
                    .await
                    .map_err(|x| x.into())
                    .map(|likes| {
                        likes.into_iter().map(|like| like.post_id.unwrap()).collect()
                    })
            }
            pub async fn delete(self, pool: &PgPool) -> Result<Self, EntityError> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(self.profile_uuid()?);
                            let arg1 = &(self.post_uuid()?);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    sqlx::types::Uuid,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg1);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    sqlx::types::Uuid,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    2usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg1),
                                );
                            query_args.add(arg0);
                            query_args.add(arg1);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "\n            DELETE FROM \"like\"\n            WHERE profile_id = $1 AND post_id = $2\n            RETURNING profile_id::TEXT, post_id::TEXT;\n            ",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_profile_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_post_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        profile_id: sqlx_query_as_profile_id,
                                        post_id: sqlx_query_as_post_id,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
                    .map_err(|x| x.into())
            }
        }
    }
    mod post {
        use super::error::EntityError;
        use anyhow::anyhow;
        use serde::{Deserialize, Serialize};
        use sqlx::types::Uuid;
        use sqlx::PgPool;
        #[serde(rename_all = "camelCase")]
        pub struct Post {
            pub id: Option<String>,
            pub slug: String,
            pub blog_slug: String,
            pub blog_name: String,
            pub author_name: Option<String>,
            pub author_slug: String,
            pub created_at: Option<String>,
            pub updated_at: Option<String>,
            pub title: String,
            pub content: String,
            pub likes: Option<i64>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Post {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "slug",
                    "blog_slug",
                    "blog_name",
                    "author_name",
                    "author_slug",
                    "created_at",
                    "updated_at",
                    "title",
                    "content",
                    "likes",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.slug,
                    &self.blog_slug,
                    &self.blog_name,
                    &self.author_name,
                    &self.author_slug,
                    &self.created_at,
                    &self.updated_at,
                    &self.title,
                    &self.content,
                    &&self.likes,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Post",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Post {
            #[inline]
            fn clone(&self) -> Post {
                Post {
                    id: ::core::clone::Clone::clone(&self.id),
                    slug: ::core::clone::Clone::clone(&self.slug),
                    blog_slug: ::core::clone::Clone::clone(&self.blog_slug),
                    blog_name: ::core::clone::Clone::clone(&self.blog_name),
                    author_name: ::core::clone::Clone::clone(&self.author_name),
                    author_slug: ::core::clone::Clone::clone(&self.author_slug),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    updated_at: ::core::clone::Clone::clone(&self.updated_at),
                    title: ::core::clone::Clone::clone(&self.title),
                    content: ::core::clone::Clone::clone(&self.content),
                    likes: ::core::clone::Clone::clone(&self.likes),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Post {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Post",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "slug",
                        &self.slug,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "blogSlug",
                        &self.blog_slug,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "blogName",
                        &self.blog_name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "authorName",
                        &self.author_name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "authorSlug",
                        &self.author_slug,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "createdAt",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "updatedAt",
                        &self.updated_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "title",
                        &self.title,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "content",
                        &self.content,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "likes",
                        &self.likes,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Post {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "slug" => _serde::__private::Ok(__Field::__field1),
                                "blogSlug" => _serde::__private::Ok(__Field::__field2),
                                "blogName" => _serde::__private::Ok(__Field::__field3),
                                "authorName" => _serde::__private::Ok(__Field::__field4),
                                "authorSlug" => _serde::__private::Ok(__Field::__field5),
                                "createdAt" => _serde::__private::Ok(__Field::__field6),
                                "updatedAt" => _serde::__private::Ok(__Field::__field7),
                                "title" => _serde::__private::Ok(__Field::__field8),
                                "content" => _serde::__private::Ok(__Field::__field9),
                                "likes" => _serde::__private::Ok(__Field::__field10),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"slug" => _serde::__private::Ok(__Field::__field1),
                                b"blogSlug" => _serde::__private::Ok(__Field::__field2),
                                b"blogName" => _serde::__private::Ok(__Field::__field3),
                                b"authorName" => _serde::__private::Ok(__Field::__field4),
                                b"authorSlug" => _serde::__private::Ok(__Field::__field5),
                                b"createdAt" => _serde::__private::Ok(__Field::__field6),
                                b"updatedAt" => _serde::__private::Ok(__Field::__field7),
                                b"title" => _serde::__private::Ok(__Field::__field8),
                                b"content" => _serde::__private::Ok(__Field::__field9),
                                b"likes" => _serde::__private::Ok(__Field::__field10),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Post>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Post;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Post",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match _serde::de::SeqAccess::next_element::<
                                Option<i64>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Post with 11 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Post {
                                id: __field0,
                                slug: __field1,
                                blog_slug: __field2,
                                blog_name: __field3,
                                author_name: __field4,
                                author_slug: __field5,
                                created_at: __field6,
                                updated_at: __field7,
                                title: __field8,
                                content: __field9,
                                likes: __field10,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<Option<i64>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("slug"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "blogSlug",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "blogName",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "authorName",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "authorSlug",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "createdAt",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updatedAt",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("title"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "content",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("likes"),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<i64>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("slug")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("blogSlug")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("blogName")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("authorName")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("authorSlug")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("createdAt")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("updatedAt")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("title")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("content")?
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("likes")?
                                }
                            };
                            _serde::__private::Ok(Post {
                                id: __field0,
                                slug: __field1,
                                blog_slug: __field2,
                                blog_name: __field3,
                                author_name: __field4,
                                author_slug: __field5,
                                created_at: __field6,
                                updated_at: __field7,
                                title: __field8,
                                content: __field9,
                                likes: __field10,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "slug",
                        "blogSlug",
                        "blogName",
                        "authorName",
                        "authorSlug",
                        "createdAt",
                        "updatedAt",
                        "title",
                        "content",
                        "likes",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Post",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Post>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Post {
            pub async fn get_by_id(
                pool: &PgPool,
                id: String,
            ) -> Result<Self, EntityError> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(Uuid::try_parse(id.as_str())
                                .map_err(|_| EntityError::malformed("id is not uuid"))?);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    sqlx::types::Uuid,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "SELECT \n\tid::text,\n\tslug,\n\tblog_slug,\n\tblog_name,\n\tauthor_name,\n\tprofile_slug AS author_slug,\n\ttitle,\n\tbody AS content,\n\tto_char(post.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(post.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tCOALESCE(likes.like_count, 0) AS likes\nFROM \"post\" LEFT JOIN (\n\tSELECT\n\t\tblog.id AS blog_id,\n\t\tblog.slug AS blog_slug,\n\t\tblog.name AS blog_name,\n\t\tprofile.id AS profile_id,\n\t\tprofile.display_name AS author_name,\n\t\tprofile_slug\n\tFROM \"blog\" LEFT JOIN (\n\t\tSELECT\n\t\t\tprofile.id,\n\t\t\tprofile.display_name,\n\t\t\t\"user\".username as profile_slug\n\t\tFROM \"profile\" LEFT JOIN \"user\"\n\t\tON user_id = \"user\".id\n\t) \"profile\"\n\tON profile_id=profile.id\n) AS \"blog\" ON post.blog_id = blog.blog_id\nLEFT JOIN (\n\tSELECT post_id, COUNT(*) AS like_count\n\tFROM \"like\"\n\tGROUP BY post_id\n) AS likes ON post.id = likes.post_id\nWHERE id = $1;\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_slug = row
                                        .try_get_unchecked::<String, _>(1usize)?
                                        .into();
                                    let sqlx_query_as_blog_slug = row
                                        .try_get_unchecked::<String, _>(2usize)?
                                        .into();
                                    let sqlx_query_as_blog_name = row
                                        .try_get_unchecked::<String, _>(3usize)?
                                        .into();
                                    let sqlx_query_as_author_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?
                                        .into();
                                    let sqlx_query_as_author_slug = row
                                        .try_get_unchecked::<String, _>(5usize)?
                                        .into();
                                    let sqlx_query_as_title = row
                                        .try_get_unchecked::<String, _>(6usize)?
                                        .into();
                                    let sqlx_query_as_content = row
                                        .try_get_unchecked::<String, _>(7usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(8usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(9usize)?
                                        .into();
                                    let sqlx_query_as_likes = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<i64>,
                                            _,
                                        >(10usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        slug: sqlx_query_as_slug,
                                        blog_slug: sqlx_query_as_blog_slug,
                                        blog_name: sqlx_query_as_blog_name,
                                        author_name: sqlx_query_as_author_name,
                                        author_slug: sqlx_query_as_author_slug,
                                        title: sqlx_query_as_title,
                                        content: sqlx_query_as_content,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        likes: sqlx_query_as_likes,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
                    .map_err(|x| EntityError::Sqlx(x))
            }
            pub async fn get_by_blog_slug(
                pool: &PgPool,
                blog_slug: String,
            ) -> Result<Vec<Self>, EntityError> {
                Ok(
                    {
                        {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(blog_slug);
                                if false {
                                    use ::sqlx::ty_match::{
                                        WrapSameExt as _, MatchBorrowExt as _,
                                    };
                                    let expr = ::sqlx::ty_match::dupe_value(arg0);
                                    let ty_check = ::sqlx::ty_match::WrapSame::<
                                        &str,
                                        _,
                                    >::new(&expr)
                                        .wrap_same();
                                    let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                        ty_check,
                                        &expr,
                                    );
                                    _ty_check = match_borrow.match_borrow();
                                    ::core::panicking::panic("explicit panic");
                                }
                                let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                                query_args
                                    .reserve(
                                        1usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::postgres::Postgres,
                                            >::size_hint(arg0),
                                    );
                                query_args.add(arg0);
                                ::sqlx::query_with::<
                                    sqlx::postgres::Postgres,
                                    _,
                                >(
                                        "SELECT \n\tid::text,\n\tslug,\n\tblog_slug,\n\tblog_name,\n\tauthor_name,\n\tprofile_slug AS author_slug,\n\ttitle,\n\tbody AS content,\n\tto_char(post.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(post.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tCOALESCE(likes.like_count, 0) AS likes\nFROM \"post\" LEFT JOIN (\n\tSELECT\n\t\tblog.id AS blog_id,\n\t\tblog.slug AS blog_slug,\n\t\tblog.name AS blog_name,\n\t\tprofile.id AS profile_id,\n\t\tprofile.display_name AS author_name,\n\t\tprofile_slug\n\tFROM \"blog\" LEFT JOIN (\n\t\tSELECT\n\t\t\tprofile.id,\n\t\t\tprofile.display_name,\n\t\t\t\"user\".username as profile_slug\n\t\tFROM \"profile\" LEFT JOIN \"user\"\n\t\tON user_id = \"user\".id\n\t) \"profile\"\n\tON profile_id=profile.id\n) AS \"blog\" ON post.blog_id = blog.blog_id\nLEFT JOIN (\n\tSELECT post_id, COUNT(*) AS like_count\n\tFROM \"like\"\n\tGROUP BY post_id\n) AS likes ON post.id = likes.post_id\nWHERE blog_slug = $1;\n",
                                        query_args,
                                    )
                                    .try_map(|row: sqlx::postgres::PgRow| {
                                        use ::sqlx::Row as _;
                                        let sqlx_query_as_id = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(0usize)?
                                            .into();
                                        let sqlx_query_as_slug = row
                                            .try_get_unchecked::<String, _>(1usize)?
                                            .into();
                                        let sqlx_query_as_blog_slug = row
                                            .try_get_unchecked::<String, _>(2usize)?
                                            .into();
                                        let sqlx_query_as_blog_name = row
                                            .try_get_unchecked::<String, _>(3usize)?
                                            .into();
                                        let sqlx_query_as_author_name = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(4usize)?
                                            .into();
                                        let sqlx_query_as_author_slug = row
                                            .try_get_unchecked::<String, _>(5usize)?
                                            .into();
                                        let sqlx_query_as_title = row
                                            .try_get_unchecked::<String, _>(6usize)?
                                            .into();
                                        let sqlx_query_as_content = row
                                            .try_get_unchecked::<String, _>(7usize)?
                                            .into();
                                        let sqlx_query_as_created_at = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(8usize)?
                                            .into();
                                        let sqlx_query_as_updated_at = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(9usize)?
                                            .into();
                                        let sqlx_query_as_likes = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<i64>,
                                                _,
                                            >(10usize)?
                                            .into();
                                        ::std::result::Result::Ok(Self {
                                            id: sqlx_query_as_id,
                                            slug: sqlx_query_as_slug,
                                            blog_slug: sqlx_query_as_blog_slug,
                                            blog_name: sqlx_query_as_blog_name,
                                            author_name: sqlx_query_as_author_name,
                                            author_slug: sqlx_query_as_author_slug,
                                            title: sqlx_query_as_title,
                                            content: sqlx_query_as_content,
                                            created_at: sqlx_query_as_created_at,
                                            updated_at: sqlx_query_as_updated_at,
                                            likes: sqlx_query_as_likes,
                                        })
                                    })
                            }
                        }
                    }
                        .fetch_all(pool)
                        .await?,
                )
            }
            pub async fn get_by_blog_and_post_slug(
                pool: &PgPool,
                blog: String,
                post: String,
            ) -> Result<Self, EntityError> {
                Ok(
                    {
                        {
                            #[allow(clippy::all)]
                            {
                                use ::sqlx::Arguments as _;
                                let arg0 = &(blog);
                                let arg1 = &(post);
                                if false {
                                    use ::sqlx::ty_match::{
                                        WrapSameExt as _, MatchBorrowExt as _,
                                    };
                                    let expr = ::sqlx::ty_match::dupe_value(arg0);
                                    let ty_check = ::sqlx::ty_match::WrapSame::<
                                        &str,
                                        _,
                                    >::new(&expr)
                                        .wrap_same();
                                    let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                        ty_check,
                                        &expr,
                                    );
                                    _ty_check = match_borrow.match_borrow();
                                    ::core::panicking::panic("explicit panic");
                                }
                                if false {
                                    use ::sqlx::ty_match::{
                                        WrapSameExt as _, MatchBorrowExt as _,
                                    };
                                    let expr = ::sqlx::ty_match::dupe_value(arg1);
                                    let ty_check = ::sqlx::ty_match::WrapSame::<
                                        &str,
                                        _,
                                    >::new(&expr)
                                        .wrap_same();
                                    let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                        ty_check,
                                        &expr,
                                    );
                                    _ty_check = match_borrow.match_borrow();
                                    ::core::panicking::panic("explicit panic");
                                }
                                let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                                query_args
                                    .reserve(
                                        2usize,
                                        0
                                            + ::sqlx::encode::Encode::<
                                                sqlx::postgres::Postgres,
                                            >::size_hint(arg0)
                                            + ::sqlx::encode::Encode::<
                                                sqlx::postgres::Postgres,
                                            >::size_hint(arg1),
                                    );
                                query_args.add(arg0);
                                query_args.add(arg1);
                                ::sqlx::query_with::<
                                    sqlx::postgres::Postgres,
                                    _,
                                >(
                                        "SELECT \n\tid::text,\n\tslug,\n\tblog_slug,\n\tblog_name,\n\tauthor_name,\n\tprofile_slug AS author_slug,\n\ttitle,\n\tbody AS content,\n\tto_char(post.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(post.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tCOALESCE(likes.like_count, 0) AS likes\nFROM \"post\" LEFT JOIN (\n\tSELECT\n\t\tblog.id AS blog_id,\n\t\tblog.slug AS blog_slug,\n\t\tblog.name AS blog_name,\n\t\tprofile.id AS profile_id,\n\t\tprofile.display_name AS author_name,\n\t\tprofile_slug\n\tFROM \"blog\" LEFT JOIN (\n\t\tSELECT\n\t\t\tprofile.id,\n\t\t\tprofile.display_name,\n\t\t\t\"user\".username as profile_slug\n\t\tFROM \"profile\" LEFT JOIN \"user\"\n\t\tON user_id = \"user\".id\n\t) \"profile\"\n\tON profile_id=profile.id\n) AS \"blog\" ON post.blog_id = blog.blog_id\nLEFT JOIN (\n\tSELECT post_id, COUNT(*) AS like_count\n\tFROM \"like\"\n\tGROUP BY post_id\n) AS likes ON post.id = likes.post_id\nWHERE blog_slug = $1 AND slug = $2;\n",
                                        query_args,
                                    )
                                    .try_map(|row: sqlx::postgres::PgRow| {
                                        use ::sqlx::Row as _;
                                        let sqlx_query_as_id = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(0usize)?
                                            .into();
                                        let sqlx_query_as_slug = row
                                            .try_get_unchecked::<String, _>(1usize)?
                                            .into();
                                        let sqlx_query_as_blog_slug = row
                                            .try_get_unchecked::<String, _>(2usize)?
                                            .into();
                                        let sqlx_query_as_blog_name = row
                                            .try_get_unchecked::<String, _>(3usize)?
                                            .into();
                                        let sqlx_query_as_author_name = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(4usize)?
                                            .into();
                                        let sqlx_query_as_author_slug = row
                                            .try_get_unchecked::<String, _>(5usize)?
                                            .into();
                                        let sqlx_query_as_title = row
                                            .try_get_unchecked::<String, _>(6usize)?
                                            .into();
                                        let sqlx_query_as_content = row
                                            .try_get_unchecked::<String, _>(7usize)?
                                            .into();
                                        let sqlx_query_as_created_at = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(8usize)?
                                            .into();
                                        let sqlx_query_as_updated_at = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<String>,
                                                _,
                                            >(9usize)?
                                            .into();
                                        let sqlx_query_as_likes = row
                                            .try_get_unchecked::<
                                                ::std::option::Option<i64>,
                                                _,
                                            >(10usize)?
                                            .into();
                                        ::std::result::Result::Ok(Self {
                                            id: sqlx_query_as_id,
                                            slug: sqlx_query_as_slug,
                                            blog_slug: sqlx_query_as_blog_slug,
                                            blog_name: sqlx_query_as_blog_name,
                                            author_name: sqlx_query_as_author_name,
                                            author_slug: sqlx_query_as_author_slug,
                                            title: sqlx_query_as_title,
                                            content: sqlx_query_as_content,
                                            created_at: sqlx_query_as_created_at,
                                            updated_at: sqlx_query_as_updated_at,
                                            likes: sqlx_query_as_likes,
                                        })
                                    })
                            }
                        }
                    }
                        .fetch_one(pool)
                        .await?,
                )
            }
            pub async fn get_by_feed(
                pool: &PgPool,
                feed_id: String,
                limit: i64,
                offset: i64,
            ) -> Result<Vec<Post>, anyhow::Error> {
                match feed_id.as_str() {
                    "new" => {
                        Self::get_by_feed_new(pool, limit, offset)
                            .await
                            .map_err(|_| ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("Cannot find feed"),
                                );
                                error
                            }))
                    }
                    "popular" => {
                        Self::get_by_feed_popular(pool, limit, offset)
                            .await
                            .map_err(|_| ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("Cannot find feed"),
                                );
                                error
                            }))
                    }
                    _ => {
                        Err(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("Cannot find feed"),
                                );
                                error
                            }),
                        )
                    }
                }
            }
            pub async fn get_by_username(
                pool: &PgPool,
                username: String,
            ) -> Result<Vec<Self>, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(username);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "SELECT \n\tid::text,\n\tslug,\n\tblog_slug,\n\tblog_name,\n\tauthor_name,\n\tprofile_slug AS author_slug,\n\ttitle,\n\tbody AS content,\n\tto_char(post.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(post.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tCOALESCE(likes.like_count, 0) AS likes\nFROM \"post\" LEFT JOIN (\n\tSELECT\n\t\tblog.id AS blog_id,\n\t\tblog.slug AS blog_slug,\n\t\tblog.name AS blog_name,\n\t\tprofile.id AS profile_id,\n\t\tprofile.display_name AS author_name,\n\t\tprofile_slug\n\tFROM \"blog\" LEFT JOIN (\n\t\tSELECT\n\t\t\tprofile.id,\n\t\t\tprofile.display_name,\n\t\t\t\"user\".username as profile_slug\n\t\tFROM \"profile\" LEFT JOIN \"user\"\n\t\tON user_id = \"user\".id\n\t) \"profile\"\n\tON profile_id=profile.id\n) AS \"blog\" ON post.blog_id = blog.blog_id\nLEFT JOIN (\n\tSELECT post_id, COUNT(*) AS like_count\n\tFROM \"like\"\n\tGROUP BY post_id\n) AS likes ON post.id = likes.post_id\nWHERE profile_slug = $1;\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_slug = row
                                        .try_get_unchecked::<String, _>(1usize)?
                                        .into();
                                    let sqlx_query_as_blog_slug = row
                                        .try_get_unchecked::<String, _>(2usize)?
                                        .into();
                                    let sqlx_query_as_blog_name = row
                                        .try_get_unchecked::<String, _>(3usize)?
                                        .into();
                                    let sqlx_query_as_author_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?
                                        .into();
                                    let sqlx_query_as_author_slug = row
                                        .try_get_unchecked::<String, _>(5usize)?
                                        .into();
                                    let sqlx_query_as_title = row
                                        .try_get_unchecked::<String, _>(6usize)?
                                        .into();
                                    let sqlx_query_as_content = row
                                        .try_get_unchecked::<String, _>(7usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(8usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(9usize)?
                                        .into();
                                    let sqlx_query_as_likes = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<i64>,
                                            _,
                                        >(10usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        slug: sqlx_query_as_slug,
                                        blog_slug: sqlx_query_as_blog_slug,
                                        blog_name: sqlx_query_as_blog_name,
                                        author_name: sqlx_query_as_author_name,
                                        author_slug: sqlx_query_as_author_slug,
                                        title: sqlx_query_as_title,
                                        content: sqlx_query_as_content,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        likes: sqlx_query_as_likes,
                                    })
                                })
                        }
                    }
                }
                    .fetch_all(pool)
                    .await
            }
            pub async fn get_liked_by_username(
                pool: &PgPool,
                username: String,
            ) -> Result<Vec<Self>, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(username);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "SELECT \n\tid::text,\n\tCOALESCE(slug, \'\') AS slug,\n\tblog_slug,\n\tblog_name,\n\tauthor_name,\n\tprofile_slug AS author_slug,\n\ttitle,\n\tbody AS content,\n\tto_char(post.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(post.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tCOALESCE(likes.like_count, 0) AS likes\nFROM \"post\" LEFT JOIN \n\t(SELECT * FROM \"like\" WHERE profile_id = \n\t\t(SELECT id FROM \"profile\" WHERE user_id =\n\t\t\t(SELECT id FROM \"user\" WHERE username = $1)))\nAS user_likes ON \"post\".id = post_id\nLEFT JOIN (\n\tSELECT\n\t\tblog.id AS blog_id,\n\t\tblog.slug AS blog_slug,\n\t\tblog.name AS blog_name,\n\t\tprofile.id AS profile_id,\n\t\tprofile.display_name AS author_name,\n\t\tprofile_slug\n\tFROM \"blog\" LEFT JOIN (\n\t\tSELECT\n\t\t\tprofile.id,\n\t\t\tprofile.display_name,\n\t\t\t\"user\".username as profile_slug\n\t\tFROM \"profile\" LEFT JOIN \"user\"\n\t\tON user_id = \"user\".id\n\t) \"profile\"\n\tON profile_id=profile.id\n) AS \"blog\" ON post.blog_id = blog.blog_id\nINNER JOIN (\n\tSELECT post_id, COUNT(*) AS like_count\n\tFROM \"like\"\n\tGROUP BY post_id\n) AS likes ON post.id = likes.post_id;\n\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_slug = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    let sqlx_query_as_blog_slug = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(2usize)?
                                        .into();
                                    let sqlx_query_as_blog_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(3usize)?
                                        .into();
                                    let sqlx_query_as_author_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?
                                        .into();
                                    let sqlx_query_as_author_slug = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(5usize)?
                                        .into();
                                    let sqlx_query_as_title = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(6usize)?
                                        .into();
                                    let sqlx_query_as_content = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(7usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(8usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(9usize)?
                                        .into();
                                    let sqlx_query_as_likes = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<i64>,
                                            _,
                                        >(10usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        slug: sqlx_query_as_slug,
                                        blog_slug: sqlx_query_as_blog_slug,
                                        blog_name: sqlx_query_as_blog_name,
                                        author_name: sqlx_query_as_author_name,
                                        author_slug: sqlx_query_as_author_slug,
                                        title: sqlx_query_as_title,
                                        content: sqlx_query_as_content,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        likes: sqlx_query_as_likes,
                                    })
                                })
                        }
                    }
                }
                    .fetch_all(pool)
                    .await
            }
            async fn get_by_feed_new(
                pool: &PgPool,
                limit: i64,
                offset: i64,
            ) -> Result<Vec<Self>, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(limit);
                            let arg1 = &(offset);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    i64,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg1);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    i64,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    2usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg1),
                                );
                            query_args.add(arg0);
                            query_args.add(arg1);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "SELECT \n\tid::text,\n\tslug,\n\tblog_slug,\n\tblog_name,\n\tauthor_name,\n\tprofile_slug AS author_slug,\n\ttitle,\n\tbody AS content,\n\tto_char(post.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(post.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tCOALESCE(likes.like_count, 0) AS likes\nFROM \"post\" LEFT JOIN (\n\tSELECT\n\t\tblog.id AS blog_id,\n\t\tblog.slug AS blog_slug,\n\t\tblog.name AS blog_name,\n\t\tprofile.id AS profile_id,\n\t\tprofile.display_name AS author_name,\n\t\tprofile_slug\n\tFROM \"blog\" LEFT JOIN (\n\t\tSELECT\n\t\t\tprofile.id,\n\t\t\tprofile.display_name,\n\t\t\t\"user\".username as profile_slug\n\t\tFROM \"profile\" LEFT JOIN \"user\"\n\t\tON user_id = \"user\".id\n\t) \"profile\"\n\tON profile_id=profile.id\n) AS \"blog\" ON post.blog_id = blog.blog_id\nLEFT JOIN (\n\tSELECT post_id, COUNT(*) AS like_count\n\tFROM \"like\"\n\tGROUP BY post_id\n) AS likes ON post.id = likes.post_id\nORDER BY post.created_at DESC\nLIMIT $1 OFFSET $2;\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_slug = row
                                        .try_get_unchecked::<String, _>(1usize)?
                                        .into();
                                    let sqlx_query_as_blog_slug = row
                                        .try_get_unchecked::<String, _>(2usize)?
                                        .into();
                                    let sqlx_query_as_blog_name = row
                                        .try_get_unchecked::<String, _>(3usize)?
                                        .into();
                                    let sqlx_query_as_author_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?
                                        .into();
                                    let sqlx_query_as_author_slug = row
                                        .try_get_unchecked::<String, _>(5usize)?
                                        .into();
                                    let sqlx_query_as_title = row
                                        .try_get_unchecked::<String, _>(6usize)?
                                        .into();
                                    let sqlx_query_as_content = row
                                        .try_get_unchecked::<String, _>(7usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(8usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(9usize)?
                                        .into();
                                    let sqlx_query_as_likes = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<i64>,
                                            _,
                                        >(10usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        slug: sqlx_query_as_slug,
                                        blog_slug: sqlx_query_as_blog_slug,
                                        blog_name: sqlx_query_as_blog_name,
                                        author_name: sqlx_query_as_author_name,
                                        author_slug: sqlx_query_as_author_slug,
                                        title: sqlx_query_as_title,
                                        content: sqlx_query_as_content,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        likes: sqlx_query_as_likes,
                                    })
                                })
                        }
                    }
                }
                    .fetch_all(pool)
                    .await
            }
            async fn get_by_feed_popular(
                pool: &PgPool,
                limit: i64,
                offset: i64,
            ) -> Result<Vec<Self>, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(limit);
                            let arg1 = &(offset);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    i64,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg1);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    i64,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    2usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg1),
                                );
                            query_args.add(arg0);
                            query_args.add(arg1);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "SELECT \n\tid::text,\n\tslug,\n\tblog_slug,\n\tblog_name,\n\tauthor_name,\n\tprofile_slug AS author_slug,\n\ttitle,\n\tbody AS content,\n\tto_char(post.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(post.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tCOALESCE(likes.like_count, 0) AS likes\nFROM \"post\" LEFT JOIN (\n\tSELECT\n\t\tblog.id AS blog_id,\n\t\tblog.slug AS blog_slug,\n\t\tblog.name AS blog_name,\n\t\tprofile.id AS profile_id,\n\t\tprofile.display_name AS author_name,\n\t\tprofile_slug\n\tFROM \"blog\" LEFT JOIN (\n\t\tSELECT\n\t\t\tprofile.id,\n\t\t\tprofile.display_name,\n\t\t\t\"user\".username as profile_slug\n\t\tFROM \"profile\" LEFT JOIN \"user\"\n\t\tON user_id = \"user\".id\n\t) \"profile\"\n\tON profile_id=profile.id\n) AS \"blog\" ON post.blog_id = blog.blog_id\nLEFT JOIN (\n\tSELECT post_id, COUNT(*) AS like_count\n\tFROM \"like\"\n\tGROUP BY post_id\n) AS likes ON post.id = likes.post_id\nORDER BY likes DESC\nLIMIT $1 OFFSET $2;\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_slug = row
                                        .try_get_unchecked::<String, _>(1usize)?
                                        .into();
                                    let sqlx_query_as_blog_slug = row
                                        .try_get_unchecked::<String, _>(2usize)?
                                        .into();
                                    let sqlx_query_as_blog_name = row
                                        .try_get_unchecked::<String, _>(3usize)?
                                        .into();
                                    let sqlx_query_as_author_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?
                                        .into();
                                    let sqlx_query_as_author_slug = row
                                        .try_get_unchecked::<String, _>(5usize)?
                                        .into();
                                    let sqlx_query_as_title = row
                                        .try_get_unchecked::<String, _>(6usize)?
                                        .into();
                                    let sqlx_query_as_content = row
                                        .try_get_unchecked::<String, _>(7usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(8usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(9usize)?
                                        .into();
                                    let sqlx_query_as_likes = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<i64>,
                                            _,
                                        >(10usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        slug: sqlx_query_as_slug,
                                        blog_slug: sqlx_query_as_blog_slug,
                                        blog_name: sqlx_query_as_blog_name,
                                        author_name: sqlx_query_as_author_name,
                                        author_slug: sqlx_query_as_author_slug,
                                        title: sqlx_query_as_title,
                                        content: sqlx_query_as_content,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        likes: sqlx_query_as_likes,
                                    })
                                })
                        }
                    }
                }
                    .fetch_all(pool)
                    .await
            }
        }
    }
    mod profile {
        use crate::auth::GitHubUser;
        use serde::{Deserialize, Serialize};
        use sqlx::types::Uuid;
        use sqlx::PgPool;
        #[serde(rename_all = "camelCase")]
        pub struct Profile {
            pub id: Option<String>,
            pub user_id: Option<String>,
            pub display_name: Option<String>,
            pub avatar_url: Option<String>,
            pub created_at: Option<String>,
            pub updated_at: Option<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Profile {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "user_id",
                    "display_name",
                    "avatar_url",
                    "created_at",
                    "updated_at",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.user_id,
                    &self.display_name,
                    &self.avatar_url,
                    &self.created_at,
                    &&self.updated_at,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Profile",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Profile {
            #[inline]
            fn clone(&self) -> Profile {
                Profile {
                    id: ::core::clone::Clone::clone(&self.id),
                    user_id: ::core::clone::Clone::clone(&self.user_id),
                    display_name: ::core::clone::Clone::clone(&self.display_name),
                    avatar_url: ::core::clone::Clone::clone(&self.avatar_url),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    updated_at: ::core::clone::Clone::clone(&self.updated_at),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Profile {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Profile",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "userId",
                        &self.user_id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "displayName",
                        &self.display_name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "avatarUrl",
                        &self.avatar_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "createdAt",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "updatedAt",
                        &self.updated_at,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Profile {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "userId" => _serde::__private::Ok(__Field::__field1),
                                "displayName" => _serde::__private::Ok(__Field::__field2),
                                "avatarUrl" => _serde::__private::Ok(__Field::__field3),
                                "createdAt" => _serde::__private::Ok(__Field::__field4),
                                "updatedAt" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"userId" => _serde::__private::Ok(__Field::__field1),
                                b"displayName" => _serde::__private::Ok(__Field::__field2),
                                b"avatarUrl" => _serde::__private::Ok(__Field::__field3),
                                b"createdAt" => _serde::__private::Ok(__Field::__field4),
                                b"updatedAt" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Profile>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Profile;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Profile",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Profile with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Profile with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Profile with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Profile with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Profile with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Profile with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Profile {
                                id: __field0,
                                user_id: __field1,
                                display_name: __field2,
                                avatar_url: __field3,
                                created_at: __field4,
                                updated_at: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("userId"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "displayName",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "avatarUrl",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "createdAt",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updatedAt",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("userId")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("displayName")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("avatarUrl")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("createdAt")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("updatedAt")?
                                }
                            };
                            _serde::__private::Ok(Profile {
                                id: __field0,
                                user_id: __field1,
                                display_name: __field2,
                                avatar_url: __field3,
                                created_at: __field4,
                                updated_at: __field5,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "userId",
                        "displayName",
                        "avatarUrl",
                        "createdAt",
                        "updatedAt",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Profile",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Profile>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Profile {
            pub fn new(
                user_id: String,
                display_name: String,
                avatar_url: Option<String>,
            ) -> Self {
                Self {
                    id: None,
                    user_id: Some(user_id),
                    display_name: Some(display_name),
                    avatar_url,
                    created_at: None,
                    updated_at: None,
                }
            }
            pub async fn upsert(self, pool: &PgPool) -> Result<Self, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(Uuid::parse_str(&self.user_id.unwrap()).ok());
                            let arg1 = &(self.display_name.unwrap());
                            let arg2 = &(self.avatar_url);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    sqlx::types::Uuid,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg1);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg2);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    3usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg1)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg2),
                                );
                            query_args.add(arg0);
                            query_args.add(arg1);
                            query_args.add(arg2);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "\n            INSERT INTO profile (user_id, display_name, avatar_url)\n            VALUES ($1, $2, $3)\n            ON CONFLICT (user_id) DO UPDATE SET\n                display_name = EXCLUDED.display_name,\n                avatar_url = EXCLUDED.avatar_url,\n                updated_at = NOW()\n            RETURNING \n                id::TEXT, user_id::TEXT, display_name,\n                to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,\n                to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,\n                avatar_url;\n            ",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_user_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    let sqlx_query_as_display_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(2usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(3usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?
                                        .into();
                                    let sqlx_query_as_avatar_url = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(5usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        user_id: sqlx_query_as_user_id,
                                        display_name: sqlx_query_as_display_name,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        avatar_url: sqlx_query_as_avatar_url,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
            }
            #[allow(dead_code)]
            pub async fn get_by_id(
                pool: &PgPool,
                id: String,
            ) -> Result<Self, sqlx::Error> {
                let uuid = Uuid::parse_str(&id).unwrap();
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(uuid);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    sqlx::types::Uuid,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "\n            SELECT \n                id::TEXT,\n                user_id::TEXT, display_name,\n                to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,\n                to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,\n                avatar_url\n            FROM profile \n            WHERE id = $1",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_user_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    let sqlx_query_as_display_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(2usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(3usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?
                                        .into();
                                    let sqlx_query_as_avatar_url = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(5usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        user_id: sqlx_query_as_user_id,
                                        display_name: sqlx_query_as_display_name,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        avatar_url: sqlx_query_as_avatar_url,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
            }
            pub async fn get_by_username(
                pool: &PgPool,
                username: String,
            ) -> Result<Self, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(username);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "SELECT \n\tprofile.id::TEXT as id,\n\tuser_id::TEXT,\n\tto_char(profile.created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(profile.updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tdisplay_name, avatar_url\nFROM profile LEFT JOIN \"user\" ON profile.user_id = \"user\".id\nWHERE username=$1;\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_user_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(2usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(3usize)?
                                        .into();
                                    let sqlx_query_as_display_name = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(4usize)?
                                        .into();
                                    let sqlx_query_as_avatar_url = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(5usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        user_id: sqlx_query_as_user_id,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        display_name: sqlx_query_as_display_name,
                                        avatar_url: sqlx_query_as_avatar_url,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
            }
            pub async fn upsert_from_github_user(
                pool: &PgPool,
                user_id: String,
                github_user: GitHubUser,
            ) -> Result<Self, sqlx::Error> {
                match &github_user {
                    tmp => {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "[{0}:{1}] {2} = {3:#?}\n",
                                    "src/entities/profile.rs",
                                    86u32,
                                    "&github_user",
                                    &tmp,
                                ),
                            );
                        };
                        tmp
                    }
                };
                Self::new(user_id, github_user.name, Some(github_user.avatar_url))
                    .upsert(&pool)
                    .await
            }
        }
    }
    mod upload {
        use serde::{Deserialize, Serialize};
        #[serde(rename_all = "camelCase")]
        pub struct Upload {
            pub id: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Upload {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Upload",
                    "id",
                    &&self.id,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Upload {
            #[inline]
            fn clone(&self) -> Upload {
                Upload {
                    id: ::core::clone::Clone::clone(&self.id),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Upload {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Upload",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Upload {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Upload>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Upload;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Upload",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Upload with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Upload { id: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            _serde::__private::Ok(Upload { id: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Upload",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Upload>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    mod user {
        use crate::auth::GitHubUser;
        use serde::{Deserialize, Serialize};
        use sqlx::PgPool;
        #[serde(rename_all = "camelCase")]
        pub struct User {
            pub id: Option<String>,
            pub created_at: Option<String>,
            pub updated_at: Option<String>,
            pub username: String,
            pub email: String,
            pub github_username: Option<String>,
            pub role: String,
            pub status: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for User {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "created_at",
                    "updated_at",
                    "username",
                    "email",
                    "github_username",
                    "role",
                    "status",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.created_at,
                    &self.updated_at,
                    &self.username,
                    &self.email,
                    &self.github_username,
                    &self.role,
                    &&self.status,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "User",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for User {
            #[inline]
            fn clone(&self) -> User {
                User {
                    id: ::core::clone::Clone::clone(&self.id),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                    updated_at: ::core::clone::Clone::clone(&self.updated_at),
                    username: ::core::clone::Clone::clone(&self.username),
                    email: ::core::clone::Clone::clone(&self.email),
                    github_username: ::core::clone::Clone::clone(&self.github_username),
                    role: ::core::clone::Clone::clone(&self.role),
                    status: ::core::clone::Clone::clone(&self.status),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for User {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "User",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "createdAt",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "updatedAt",
                        &self.updated_at,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "username",
                        &self.username,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "email",
                        &self.email,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "githubUsername",
                        &self.github_username,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "role",
                        &self.role,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "status",
                        &self.status,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for User {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "createdAt" => _serde::__private::Ok(__Field::__field1),
                                "updatedAt" => _serde::__private::Ok(__Field::__field2),
                                "username" => _serde::__private::Ok(__Field::__field3),
                                "email" => _serde::__private::Ok(__Field::__field4),
                                "githubUsername" => _serde::__private::Ok(__Field::__field5),
                                "role" => _serde::__private::Ok(__Field::__field6),
                                "status" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"createdAt" => _serde::__private::Ok(__Field::__field1),
                                b"updatedAt" => _serde::__private::Ok(__Field::__field2),
                                b"username" => _serde::__private::Ok(__Field::__field3),
                                b"email" => _serde::__private::Ok(__Field::__field4),
                                b"githubUsername" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                b"role" => _serde::__private::Ok(__Field::__field6),
                                b"status" => _serde::__private::Ok(__Field::__field7),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<User>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = User;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct User",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct User with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct User with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct User with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct User with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct User with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct User with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct User with 8 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct User with 8 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(User {
                                id: __field0,
                                created_at: __field1,
                                updated_at: __field2,
                                username: __field3,
                                email: __field4,
                                github_username: __field5,
                                role: __field6,
                                status: __field7,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "createdAt",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updatedAt",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "username",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("email"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "githubUsername",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("role"),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("createdAt")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("updatedAt")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("username")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("email")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("githubUsername")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("role")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("status")?
                                }
                            };
                            _serde::__private::Ok(User {
                                id: __field0,
                                created_at: __field1,
                                updated_at: __field2,
                                username: __field3,
                                email: __field4,
                                github_username: __field5,
                                role: __field6,
                                status: __field7,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "createdAt",
                        "updatedAt",
                        "username",
                        "email",
                        "githubUsername",
                        "role",
                        "status",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "User",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<User>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl User {
            pub fn new(
                username: String,
                email: String,
                github_username: String,
            ) -> Self {
                User {
                    id: None,
                    created_at: None,
                    updated_at: None,
                    username,
                    email,
                    github_username: Some(github_username),
                    role: "user".to_string(),
                    status: "active".to_string(),
                }
            }
            pub async fn upsert(self, pool: &PgPool) -> Result<Self, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(self.username);
                            let arg1 = &(self.email);
                            let arg2 = &(self.github_username.unwrap());
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg1);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg2);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    3usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg1)
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg2),
                                );
                            query_args.add(arg0);
                            query_args.add(arg1);
                            query_args.add(arg2);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "INSERT INTO \"user\" (username, email, github_username)\nVALUES ($1, $2, $3)\nON CONFLICT (username) DO UPDATE SET\n\temail = EXCLUDED.email,\n\tgithub_username = EXCLUDED.github_username,\n\tupdated_at = now()\nRETURNING \n\tid::TEXT,\n\tto_char(\"user\".created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(\"user\".updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tusername,\n\temail, \n\tgithub_username, \n\trole,\n\tstatus;\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(2usize)?
                                        .into();
                                    let sqlx_query_as_username = row
                                        .try_get_unchecked::<String, _>(3usize)?
                                        .into();
                                    let sqlx_query_as_email = row
                                        .try_get_unchecked::<String, _>(4usize)?
                                        .into();
                                    let sqlx_query_as_github_username = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(5usize)?
                                        .into();
                                    let sqlx_query_as_role = row
                                        .try_get_unchecked::<String, _>(6usize)?
                                        .into();
                                    let sqlx_query_as_status = row
                                        .try_get_unchecked::<String, _>(7usize)?
                                        .into();
                                    ::std::result::Result::Ok(Self {
                                        id: sqlx_query_as_id,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        username: sqlx_query_as_username,
                                        email: sqlx_query_as_email,
                                        github_username: sqlx_query_as_github_username,
                                        role: sqlx_query_as_role,
                                        status: sqlx_query_as_status,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
            }
            pub async fn upsert_from_github_user(
                pool: &PgPool,
                github_user: GitHubUser,
            ) -> Result<Self, sqlx::Error> {
                let user = Self::new(
                    github_user.login.clone(),
                    github_user.email.clone(),
                    github_user.login.clone(),
                );
                user.upsert(pool).await
            }
            #[allow(dead_code)]
            pub async fn get_by_username(
                pool: &PgPool,
                username: String,
            ) -> Result<Self, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(username);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "SELECT \n\tid::TEXT,\n\tto_char(\"user\".created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(\"user\".updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tusername, email, github_username, role, status\nFROM \"user\" \nWHERE username=$1;\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(2usize)?
                                        .into();
                                    let sqlx_query_as_username = row
                                        .try_get_unchecked::<String, _>(3usize)?
                                        .into();
                                    let sqlx_query_as_email = row
                                        .try_get_unchecked::<String, _>(4usize)?
                                        .into();
                                    let sqlx_query_as_github_username = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(5usize)?
                                        .into();
                                    let sqlx_query_as_role = row
                                        .try_get_unchecked::<String, _>(6usize)?
                                        .into();
                                    let sqlx_query_as_status = row
                                        .try_get_unchecked::<String, _>(7usize)?
                                        .into();
                                    ::std::result::Result::Ok(User {
                                        id: sqlx_query_as_id,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        username: sqlx_query_as_username,
                                        email: sqlx_query_as_email,
                                        github_username: sqlx_query_as_github_username,
                                        role: sqlx_query_as_role,
                                        status: sqlx_query_as_status,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
            }
            #[allow(dead_code)]
            pub async fn get_by_github_username(
                pool: &PgPool,
                github_username: String,
            ) -> Result<Self, sqlx::Error> {
                {
                    {
                        #[allow(clippy::all)]
                        {
                            use ::sqlx::Arguments as _;
                            let arg0 = &(github_username);
                            if false {
                                use ::sqlx::ty_match::{
                                    WrapSameExt as _, MatchBorrowExt as _,
                                };
                                let expr = ::sqlx::ty_match::dupe_value(arg0);
                                let ty_check = ::sqlx::ty_match::WrapSame::<
                                    &str,
                                    _,
                                >::new(&expr)
                                    .wrap_same();
                                let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(
                                    ty_check,
                                    &expr,
                                );
                                _ty_check = match_borrow.match_borrow();
                                ::core::panicking::panic("explicit panic");
                            }
                            let mut query_args = <sqlx::postgres::Postgres as ::sqlx::database::HasArguments>::Arguments::default();
                            query_args
                                .reserve(
                                    1usize,
                                    0
                                        + ::sqlx::encode::Encode::<
                                            sqlx::postgres::Postgres,
                                        >::size_hint(arg0),
                                );
                            query_args.add(arg0);
                            ::sqlx::query_with::<
                                sqlx::postgres::Postgres,
                                _,
                            >(
                                    "SELECT \n\tid::TEXT,\n\tto_char(\"user\".created_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS created_at,\n\tto_char(\"user\".updated_at, \'YYYY-MM-DDThh:mm:ss.ss\') AS updated_at,\n\tusername, email, github_username, role, status\nFROM \"user\" \nWHERE github_username=$1;\n",
                                    query_args,
                                )
                                .try_map(|row: sqlx::postgres::PgRow| {
                                    use ::sqlx::Row as _;
                                    let sqlx_query_as_id = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(0usize)?
                                        .into();
                                    let sqlx_query_as_created_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(1usize)?
                                        .into();
                                    let sqlx_query_as_updated_at = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(2usize)?
                                        .into();
                                    let sqlx_query_as_username = row
                                        .try_get_unchecked::<String, _>(3usize)?
                                        .into();
                                    let sqlx_query_as_email = row
                                        .try_get_unchecked::<String, _>(4usize)?
                                        .into();
                                    let sqlx_query_as_github_username = row
                                        .try_get_unchecked::<
                                            ::std::option::Option<String>,
                                            _,
                                        >(5usize)?
                                        .into();
                                    let sqlx_query_as_role = row
                                        .try_get_unchecked::<String, _>(6usize)?
                                        .into();
                                    let sqlx_query_as_status = row
                                        .try_get_unchecked::<String, _>(7usize)?
                                        .into();
                                    ::std::result::Result::Ok(User {
                                        id: sqlx_query_as_id,
                                        created_at: sqlx_query_as_created_at,
                                        updated_at: sqlx_query_as_updated_at,
                                        username: sqlx_query_as_username,
                                        email: sqlx_query_as_email,
                                        github_username: sqlx_query_as_github_username,
                                        role: sqlx_query_as_role,
                                        status: sqlx_query_as_status,
                                    })
                                })
                        }
                    }
                }
                    .fetch_one(pool)
                    .await
            }
        }
    }
    mod webhook {
        use serde::{Deserialize, Serialize};
        #[serde(rename_all = "camelCase")]
        pub struct Webhook {
            pub id: String,
            pub origin: String,
            pub body: String,
            pub created_at: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Webhook {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Webhook",
                    "id",
                    &self.id,
                    "origin",
                    &self.origin,
                    "body",
                    &self.body,
                    "created_at",
                    &&self.created_at,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Webhook {
            #[inline]
            fn clone(&self) -> Webhook {
                Webhook {
                    id: ::core::clone::Clone::clone(&self.id),
                    origin: ::core::clone::Clone::clone(&self.origin),
                    body: ::core::clone::Clone::clone(&self.body),
                    created_at: ::core::clone::Clone::clone(&self.created_at),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Webhook {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Webhook",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "origin",
                        &self.origin,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "body",
                        &self.body,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "createdAt",
                        &self.created_at,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Webhook {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "origin" => _serde::__private::Ok(__Field::__field1),
                                "body" => _serde::__private::Ok(__Field::__field2),
                                "createdAt" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"origin" => _serde::__private::Ok(__Field::__field1),
                                b"body" => _serde::__private::Ok(__Field::__field2),
                                b"createdAt" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Webhook>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Webhook;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Webhook",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Webhook with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Webhook with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Webhook with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Webhook with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Webhook {
                                id: __field0,
                                origin: __field1,
                                body: __field2,
                                created_at: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("origin"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("body"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "createdAt",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("origin")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("body")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("createdAt")?
                                }
                            };
                            _serde::__private::Ok(Webhook {
                                id: __field0,
                                origin: __field1,
                                body: __field2,
                                created_at: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "origin",
                        "body",
                        "createdAt",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Webhook",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Webhook>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub use blog::Blog;
    pub use feed::Feed;
    pub use like::Like;
    pub use post::Post;
    pub use profile::Profile;
    pub use user::User;
}
mod store {
    use crate::auth::Client;
    use sqlx::PgPool;
    /// Store is the shared state of the application.
    /// It contains a pool of Postgres connections and a client for handling authentication.
    pub struct Store {
        pub pool: PgPool,
        pub auth_client: Client,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Store {
        #[inline]
        fn clone(&self) -> Store {
            Store {
                pool: ::core::clone::Clone::clone(&self.pool),
                auth_client: ::core::clone::Clone::clone(&self.auth_client),
            }
        }
    }
    impl Store {
        pub fn new(pool: PgPool, auth_client: Client) -> Self {
            Store { pool, auth_client }
        }
    }
}
fn main() {
    let body = async {
        shuttle_runtime::start(loader).await;
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
async fn loader(
    mut factory: shuttle_runtime::ProvisionerFactory,
    mut resource_tracker: shuttle_runtime::ResourceTracker,
) -> shuttle_axum::ShuttleAxum {
    use shuttle_runtime::Context;
    use shuttle_runtime::{Factory, ResourceBuilder};
    let secret_store = shuttle_runtime::get_resource(
            shuttle_secrets::Secrets::new(),
            &mut factory,
            &mut resource_tracker,
        )
        .await
        .context({
            let res = ::alloc::fmt::format(
                format_args!("failed to provision {0}", "shuttle_secrets :: Secrets"),
            );
            res
        })?;
    __shuttle_axum(secret_store).await
}
async fn __shuttle_axum(secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let db_connection_str = secret_store
        .get("DATABASE_URL")
        .expect("DATABASE_URL environment variable not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to database");
    let client_id = secret_store
        .get("CLIENT_ID")
        .expect("CLIENT_ID environment variable not set");
    let client_secret = secret_store
        .get("CLIENT_SECRET")
        .expect("CLIENT_SECRET environment variable not set");
    let post_auth_redirect_uri = secret_store
        .get("POST_AUTH_REDIRECT_URI")
        .expect("POST_AUTH_REDIRECT_URI environment variable not set");
    let auth_client = auth::Client::new(
        client_id,
        client_secret,
        post_auth_redirect_uri,
    );
    let store = store::Store::new(pool, auth_client);
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);
    let router = Router::new()
        .route("/v1/ready", get(api::ready::ready))
        .route("/v1/blogs", post(api::blogs::upsert_blog))
        .route(
            "/v1/blogs/:blog_slug",
            get(api::blogs::get_blog_by_blog_slug).delete(api::blogs::delete_blog),
        )
        .route(
            "/v1/blogs/:blog_slug/posts",
            get(api::blogs::get_blog_posts_by_blog_slug),
        )
        .route(
            "/v1/blogs/:blog_slug/posts/:post_slug",
            get(api::blogs::get_post_by_blog_and_post_slug),
        )
        .route("/v1/feeds/:id", get(api::feeds::get_feed_by_id))
        .route("/v1/feeds/:id/posts", get(api::feeds::get_feed_posts_by_id))
        .route("/v1/auth/login", get(api::auth::login))
        .route("/v1/auth/callback", get(api::auth::callback))
        .route("/v1/posts/:post_id", get(api::posts::get_post_by_post_id))
        .route("/v1/posts/:post_id/likes", post(api::posts::post_like_to_post))
        .route("/v1/profiles/:username", get(api::profiles::get_profile_by_username))
        .route("/v1/profiles/:username/blogs", get(api::profiles::get_blog_by_username))
        .route("/v1/profiles/:username/posts", get(api::profiles::get_posts_by_username))
        .route(
            "/v1/profiles/:username/likes",
            get(api::profiles::get_liked_posts_by_username),
        )
        .route(
            "/v1/profiles/:username/likes/ids",
            get(api::profiles::get_likes_ids_by_username),
        )
        .route("/v1/users/:username", get(api::users::get_user_by_username))
        .route("/v1/likes", post(api::likes::post_like))
        .route("/v1/likes/:post_id/:profile_id", delete(api::likes::delete_like))
        .with_state(store)
        .layer(cors_layer);
    Ok(router.into())
}

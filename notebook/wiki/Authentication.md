# Authentication

Devy authenticates users from the [[Backend]] API. A user calls to the `/v1/auth/login` route. They will be forwarded to GitHub's OAuth endpoint. After authentication, they will be forwarded to `/v1/auth/callback`. 

The callback endpoint expects a `code` to be present in the query string of the URL. The callback handler will exchange this code for a token. That token is used to grab information about the GitHub user. This information is looked up in the database to find a matching user and profile. If these are not found, they are created.

The user and profile matching the GitHub user are used to build a Session. 

---

I want to protect several routes on the API. I'm not sure if it makes more sense to implement authentication on the frontend or the backend. The frontend has a server component so it is feasible to implement it there.

## Desired Behavior

- A user can log in with GitHub.
- That login grants the backend access to clone repositories as the user.

## GitHub App vs OAuth App

[Differences between GitHub Apps and OAuth apps - GitHub Docs](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/differences-between-github-apps-and-oauth-apps)

I want this to be a GitHub App so I can perform actions on the user's behalf.

## Frontend vs Backend Implementation

### Frontend implementation

This would be based on Next Auth. A user would be redirected to that login flow with GitHub and get an encrypted JWT with their login information. The frontend would send that JWT along with any requests to the backend. The backend could verify the authenticity of the JWT by sharing a secret with the frontend.

What I don't like about this approach is the reliance on a shared token between the frontend and the backend. At the moment, this token would be shared by having the same environment variable present in both environments. If that token gets compromised, an attacker would be able to forge JWTs that would be accepted by the backend.

The only way to reset the secret would be to invalidate all logins and re-deploy the frontend and backend in order for them to load the secret.

### Backend implementation

I think this is what I will go with.

The backend will do the OAuth flow with GitHub and Next Auth will leverage this. That way I can hold sessions in the frontend, and not share the JWT secret across both systems. I can more readily swap the JWT secret and even have them automatically cycle.

[Build Auth Into Your Rust Web Application (OAuth2)](https://www.youtube.com/watch?v=S8Usi3zsLIs)

[Authenticating with your backend using NextAuth.js - DEV Community](https://dev.to/ekrresa/authenticating-with-your-backend-using-nextauthjs-25g)

[github.rs](https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github.rs)

[An Introduction to OAuth 2 | DigitalOcean](https://www.digitalocean.com/community/tutorials/an-introduction-to-oauth-2)

## Architecture

The frontend will have a link "Sign in" which sends the user to `/api/auth/login`. This is hosted on the backend and the request will be forwarded to `api.devy.page/auth/login`. This request will have a param with the current path of the user when they clicked the "Sign in" link.

The `login` path will forward the user to the GitHub login page. Once the user logs in, they will be forwarded to `api.devy.page/auth/callback`. This 


[JWT Authentication - Shuttle](https://docs.shuttle.rs/examples/rocket-jwt-authentication)
 
https://github.com/shuttle-hq/shuttle-examples

https://docs.rs/rocket_oauth2/latest/rocket_oauth2/

https://www.shuttle.rs/blog/2023/08/30/using-oauth-with-axum




# About the user authorization callback URL

You can specify a URL that users will be redirected to after they authorize a GitHub App.

When you register a GitHub App, you can specify a callback URL. When you use the web application flow to generate a user access token in order to act on behalf of a user, users will be redirected to the callback URL after they authorize the GitHub App.

You can specify up to 10 callback URLs. If you specify multiple callback URLs, you can use the `redirect_uri` parameter when you prompt the user to authorize your GitHub App, to indicate which callback URL the user should be redirected to. If you do not specify `redirect_uri`, the first callback URL will be used. For more information about using the `redirect_uri` parameter, see "[Generating a user access token for a GitHub App](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)".

The callback URL is different from the setup URL. Users are redirected to the setup URL after they install a GitHub App. Users are redirected to the callback URL when they authorize a GitHub App via the web application flow. For more information, see "[About the setup URL](https://docs.github.com/en/apps/creating-github-apps/setting-up-a-github-app/about-the-setup-url)."

For more information about generating user access tokens, see "[Generating a user access token for a GitHub App](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)". For more information about registering a GitHub App, see "[Registering a GitHub App](https://docs.github.com/en/apps/creating-github-apps/setting-up-a-github-app/creating-a-github-app)." For more information about modifying a GitHub App registration, see "[Modifying a GitHub App registration](https://docs.github.com/en/apps/maintaining-github-apps/modifying-a-github-app)."

Press alt+up to activate




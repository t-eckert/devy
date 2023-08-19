pub mod controller {
    use rocket_db_pools::Connection;
    use crate::db::DB;
    use super::model::Blog;
    use crate::entities::{Post, Profile};
    use crate::tables;
    pub struct BlogController {}
    impl BlogController {
        pub async fn upsert(mut db: Connection<DB>, blog: Blog) -> Option<Blog> {
            let blog = {
                {
                    #[allow(clippy::all)]
                    {
                        use ::sqlx::Arguments as _;
                        let arg0 = &(blog.name);
                        let arg1 = &(blog.slug);
                        let arg2 = &(blog.username);
                        let arg3 = &(blog.description);
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
                                "\n            INSERT INTO \"blog\" (\"profile_id\", \"name\", \"slug\", \"description\")\n            VALUES (\n                (SELECT profile_id from \"user\" WHERE username=$3),\n                $1, $2, $4\n            )\n            RETURNING \n                name, slug,\n                to_char(created_at, 'YYYY-MM-DDThh:mm:ss.sss') AS created_at,\n                to_char(updated_at, 'YYYY-MM-DDThh:mm:ss.sss') AS updated_at,\n                'analogue' as username, 'Anna Logue' as display_name, description;\n            ",
                                query_args,
                            )
                            .try_map(|row: sqlx::postgres::PgRow| {
                                use ::sqlx::Row as _;
                                let sqlx_query_as_name = row
                                    .try_get_unchecked::<String, _>(0usize)?;
                                let sqlx_query_as_slug = row
                                    .try_get_unchecked::<String, _>(1usize)?;
                                let sqlx_query_as_created_at = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(2usize)?;
                                let sqlx_query_as_updated_at = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(3usize)?;
                                let sqlx_query_as_username = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(4usize)?;
                                let sqlx_query_as_display_name = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(5usize)?;
                                let sqlx_query_as_description = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(6usize)?;
                                Ok(Blog {
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
                .fetch_one(&mut *db)
                .await;
            match blog {
                Ok(blog) => Some(blog),
                Err(e) => {
                    {
                        ::std::io::_print(format_args!("Error: {0}\n", e));
                    };
                    None
                }
            }
        }
        pub async fn get_by_slug(mut db: Connection<DB>, slug: String) -> Option<Blog> {
            let blog = {
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
                                "\n            SELECT \n                name, slug,\n                to_char(blog.created_at, 'YYYY-MM-DDThh:mm:ss.sss') AS created_at,\n                to_char(blog.updated_at, 'YYYY-MM-DDThh:mm:ss.sss') AS updated_at,\n                username, display_name, description\n            FROM \"blog\" LEFT JOIN (\n                SELECT profile.id, display_name, username\n                FROM \"profile\" LEFT JOIN \"user\"\n                ON user_id=\"user\".id\n            ) AS \"profile\" ON profile_id=\"profile\".id\n            WHERE slug=$1;\n            ",
                                query_args,
                            )
                            .try_map(|row: sqlx::postgres::PgRow| {
                                use ::sqlx::Row as _;
                                let sqlx_query_as_name = row
                                    .try_get_unchecked::<String, _>(0usize)?;
                                let sqlx_query_as_slug = row
                                    .try_get_unchecked::<String, _>(1usize)?;
                                let sqlx_query_as_created_at = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(2usize)?;
                                let sqlx_query_as_updated_at = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(3usize)?;
                                let sqlx_query_as_username = row
                                    .try_get_unchecked::<String, _>(4usize)?;
                                let sqlx_query_as_display_name = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(5usize)?;
                                let sqlx_query_as_description = row
                                    .try_get_unchecked::<
                                        ::std::option::Option<String>,
                                        _,
                                    >(6usize)?;
                                Ok(Blog {
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
                .fetch_one(&mut *db)
                .await;
            match blog {
                Ok(blog) => Some(blog),
                Err(e) => {
                    {
                        ::std::io::_print(format_args!("Error: {0}\n", e));
                    };
                    None
                }
            }
        }
    }
}

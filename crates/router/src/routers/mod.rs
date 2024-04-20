mod auth_router;
mod blogs_router;
mod feeds_router;
mod forms_router;
mod likes_router;
mod profiles_router;
mod uploads_router;
mod users_router;

pub use auth_router::AuthRouter;
pub use blogs_router::BlogsRouter;
pub use feeds_router::FeedsRouter;
pub use forms_router::FormsRouter;
pub use likes_router::LikesRouter;
pub use profiles_router::ProfilesRouter;
pub use uploads_router::UploadsRouter;
pub use users_router::UsersRouter;

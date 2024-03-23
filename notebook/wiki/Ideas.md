# Ideas

- Users should have achievements
- Show little dots for "presence" when other users are reading the same post
- Before a build can be done for the backend, the command `cargo sqlx prepare` must be run to create a document that is used for "offline" building. Make sure the version of the `sqlx-cli` matches the version in `Cargo.toml`, otherwise the process will fail to find any queries.
- A user can write a blog that is in response to another blog. 
- It would be cool to let people curate their own feeds and share them. 
- I could add some cool interactivity to [[Devy]] with [g9 Gallery | Automatically Interactive Graphics](http://omrelli.ug/g9/gallery/).
- Customer feedback with [Canny](https://canny.io/)
- 

## `devyctl` 

This is a control system for admin processes in Devy.
## CLI

I want to build a CLI for interacting with Devy as a user. This would be a much later on project.

I just think it would be fun to have this programmable system.

By default, it will present navigable lists, almost like simplified web pages in the terminal

Possible commands:
```
devy feeds new
devy feeds popular
devy profiles t-eckert
devy profiles t-eckert [posts|bookmarks|blogs|likes]
devy blogs machine-learning-for-dummies
```
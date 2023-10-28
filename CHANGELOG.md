# Changelog

## `v0.2.0` 

`28 October 2023`

In this release, I am still experimenting with deployment and getting the development process to the point where releases and testing is automated.
I tried out a SaaS for deploying the backend, but returned to a platform where I deploy a Docker image. I know in the next phase of this project, I will
need to have a Git binary available for cloning repositories so having a Docker image where I can ensure its presence is vital.

I also swapped out the backend from using Rocket to using Axum. Both are great frameworks, I just needed a different programming model for what I was 
trying to do. Axum has less magic which allowed me to extend it as I needed. This did make this release take longer as I needed to reimplement authentication.

[Pull request](https://github.com/t-eckert/devy/pull/8)

### Frontend

- Set the default feed to `New`.
- Add a footer to the site.
- Add 404 and 500 error pages.
- Add a loading component.
- Users can now "like" posts.
- Authentication is stored in browser local storage.
- Feeds of posts are paginated.
- Users can logout.
- Posts show how many likes they have.
- There are now profile pages with users' posts and the posts they have liked.

### Backend

- Add SSL for backend.
- Implement `/feeds/new` and `/feeds/popular`.
- Implement `/profiles/:username`.
- Profiles and usernames are upserted from GitHub on login.
- Implement `POST` and `DELETE` for blogs.
- Every `profile` must reference a unique `user_id`.
- API routes are now prefixed with `/v1`.

### Development

- Update Makefile for builds.
- Add a README for the frontend.
- Add a README for the integration tests.
- Implement a `lib` directory for the frontend.
- Move from a controller model to using entities.
- Create a `devy-test-db` image for local development.
- Add tracing to the backend.
- Add "like" entities to the seed.
- Integration tests can test equality of only keys contained in "expected" mapping.
- Integration tests are run on every push and pull request.
- There is now tooling for generating likes from posts and profiles.


## `v0.1.4` Francis Scott Would Have Been Disappointed

`16 September 2023`

- Fix bug where keys not being passed to blogs in member card would crash build.

## `v0.1.3` You Look Nice

`16 September 2023`

- Display profile information on the profile page for each user.
- Fix issue with Shoulder not rendering on desktop.

## `v0.1.2` Target Adjustment

- Split out the API to prevent redirection issues with path forwarding.
- Show "NotFound" when a Profile is not found.
- Add libssl to backend Dockerfile.
- Store the full token in session.
- Hide "Shoulder" in mobile view.

## `v0.1.1` Who are you?

`9 September 2023`

- Now you can log in with your GitHub account.

## `v0.1.0` First Mover Advantage

`1 September 2023`

- The Devy site is now live, without most of its features, just a single blog and profile: mine.

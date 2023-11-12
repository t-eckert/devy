# Changelog

## `v0.3.0` Clones!

`12 November 2023`

This release lays the groundwork for the repo uploading feature. It adds an endpoint to the backend that can accept `Repo` objects
which are cloned into a directory named by a UUID, then just deleted. I also added a nice menu component and Vercel Analytics.

[Pull Request](https://github.com/t-eckert/devy/pull/11)

### Frontend

- Create a nice menu component.

### Backend

- Add `POST` `/v1/uploads` route.
- Uploads sent to the `/v1/uploads` route are cloned into random directories under `/tmp`.

### Development

- Add [Vecel Analytics](https://vercel.com/analytics) to the frontend.

## `v0.2.2` You Can't Like All of Them

`4 November 2023`

This update improves the behavior of the like button with optimistic updates
and fixes a bug where the endpoint to get all the posts liked by a user would
just return all posts.

[Pull Request](https://github.com/t-eckert/devy/pull/10)

### Frontend

-   Use ReactQuery at layout level to provide a query client.
-   Update Storybook.
-   Update Next to 14.
-   Add Preview to Logomark.
-   Increase page size to 25.
-   Add optimistic updates to like button.
-   Make the like button more fun and reactive.
-   Add a border to the post preview on hover.

### Backend

-   Fix issue where users are reported to like all posts.

## `v0.2.1` Sanding Down the Edges

`2 November 2023`

This minor release mostly affects component organization and addresses some of the hard edges I skipped over when doing the initial development of `v0.2.x`.
I now have a great setup with [Storybook](https://storybook.js.org/) that I can use to test out components I develop. I also configured the set color palette
for the site so that it will work in dark mode and an eventual light mode.

[Pull Request](https://github.com/t-eckert/devy/pull/9)

### Frontend

- Add an opaque background to the menu.
- Add an about section to profiles.
- Center the feed more nicely on the home page.
- Configure the different button types.
- Add a counter component that will abbreviate large numbers.

## `v0.2.0` It's More Likeable

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

- The Devy site is now live.

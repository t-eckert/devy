# [[Devy]] Upload Design

## Background

Devy is a community blogging site, like Medium, where users manage their content from GitHub repositories. It requires an event-driven system to update content on Devy when a user pushes an update to their repo or connects a repo to their Devy profile. On Devy, a user has a profile and may have multiple blogs that each have multiple posts.

Devy blogs map directly to GitHub repos. Markdown files within those repos map directly to posts in the blog. A configuration file at the root of the repo called `devy.toml` may be optionally used to configure the upload behavior of the blog.

The GitHub repository is always the source of truth for content. There is no way to change post content from Devy itself. In this way, Devy can be thought of as a read-only replica of the markdown content in a repository. 

## Design Goals

Reliability, consistency, and idempotency are more important to this design than speed or efficiency. Because this process is core to the site and deals with user content, it must be resilient against corrupting the blogs of users. In this design, I am assuming that the target user will value correctness of content over the time taken to process a given upload after it is pushed. 

There is a limit to this tradeoff. I don't want the time between an upload trigger and content being refreshed to take longer than 5 minutes in more than 0.01% of cases. I would like the mean time to upload to take <2 minutes. If this objective is not being met, the design is intended to be horizontally scalable. More instances of the backend can be created to ease this bottleneck. The database layer is not anticipated to be a bottleneck at this level of service.
## Terminology

- A **repository** or **repo** is a GitHub repository which includes its files, metadata, and the events that it owns such as `push` events. Only one branch of the repository can map to the current state of a blog. The repository also has a representation in Devy as `Repo`.
- An **upload** is data tracking the progress of an event that flows from an upload trigger to the upload workflow through to completion of the event. A representation of the upload will exist in Devy as `Upload`. After the completion of the upload event, whether it completes successfully or unsuccessfully, the `Upload` will be stored in its most up-to-date state in the database.

## Application Architecture Overview

The frontend accessible at `https://devy.page` and is responsible for rendering content, managing authentication and local state for the user, and providing a user interface for the user's Devy account. This includes being able to trigger new upload events directly or as a side-effect of connecting a GitHub repository to a blog.

The backend is accessible through an API at `https://api.devy.page` and is responsible for handling API requests, managing authentication using OAuth2, and running the upload workflow. 

The database is a storage layer for all Devy state persistence. It may only be accessed from the backend.
## The Upload Workflow
### Triggering an Upload

An upload may be triggered in two ways:

- A `POST` to `api.devy.page/v1/uploads/<username>/<blog-slug>` which must have a valid JWT token in the header. This JWT will be checked to ensure that the user who sent the request owns the blog.
- A webhook from GitHub posted to `api.devy.page/v1/webhooks`. These webhooks will be checked for the authenticity of their source by examining the SSL cert of the sender and possibly their IP. If a webhook is received, it will be stored in the database for future reference.
### Creating the Upload Entity

Once a valid upload trigger occurs, the endpoint handler that received that trigger will be responsible for creating an `Upload` entity based on the data received.   That endpoint handler will call the function `handle_upload` which will spin up a separate process to manage the upload workflow. 

The upload workflow takes longer than is reasonable to make a user wait for an HTTP response, so the endpoint handler, after kicking off the upload workflow, will return `201 Received` to the caller. If the process spawning fails or the request is otherwise malformed, a different response may be sent to the caller, informing them of the error.

Throughout the upload workflow, the `Upload` entity exists as both an in-memory representation and as a value in the database which are constantly synchronized with timestamped, idempotent updates. If the in-memory representation is lost, it can be rehydrated from database representation and replayed from wherever the upload failed.

When `handle_upload` receives the `Upload` entity, it assigns it to an available background worker or spawns a new one if needed. Multiple uploads may be handled by the backend concurrently, including multiple uploads for the same blog. This must be taken into account when 

## Entities

Repo
```
{
	id uuid
	owner uuid
	url text
	created_at dtz
	updated_at dtz
}
```

Upload
```
{
	id uuid
	status text
	worker text	
	logs text
	
}
```

Blog
```
{
	id uuid
}
```





---

Devy needs to take uploads from users based on webhooks from GitHub. Those webhooks should be used to trigger a background process in the backend that pulls down the repository of the user and updates that user's blog.

A single repository maps to a single blog.
A user that does not support the project on GitHub can only have one blog.
Because this work will be implemented before user support is implemented, every user will only get one blog.

Designing the happy path.
A user has already linked their account to a given GitHub repository --> creating a blog from it.
When the user pushes to their repo, a webhook will be sent to `api.devy.page/v1/webhooks`.

I need to verify the authenticity of the GitHub webhook by looking at the IP of the sender or their SSL cert to ensure the post came from GitHub.

Statuses
- `received`: default status when a new `Upload` is created.
- `verified`: the `Upload` is from a reputable source.
* `cancelled`: the `Upload` was cancelled for a reason unrelated to an error.
- `failed`: the `Upload` was stopped due to an error.
- `scheduled`: the `Upload` has been assigned to a worker 

Cancelling uploads
An upload may need to be cancelled because another upload for the same repository was triggered after the itself but before it finishes. An upload may also be cancelled manually.


I need to make a `Snapshot` system for comparing what the posts look like in the database with the posts from the latest push. Then the diff can give a map of instructions and the uploader can act on that.



## Uploader

1. Clone the repository into memory.
2. Get the `blog` that the repository points to.
3. Get the latest successful upload from that `blog`.
4. Compare the SHA of that upload with the SHA of the current upload.
5. Diff the two and create a list of markdown files that should be updated, created, or deleted.
6. Delete every `post` that should be deleted.
7. Create every new `post`.
8. Update every existing `post` that needs updating.
9. Set the status of the `upload` to `COMPLETE`.


- [x] An upload is received by the uploader. 
- [ ] It checks for a repo with that url
- [ ] It gets the blog and user for that repo. 
- [x] It clones the repo
- [ ] It collects every markdown file and adds the path to a hash table with the text as its key. 
- [ ] For each key, it compares the contents with the existing contents. 
- [ ] Diffs are sent to the updater
- [ ] New ones are sent to creator
- [x] Delete the repo 
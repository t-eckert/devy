# Authentication, Authorization, and User Management

Devy leverages GitHub for user authentication using OAuth2. I want
to build on this system to enable other authentication sources.
This document lays out a model of identifying users and their
permissions in an extensible manner.

The authentication model is based on session and refresh tokens
with an approach that allows for revoking sessions while safely
enabling long-running sign-ins.

## Users

When a user signs up for Devy, a user entry is populated in the database.
The data comprising the user are split across three relations in the
database: identity, user, and profile.

Identity is a small relation that holds the globally unique identifiable
information for a user:

```
Identity {
  id: UUID
  username: TEXT
  email: TEXT
  user_id: UUID
  profile_id: UUID
  created_at: TIMESTAMPZ
  updated_at: TIMESTAMPZ
}
```

Both the username and the email of a user must each be globally unique.

User is a relation that contains information relevant to the user's
permissions and authentication methods.

```
User {
  id: UUID
  identity_id: UUID
  role: ROLE enum [user,admin]
  status: STATUS enum [active,inactive,suspended,deleted]
  is_github_auth_enabled: BOOLEAN
  github_username: TEXT
  created_at: TIMESTAMPZ
  updated_at: TIMESTAMPZ
}
```

Profile is a relation that contains the public facing information
about a user. This includes being the identifier for "likes", "bookmarks",
and "following" for example.

```
Profile {
  id: UUID
  identity_id: UUID
  display_name: TEXT
  avatar_url: TEXT
  bio: TEXT
  website_url: TEXT
  twitter_username: TEXT
  github_username: TEXT
  bluesky_url: TEXT
  linkedin_url: TEXT
  status: STATUS enum [active,suspended]
  visibility: VISIBILITY enum [public,private,unlisted]
  is_deleted: BOOLEAN
  is_locked: BOOLEAN
  is_featured: BOOLEAN
  is_bot: BOOLEAN
  is_sponsor: BOOLEAN
  created_at: TIMESTAMPZ
  updated_at: TIMESTAMPZ
}
```

## Authentication

When a user signs in, two tokens are generated for them:
the `auth-token` and the `session-token`.

The `auth-token` is a JWT token that is valid for 15 minutes.
This uniquely identifies the user and allows actions to be taken
on the user's behalf.

The `session-token` is a JWT token that is valid for 7 days, it is
tied to a "session" in the database and can be used to generate
new `auth-tokens`. When this generation is done, the `session`
is checked against the database. If the session has been nullified
by expiration or manual action.

When a user signs out, the session is expired and deleted from the database.

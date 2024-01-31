# Contributing

## Running Integration Tests

## Building the Backend

Before a build can be done for the backend, the command `cargo sqlx prepare`
must be run to create a document that is used for "offline" building. Make sure
the version of the `sqlx-cli` matches the version in `Cargo.toml`, otherwise
the process will fail to find any queries.

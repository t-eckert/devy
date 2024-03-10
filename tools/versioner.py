""" Versioner

Called by `make version`

1. Sets the version of the project following yyyy.mm.dd format.
2. Updates a `version` file at the root of the project.
3. Creates a new file in the "CHANGELOG" directory with the version as the name
4. Opens that file for editing.
5. Commits the version change to git.
"""

from datetime import datetime
from pathlib import Path

import tomllib
import json
import os

package_paths = [
    Path("./site/package.json"),
    Path("./crates/api/Cargo.toml"),
    Path("./crates/auth/Cargo.toml"),
    Path("./crates/db/Cargo.toml"),
    Path("./crates/entities/Cargo.toml"),
    Path("./crates/forms/Cargo.toml"),
    Path("./crates/router/Cargo.toml"),
    Path("./crates/store/Cargo.toml"),
    Path("./crates/upload/Cargo.toml"),
]


def get_current_version() -> str:
    # Gets the current version from the version file at the root of the project
    with open("version", "r") as f:
        return f.read()


def gen_version() -> str:
    # Generates a version in the form yyyy.mm.dd.v from the current date.
    cur = get_current_version()
    if not cur:
        return datetime.now().strftime("%Y.%m.%d")

    if cur[:10] == datetime.now().strftime("%Y.%m.%d"):
        desc = 1 if len(cur) == 10 else int(cur[11:]) + 1
        return f"{cur[:10]}.{desc}"

    return datetime.now().strftime("%Y.%m.%d")


def get_name(version: str) -> str:
    return input(f"{version} Name: ")


def slug(s: str) -> str:
    return (
        s.lower()
        .replace(" ", "-")
        .replace(".", "-")
        .replace("/", "-")
        .replace("_", "-")
        .replace(":", "-")
    )


def write_version(version: str) -> None:
    # Writes the version to a version file at the root of the project.
    with open("version", "w") as f:
        f.write(version)


def get_packages(paths: list[Path]) -> dict[str, str]:
    # Gets the packages and their versions from the given paths.
    packages = {}
    for path in paths:
        if path.match("*.json"):
            with open(path, "rb") as f:
                package = json.load(f)
            packages[package["name"]] = package["version"]
        if path.match("*.toml"):
            with open(path, "rb") as f:
                cargo = tomllib.load(f)
            packages[cargo["package"]["name"]] = cargo["package"]["version"]
    return packages


def format_changelog(version: str, name: str, packages: dict[str, str]) -> str:
    # Formats the changelog for a given version.

    package_list = "\n".join(
        [f"* `{name}`: `v{version}`" for name, version in packages.items()]
    )

    return f"""# `v{version}` {name}

## Release Notes

## Packages

{package_list}
"""


def write_changelog(changelog: str, filename: str) -> None:
    # Writes the version to a changelog file in the changelog directory.
    with open(filename, "w") as f:
        f.write(changelog)


def tag_version(version: str, message: str) -> None:
    os.execlp("git", "git", "tag", version, "-m", message)


def open_changelog(filename: str) -> None:
    os.execlp("$EDITOR", "$EDITOR", filename)


def commit(version: str) -> None:
    os.execlp("git", "git", "commit", "-am", f"version: {version}")


if __name__ == "__main__":
    version = gen_version()
    write_version(version)

    name = get_name(version)
    filename = f"changelog/{version}.md"
    write_changelog(
        format_changelog(version, name, get_packages(package_paths)), filename
    )
    tag_version(version, name)
    open_changelog(filename)
    commit(version)

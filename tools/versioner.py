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
import subprocess
import json
import os

package_paths = [
    Path("./site/package.json"),
    Path("./api/Cargo.toml"),
    Path("./lib/Cargo.toml"),
    Path("./db/Cargo.toml"),
    Path("./devyctl/Cargo.toml"),
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


def read_version() -> str:
    with open("version", "r") as f:
        return f.read()


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


def format_changelog(
    version: str, name: str, changes: str, packages: dict[str, str]
) -> str:
    # Formats the changelog for a given version.

    package_list = "\n".join(
        [f"* `{name}`: `v{version}`" for name, version in packages.items()]
    )

    return f"""# `v{version}` {name}

## Release Notes

## Changes

{changes}

## Packages

{package_list}
"""


def write_changelog(changelog: str, filename: str) -> None:
    # Writes the version to a changelog file in the changelog directory.
    with open(filename, "w") as f:
        f.write(changelog)


def tag_version(version: str, message: str) -> None:
    os.execlp("git", "git", "tag", version, "-m", message)


def commit(version: str) -> None:
    os.execlp("git", "git", "commit", "-am", f"version: {version}")


def changes(previous: str) -> str:
    command = ["git", "hist", previous + "..HEAD"]

    result = subprocess.run(command, text=True, capture_output=True)

    if result.returncode == 0:
        return result.stdout
    else:
        return result.stderr


if __name__ == "__main__":
    prev_version = read_version()
    version = gen_version()
    write_version(version)

    name = get_name(version)
    filename = f"changelog/{version}.md"
    write_changelog(
        format_changelog(
            version, name, changes(prev_version), get_packages(package_paths)
        ),
        filename,
    )
    tag_version(version, name)
    commit(version)

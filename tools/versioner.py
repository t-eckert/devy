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
import glob


CHANGELOG_DIR = "./frontend/static/changelog"


def main():
    prev_version = read_version()

    version = gen_version()
    write_version(version)
    filename = f"{CHANGELOG_DIR}/{version}.md"

    name = get_name(version)

    write_changelog(
        format_changelog(version, name, get_git_changes(prev_version)),
        filename,
    )
    update_index()

    tag_version(version, name)
    commit(version)


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


def read_version() -> str:
    with open("version", "r") as f:
        return f.read()


def write_version(version: str) -> None:
    # Writes the version to a version file at the root of the project.
    with open("version", "w") as f:
        f.write(version)


def format_changelog(version: str, name: str, changes: str) -> str:
    # Formats the changelog for a given version.

    return f"""## {name}

### Changes

{changes}
"""


def write_changelog(changelog: str, filename: str) -> None:
    # Writes the version to a changelog file in the changelog directory.
    with open(filename, "w") as f:
        f.write(changelog)


def tag_version(version: str, message: str) -> None:
    os.execlp("git", "git", "tag", version, "-m", message)


def commit(version: str) -> None:
    os.execlp("git", "git", "commit", "-am", f"version: {version}")


def get_git_changes(previous: str) -> str:
    command = ["git", "hist", previous + "..HEAD"]

    result = subprocess.run(command, text=True, capture_output=True)

    if result.returncode != 0:
        raise Exception(result.stderr)

    result.stdout.splitlines()

    return format_changes(result.stdout)


def format_changes(changes: str) -> str:
    return "\n".join([format_change(change) for change in changes.splitlines()])


def format_change(change: str) -> str:
    return "- " + change.split("|")[-1].strip().split("{{")[0].strip()


def update_index():
    files = glob.glob(f"{CHANGELOG_DIR}/*.md")
    index = [x[len(CHANGELOG_DIR)+1:-3] for x in files]
    with open(f"{CHANGELOG_DIR}/index.json", "w") as w:
        w.write(json.dumps(sorted(index)[::-1]))


if __name__ == "__main__":
    main()

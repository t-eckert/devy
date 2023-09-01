""" Versioner

Called by `make version`

Updates the version number across all components of the project.
The version number is the current date in the format YYYY.MM.DD
"""

import json
import tomllib
import toml

def get_version_and_name() -> tuple[str, str]:
    version = input("Version: ")
    name = input("Name: ")

    return (version, name)


def version_frontend(version: str):
    package_json = "./frontend/package.json"

    with open(package_json, "r") as file:
        package = json.load(file)

    package["version"] = version

    with open(package_json, "w") as file:
        json.dump(package, file, indent=2)


def version_backend(version: str):
    cargo_toml = "./backend/Cargo.toml"

    with open(cargo_toml, "rb") as file:
        cargo = tomllib.load(file)

    cargo["package"]["version"] = version

    with open(cargo_toml, "w") as file:
        toml.dump(cargo, file)

if __name__ == "__main__":
    version, name = get_version_and_name()

    print(f"Setting version {version}: {name}")

    version_frontend(version)
    version_backend(version)

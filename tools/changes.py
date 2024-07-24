from rich import print

import subprocess


def main():
    previous = read_version()
    print(f"Changes since {previous}:")
    print(changes(previous))


def read_version() -> str:
    with open("version", "r") as f:
        return f.read()


def changes(previous: str) -> str:
    command = ["git", "hist", previous + "..HEAD"]

    result = subprocess.run(command, text=True, capture_output=True)

    if result.returncode == 0:
        return result.stdout
    else:
        return result.stderr


if __name__ == "__main__":
    main()

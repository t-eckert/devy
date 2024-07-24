"""This script runs the integration tests by building the Docker images and running them through Docker Compose."""

import subprocess

if __name__ == "__main__":

    # Build the Docker images
    subprocess.run(["docker-compose", "build"])

    # Run the Docker containers
    subprocess.run(["docker-compose", "up", "-d"])

    print("Now is when you should run the tests")

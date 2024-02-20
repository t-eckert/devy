"""This script runs the integration tests by building the Docker images and running them through Docker Compose."""

from time import sleep
import subprocess

if __name__ == "__main__":

    # Build the Docker images
    subprocess.run(["docker-compose", "build"])

    # Run the Docker containers
    subprocess.run(["docker-compose", "up", "-d"])

    sleep(5)

    # Run tests
    subprocess.run(["pytest"])

    # Clean up the Docker containers
    subprocess.run(["docker-compose", "down"])

# Integration Tests

These tests exercise the happy path of behaviors on the site, leveraging the UI
and the API. They are run with Pytest. They are written to be run against a
seeded database.

To run them, you will need:

- Python
- Node
- Cargo
- Docker

Create a virtual environment.

```shell
python -m venv .venv
```

Start that virtual environment.

```shell
source .venv/bin/activate
```

Install dependencies.

```shell
pip install -r requirements.txt
```

Run tests.

```shell
pytest
```

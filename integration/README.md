# Integration Tests

These tests help ensure the frontend, backend, and database all integrate together correctly.

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

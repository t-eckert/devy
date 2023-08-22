import psycopg
import os

from psycopg.abc import Query

connection_string = os.getenv("DATABASE_URL") or ""
if connection_string is "" or connection_string is None:
    raise Exception("DATABASE_URL is not set.")

def generate_seed() -> Query:
    return ""

def execute_seed(seed: Query):
    with psycopg.connect(connection_string) as conn:
        with conn.cursor() as cur:
            cur.execute(seed)

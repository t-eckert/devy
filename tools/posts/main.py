# pyright: reportGeneralTypeIssues=false

from rich import print

import openai
import uuid
import os
import sys

openai.api_key = os.getenv("OPENAI_API_KEY")
if openai.api_key is None:
    raise Exception("OPENAI_API_KEY is not set.")

def chat(prompt, model="gpt-3.5-turbo"):
    completion = openai.ChatCompletion.create(
        model=model,
        messages=[{"role": "user", "content": prompt}]
    )

    return completion.choices[0].message.content


if __name__ == "__main__":
    topic = sys.argv[1]

    id = uuid.uuid4()

    query = f"Give me a blog post formatted in markdown about {topic}."
    print(query)
    message = chat(query)
    print(message)
    with open(f"./generated/{id}.md", "w") as f:
        f.write(message)

    query = f"Create a list of tags for this blog post, formatted in JSON: {message}"
    print(query)
    message = chat(query)
    print(message)

    with open(f"./generated/{id}.tags.json", "w") as f:
        f.write(message)


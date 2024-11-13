# Seeding Data

It's time to increase the size of my seed data for local testing. I need more variability and a
better way to grow the data over time. Most of the data in Devy is hierarchical. I can build
out the seed data with that structure as big JSON objects and then parse that into SQL.

```json
{
  "user": {"username": "t-swizzle", "email": "t-swiz@email.com"},
  "profile": {
    "name": "Taylor Schwift",
    "description": "Loving him was red, but my pipelines are green.",
    "githubUsername": "t-swizzle"
  },
  "blogs": [{
    "name": "Code Corner",
    "slug": "code-corner"
    "repo": "https://github.com/t-swizzle/code-corner"
    "posts": [
     {

     },
    ]
  }],
}
```

etc.

I want to build something that creates this data using OpenAI then translating that
into raw SQL that can be loaded into the Docker image.

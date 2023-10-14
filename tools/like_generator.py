""" Like Generator

Iterates over post ids and profile ids to randomly have profiles "like" posts.
"""

import random
import re

def like_generator(posts, profiles):
    for post in posts:
        likeable = random.random()
        for profile in profiles:
            liking = random.random()
            if liking > likeable:
                yield (post, profile)


def parse_posts(sql: str):
    post_id_pattern = re.compile(r"\s\(\'(.+?)\',")

    with open(sql, "r") as f:
        lines = f.readlines()
        for line in lines:
            match = re.search(post_id_pattern, line)
            if match:
                yield match.group(1)


def parse_profiles(sql: str):
    post_id_pattern = re.compile(r"\(\'(.+?)\',")

    with open(sql, "r") as f:
        lines = f.readlines()
        for line in lines:
            match = re.search(post_id_pattern, line)
            if match:
                yield match.group(1)

def format_likes(likes) -> str:
    formatted = 'INSERT INTO "like" (post_id, profile_id) VALUES\n'
    for i, like in enumerate(likes):
        formatted += f"\t('{like[0]}', '{like[1]}'){',' if len(likes)-1 != i else ';'}\n"
    return formatted


if __name__ == "__main__":
    posts = [post for post in parse_posts("./seed/posts.sql")]
    profiles = [post for post in parse_profiles("./seed/profiles.sql")]
    likes = [like for like in like_generator(posts, profiles)]
    print(format_likes(likes))

""" Like Generator

Iterates over post ids and profile ids to randomly have profiles "like" posts.
"""

import random
import re

def like_generator(profiles, posts):
    for profile in profiles:
        for post in posts:
            if random.randint(0, 1) == 1:
                yield (profile, post)



def parse_profiles(sql: str):
    post_id_pattern = re.compile("\(\'(.+?)\',")

    with open(sql, "r") as f:
        lines = f.readlines()
        for line in lines:
            match = re.search(post_id_pattern, line)
            if match:
                yield match.group(1)

def parse_posts(sql: str):
    post_id_pattern = re.compile("\(\'(.+?)\',")

    with open(sql, "r") as f:
        lines = f.readlines()
        for line in lines:
            match = re.search(post_id_pattern, line)
            if match:
                yield match.group(1)

def format_likes(likes) -> str:
    formatted = ""
    for like in likes:
        formatted += str(like) + "\n"
    return formatted

if __name__ == "__main__":
    for profile in parse_posts("./seed/posts.sql"):
        print(profile)
    # print(format_likes(like_generator(parse_posts("./seed/posts.sql"), parse_profiles("./seed/profiles.sql"))))

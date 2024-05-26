from rich import print


def is_subset(a: dict, b: dict) -> bool:
    # Iterate through keys in a and check key/value equality in b,
    # ignoring keys not in a.
    return all(a[k] == b[k] for k in a if k in b)


def tee(a):
    print(a)
    return a

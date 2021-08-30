#!/usr/bin/env python3

def iter_generate_route(_from: tuple, _to: tuple) -> list:
    if _from[1] == _to[1]:
        for r in range(_to[0] - _from[0], _from[0]):
            yield r, _from[1]

    elif _from[0] == _to[0]:
        for r in range(_to[1] - _from[1], _from[1]):
            yield _from[0], r


def main():
    items = list(iter_generate_route((5, 10), (20, 10)))
    print(", ".join(items))


if __name__ == '__main__':
    main()

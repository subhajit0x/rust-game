#!/usr/bin/env python3
import json
import random
from typing import Union

from PIL import Image
from enum import Enum

PATH = "./src/resources/map.png"
WIDTH = 64
HEIGHT = 32


class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3


def index_of_max(a: tuple):
    largest = max(a)
    return a.index(largest)


def get_color(pixel):
    max_index = index_of_max(pixel)
    if max_index == 0:
        color = Color.RED
    elif max_index == 1:
        color = Color.GREEN
    elif max_index == 2:
        color = Color.BLUE
    else:
        raise ValueError('')

    return color


tiles = {
    "grass": {
        "default": [70]
    },
    "road": {
        "default": [71],
        "grass": {
            "up": 6,
            "down": 44,
            "left": 14,
            "right": 48,
            "corner": {
                "up_right": 33,
                "up_left": 19,
                "down_right": 62,
                "down_left": 57,
            },
        }
    },
}


class PixelHelpers:
    def __init__(self, loaded_image):
        self.loaded_image = loaded_image

    def get_pixel(self, x, y):
        try:
            pixel = self.loaded_image[x, y]
        except IndexError:
            pixel = None

        return pixel

    def down(self, x, y) -> Union[tuple, None]:
        return self.get_pixel(x, y + 1)

    def up(self, x, y) -> Union[tuple, None]:
        return self.get_pixel(x, y - 1)

    def left(self, x, y) -> Union[tuple, None]:
        return self.get_pixel(x - 1, y)

    def right(self, x, y) -> Union[tuple, None]:
        return self.get_pixel(x + 1, y)


def get_sprite(center, left, right, up, down):
    sprite = ''

    if get_color(center) == Color.RED:
        sprite = random.choice(tiles["road"]["default"])
        # road
        if up and get_color(up) == Color.GREEN:
            sprite = tiles["road"]["grass"]["up"]
            if right and get_color(right) == Color.GREEN:
                sprite = tiles["road"]["grass"]["corner"]["up_right"]
            elif left and get_color(left) == Color.GREEN:
                sprite = tiles["road"]["grass"]["corner"]["up_left"]
        elif down and get_color(down) == Color.GREEN:
            sprite = tiles["road"]["grass"]["down"]
            if right and get_color(right) == Color.GREEN:
                sprite = tiles["road"]["grass"]["corner"]["down_right"]
            elif left and get_color(left) == Color.GREEN:
                sprite = tiles["road"]["grass"]["corner"]["down_left"]
        elif left and get_color(left) == Color.GREEN:
            sprite = tiles["road"]["grass"]["left"]
        elif right and get_color(right) == Color.GREEN:
            sprite = tiles["road"]["grass"]["right"]
    elif get_color(center) == Color.GREEN:
        sprite = random.choice(tiles["grass"]["default"])
    return sprite


def iter_generate_map(loaded_image):
    pixel_helpers = PixelHelpers(loaded_image)

    for y in range(HEIGHT):
        for x in range(WIDTH):
            center = loaded_image[x, y]
            up = pixel_helpers.up(x, y)
            down = pixel_helpers.down(x, y)
            left = pixel_helpers.left(x, y)
            right = pixel_helpers.right(x, y)

            center_color = get_color(center)
            sprite = get_sprite(
                center=center,
                up=up,
                down=down,
                left=left,
                right=right,
            )
            chance = random.uniform(0, 1)
            decor = None

            if 0.95 <= chance < 0.96:
                decor = 101
            if 0.96 <= chance < 0.97:
                decor = 102
            if 0.97 <= chance < 0.98:
                decor = 103
            if 0.98 <= chance < 0.99:
                decor = 104
            if 0.99 <= chance <= 1:
                decor = 105

            yield {
                "pos_x": x,
                "pos_y": y,
                "color": center_color,
                "sprite": sprite,
                "decor": center_color == Color.GREEN and decor or None,
            }


def load_image(path: str):
    return Image.open(path).convert("RGB").load()


def main():
    loaded_image = load_image(PATH)
    map_dict = {}
    for tile in iter_generate_map(loaded_image):
        pos_x = tile["pos_x"]
        pos_y = tile["pos_y"]
        map_dict.update({f"{pos_x}_{pos_y}": {
            "sprite": tile["sprite"],
            "decor": tile["decor"],
        }})

    print(json.dumps(map_dict, indent=2))


if __name__ == '__main__':
    main()

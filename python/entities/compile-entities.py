# this script takes the assets in ./assets/ and builds rust code automatically matching the schema they are named from

from os import listdir
from os.path import isfile, join
import struct

from PIL import Image

asset_dir = "./assets/"
assets = [x for x in listdir(asset_dir) if isfile(join(asset_dir, x))]
assets = [x for x in assets if x != "naming-schema"]


def unpack_filename(name: str):
    filename = name.split(".")[0]
    parts = filename.split("-")

    if len(parts) == 2:
        parts.append(None)  # pyright: ignore

    entity, variant, step = parts
    if step is not None:
        step = int(step)

    return entity, variant, step


def unpack_pixels(file: str):
    image = Image.open(join(asset_dir, file))
    image = image.convert("RGBA")

    width, height = image.size
    pixel_count = width * height

    bytes = image.tobytes()
    u32_array = list(struct.unpack(f">{pixel_count}I", bytes))

    return width, height, u32_array


entities = set()

statics = {}
# {
#   "rabbit": {
#       0: number{},
#   }
# }

motions = {}
# {
#   "rabbit": {
#       2: number{}{},
#   }
# }

dimensions = {}
# {
#   "rabbit": (18, 23)
# }

for filename in assets:
    entity, variant, step = unpack_filename(filename)
    width, height, sprite = unpack_pixels(filename)

    if entity not in entities:
        entities.add(entity)

        statics[entity] = {}
        motions[entity] = {}

        dimensions[entity] = width, height

    if dimensions[entity] != (width, height):
        print(
            f"mismatching dimensions on {entity}: wanted {dimensions[entity]}, got {width, height}"
        )
        break

    if step is None:
        statics[entity][variant] = sprite

    else:
        if motions[entity].get(variant) is None:
            motions[entity][variant] = {}

        motions[entity][variant][step] = sprite


rust_entities_path = "./../../rust-wasm/src/entities/compiled_entities.rs"


def init_entity_str(width, height, sprite):
    return (
        f"EntitySprite {'{'} width: {width}, height: {height}, sprite: [{sprite}] {'}'}"
    )


with open(rust_entities_path, "w") as file:
    for entity in entities:
        stats, mots = statics[entity], motions[entity]
        width, height = dimensions[entity]

        file.write("""

""")

        for variant in stats:
            stat = stats[variant]
            file.write()

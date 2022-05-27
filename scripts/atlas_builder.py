#!/usr/bin/env python3 

import io
import os
import sys

from PIL import Image
import glob

OUT_IMG_PATH = "resources/atlas.png"
OUT_CONFIG_PATH = "resources/atlasConfig.bin"
CONFIG_PATH = "scripts/config.txt"
IMGS_PATH = "assets/textures"

config: "dict[str, int]" = {}


class Img:
    def __init__(self, img: Image.Image, _id):
        self.pos = (0, 0)
        self.img = img
        self.id = _id


class ImgWrapper:
    def __init__(self, img: Image.Image, imgs: [Img]):
        self.img = img
        self.imgs = imgs


images: [Img] = []


def loadConfig():
    global configMaxId
    file = io.open(CONFIG_PATH)
    for line in file:
        _id = int(line.split("=")[1])
        config[line.split("=")[0]] = _id


def loadImages():
    global images
    files = glob.glob(IMGS_PATH + "/*.png")
    files = filter(lambda x: config.get(os.path.splitext(os.path.basename(x))[0]) is not None, files)
    images = list(map(lambda x: Img(Image.open(x).convert("RGBA"), config[os.path.splitext(os.path.basename(x))[0]]),files))
                      


def sortImages():
    images.sort(key=lambda x: x.img.size)
    for i in range(len(images)):
        assert images[i].img.height == images[i].img.width


def movImgWrapper(img: ImgWrapper, x: int, y: int):
    for i in img.imgs:
        if isinstance(i, Img):
            i.pos = (i.pos[0] + x, i.pos[1] + y)
        else:
            assert isinstance(i, ImgWrapper)
            movImgWrapper(i, x, y)


def outPos(arr: [], img: ImgWrapper):
    for i in img.imgs:
        if isinstance(i, Img):
            print(i.id, i.pos, i.img.size)
            arr.append((i.id, i.pos[0], i.pos[1], i.img.size[0], i.img.size[1]))
        else:
            assert isinstance(i, ImgWrapper)
            outPos(arr, i)


def main():
    loadConfig()
    loadImages()
    sortImages()


    

    while len(images) > 1:
        size = images[0].img.height
        newImg = Image.new(mode="RGBA", size=(images[0].img.width * 2, images[0].img.height * 2), color="black")
        imgs = []
        x, y = 0, 0
        for i in range(4):
            if len(images) == 0:
                break
            if isinstance(images[0], Img):
                if images[0].img.height != size:
                    break
                newImg.paste(images[0].img, (x, y, x + size, y + size))
                images[0].pos = (images[0].pos[0] + x, images[0].pos[1] + y)
                if i % 2 == 0:
                    x += size
                else:
                    x = 0
                    y += size
                imgs.append(images.pop(0))
            else:
                assert isinstance(images[0], ImgWrapper)
                if images[0].img.height != size:
                    break
                newImg.paste(images[0].img, (x, y, x + size, y + size))
                movImgWrapper(images[0], x, y)

                if i % 2 == 0:
                    x += size
                else:
                    x = 0
                    y += size
                imgs.append(images.pop(0))

        images.append(ImgWrapper(newImg, imgs))
        sortImages()

    assert len(images) == 1
    os.makedirs(os.path.dirname(OUT_IMG_PATH), exist_ok=True)
    images[0].img.save(OUT_IMG_PATH)

    arr = []
    outPos(arr, images[0])
    arr.sort(key=lambda x: x[0])

    os.makedirs(os.path.dirname(OUT_CONFIG_PATH), exist_ok=True)

    file = io.open(OUT_CONFIG_PATH, mode="wb")
    x = -1
    if arr[0][0] != 0:
        print("Error: Tiles ID must start at 0")
        exit(-1)
    for e in arr:
        if e[0] != x + 1:
            print("Error: Tiles ID must be contiguous")
            exit(-1)
        x += 1
        for i in range(4):
            file.write(e[i + 1].to_bytes(4, byteorder=sys.byteorder))
    file.close()


if __name__ == '__main__':
    main()
# http://paulbourke.net/fractals/peterdejong/peterdejong.pdf
# http://paulbourke.net/fractals/peterdejong/

import numpy as np
from PIL import Image

def calulate(x, y, a, b, c, d):
    x_ = np.sin(a*y) - np.cos(b*x)
    y_ = np.sin(c*x) - np.cos(d*y)
    return (x_, y_)

def attract(iterations=1e7, x=0, y=0, point_size=4, a=1., b=1., c=1., d=1., save_location="/tmp/attractor.png", dimension=(1024, 1024)):
    canvas = np.zeros((*dimension, 3))

    width = dimension[0] - 1
    height = dimension[1] - 1
    for i in range(int(iterations)):
        x_pos = int((x+2)/4 * width)
        y_pos = int((y+2)/4 * height)
        canvas[x_pos:x_pos + point_size, y_pos:y_pos + point_size, 0] = 255.
        x, y = calulate(x, y, a, b, c, d)

    Image.fromarray(canvas.astype('uint8')).save(save_location)

        

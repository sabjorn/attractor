# http://paulbourke.net/fractals/peterdejong/peterdejong.pdf
# http://paulbourke.net/fractals/peterdejong/
"""
usage:
run 
    WIDTH=1024; HEIGHT=1024; 
    python3 strange_attractor.py -a 2 2 -b -2 2 -c 2 2 -d 2 2 -f 30 -i 1e4 -r $WIDTH $HEIGHT | ffmpeg -y -f rawvideo -vcodec rawvideo -s ${WIDTH}x${HEIGHT} -pix_fmt rgb24 -r 30 -i - -c:v libx264 -pix_fmt yuv420p -an /tmp/lol.mov
to generate video
"""
import sys
import argparse
import numpy as np

def calulate(x, y, a, b, c, d):
    x_ = np.sin(a*y) - np.cos(b*x)
    y_ = np.sin(c*x) - np.cos(d*y)
    return (x_, y_)

def attract(iterations=1e7, point_size=4, a=1., b=1., c=1., d=1., dimensions=[1024, 1024]):
    x = y = 0
    canvas = np.zeros((*dimensions, 3))
    width = dimensions[0] - 1
    height = dimensions[1] - 1
    
    for i in range(int(iterations)):
        x_pos = int((x+2)/4 * width)
        y_pos = int((y+2)/4 * height)
        canvas[x_pos:x_pos + point_size, y_pos:y_pos + point_size, 0] = 255.
        x, y = calulate(x, y, a, b, c, d)

    sys.stdout.buffer.write(canvas.astype('uint8').tostring())

        
if __name__ == '__main__':
    parser = argparse.ArgumentParser(
            description="generates an output stream of generated frames (rgb24 bytes) of strange attractor to stdout")
    parser.add_argument(
        "-f",
        "--frames",
        help="number of frames to generate",
        type=int,
        default=30)
    parser.add_argument(
        "-r",
        "--dimensions",
        help="width height of output frame",
        nargs=2,
        type=int,
        default=(512, 512))
    parser.add_argument(
        "-i",
        "--iterations",
        help="number of iterations, suggested range 1e4 to 1e5",
        type=float,
        default=1e4)
    parser.add_argument(
        "-a",
        "--arange",
        help="end points of 'a' variable range",
        nargs=2,
        type=float,
        default=(-1.,1.))
    parser.add_argument(
        "-b",
        "--brange",
        help="end points of 'b' variable range",
        nargs=2,
        type=float,
        default=(-1.,1.))
    parser.add_argument(
        "-c",
        "--crange",
        help="end points of 'c' variable range",
        nargs=2,
        type=float,
        default=(-1.,1.))
    parser.add_argument(
        "-d",
        "--drange",
        help="end points of 'd' variable range",
        nargs=2,
        type=float,
        default=(-1.,1.))
    args = parser.parse_args()

    frames = args.frames
    a = np.linspace(*args.arange, frames)
    b = np.linspace(*args.brange, frames)
    c = np.linspace(*args.crange, frames)
    d = np.linspace(*args.drange, frames)

    for i in range(frames):
        attract(a=a[i], b=b[i], c=c[i], d=d[i], iterations=args.iterations, point_size=1, dimensions=args.dimensions)

# Strange Attractor
Example code for generating a [strange attractor](https://en.wikipedia.org/wiki/Attractor#Strange_attractor) in Python and Rust.

The two versions were implemented to compare performance between the languages.

## Usage
Both implementations expect to write RGB 24-bit (8-bit per channel) to STDOUT. This allows for the output to be piped into other applications (e.g. `ffmpeg`) or piped out to disk.

example:

```
python3 strange_attractor.py -a 2 2 -b -2 2 -c 2 2 -d 2 2 -f 30 -i 1e4 -r $WIDTH $HEIGHT | ffmpeg -y -f rawvideo -vcodec rawvideo -s ${WIDTH}x${HEIGHT} -pix_fmt rgb24 -r 30 -i - -c:v libx264 -pix_fmt yuv420p -an /tmp/lol.mov
```

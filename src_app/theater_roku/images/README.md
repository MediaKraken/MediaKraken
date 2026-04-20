# Channel art

The Roku manifest and `HomeScene` reference the following PNGs. Drop matching
files in this directory before building a release sideload.

## Manifest art

| File                              | Recommended size |
| --------------------------------- | ---------------- |
| `home-tile-mm-hd.png`             | 290 x 218 px     |
| `home-tile-mm-sd.png`             | 214 x 144 px     |
| `side-tile-mm-hd.png`             | 108 x 69  px     |
| `side-tile-mm-sd.png`             | 80  x 46  px     |
| `splash-mm-hd.png`                | 1280 x 720 px    |
| `splash-mm-sd.png`                | 720  x 480 px    |

## Category tile art

Each tile is rendered at 180 x 160 px (`scaleToFit`) so square 320 x 320 PNGs
work well.

- `cat-movie.png`
- `cat-music.png`
- `cat-tv.png`
- `cat-sports.png`
- `cat-livetv.png`
- `cat-photo.png`
- `cat-games.png`
- `cat-books.png`
- `cat-radio.png`
- `cat-homemovie.png`
- `cat-3d.png`
- `cat-internet.png`

The Tizen client (`src_app/theater_tizen/static/image/`) ships with
visually-equivalent icons that can be re-cropped for Roku.

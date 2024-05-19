Parse ansi strings and convert them to html and svg formats

```bash
neofetch | ansi2 --format=svg --theme=vscode > neofetch.svg

npm run bench:run | ansi2 --format=svg | resvg - -c > bench.png
```

## [ansi2](./ansi2)

```rs
use ansi2::{Canvas};

let canvas = Canvas::new(s);
for row in canvas.pixels.iter() {
  for pixel in row.iter() {
      // draw pixel
  }
}
```

## 16colo
https://16colo.rs/pack/laz17/ll-darlaakacrystal.ans
```bash
cat ./ll-darlaakacrystal.ans | ansi2 --format=svg --width=80 > ll-darlaakacrystal.svg

```
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

## [ansi2-wasm](./ansi2-wasm)
```
npm i ansi2 -g

neofetch | ansi2 --format=svg --theme=vscode > ./neofetch.svg

```


## [ansi2html](./ansi2html)
```
cargo install ansi2html

neofetch | ansi2html > neofetch.html

```

## [ansi2svg](./ansi2svg)

```
cargo install ansi2svg

neofetch | ansi2svg > neofetch.svg
```

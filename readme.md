Parse ansi strings and convert them to html and svg formats

```bash
cargo install ansi2
cargo binstall ansi2

neofetch | ansi2 --format=svg --theme=vscode > neofetch.svg

npm run bench:run | ansi2 --format=svg | resvg - -c > bench.png
npm run bench:run | ansi2 --format=html | resvg - -c > bench.html
npm run bench:run | ansi2 --format=text | resvg - -c > bench.text
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

## [ansi2-wasm](./ansi2-wasm)
```
npm i ansi2 -g

neofetch | ansi2 --format=svg --theme=vscode > ./neofetch.svg

```


## html
```
neofetch | ansi2 --format=html > neofetch.html

```

## svg

```
neofetch | ansi2 --format=svg > neofetch.svg
```

## theme
vga / vscode / ubuntu
```
neofetch | ansi2 --format=svg --theme=vscode > neofetch.svg
```

## example
### neofetch

<div align="center">
	<a href="https://github.com/ahaoboy/neofetch">
		<img src="assets/win11.svg">
	</a>
</div>

### vitest
<div align="center">
	<a href="https://github.com/ahaoboy/ansi2">
		<img src="assets/vitest.svg">
	</a>
</div>
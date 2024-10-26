#!/bin/bash

for i in win11 vitest 8bit-color 24bit-color nu-ls ansi-default colortable hyperlink-demo
do
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js -c > "$i.svg"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=light -c > "$i-light.svg"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=dark -c > "$i-dark.svg"

  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js -c -f=html > "$i.html"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=light -c -f=html  > "$i-light.html"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=dark -c -f=html > "$i-dark.html"
  echo "$i done"
done

cat "nu-ls.ans" | node ../ansi2-wasm/bin/cli.js -c --length-adjust=spacingAndGlyphs > "nu-ls.fix.svg"

cat "take-my-ansi.utf8.ans" | node ../ansi2-wasm/bin/cli.js -c -w=80 > "take-my-ansi.utf8.svg"
cat "take-my-ansi.utf8.ans" | node ../ansi2-wasm/bin/cli.js -c -w=80 -f=html > "take-my-ansi.utf8.html"

#!/bin/bash

for i in win11 vitest 8bit-color 24bit-color nu-ls ansi-default colortable hyperlink-demo hyperfine
do
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js > "$i.svg"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js -f=ans > "$i.min.ans"

  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=light > "$i-light.svg"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=dark > "$i-dark.svg"

  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js -f=html > "$i.html"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=light -f=html  > "$i-light.html"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=dark -f=html > "$i-dark.html"
  echo "$i done"
done

cat "nu-ls.ans" | node ../ansi2-wasm/bin/cli.js --length-adjust=spacingAndGlyphs > "nu-ls.fix.svg"

cat "take-my-ansi.utf8.ans" | node ../ansi2-wasm/bin/cli.js -w=80 > "take-my-ansi.utf8.svg"
cat "take-my-ansi.utf8.ans" | node ../ansi2-wasm/bin/cli.js -w=80 -f=html > "take-my-ansi.utf8.html"
cat "take-my-ansi.utf8.ans" | node ../ansi2-wasm/bin/cli.js -w=80 -f=ans > "take-my-ansi.utf8.min.ans"

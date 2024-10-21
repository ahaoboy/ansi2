#!/bin/bash

for i in win11 vitest 8bit-color 24bit-color nu-ls ansi-default sample colortable hyperlink-demo
do
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js -c > "$i.svg"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=light -c > "$i-light.svg"
  cat "$i.ans" | node ../ansi2-wasm/bin/cli.js --mode=dark -c > "$i-dark.svg"
  echo "$i done"
done
#!/bin/bash

for i in win11 vitest
do
  cat "$i.ansi" | node ../ansi2-wasm/bin/cli.js > "$i.svg"
  cat "$i.ansi" | node ../ansi2-wasm/bin/cli.js --mode=light > "$i-light.svg"
  cat "$i.ansi" | node ../ansi2-wasm/bin/cli.js --mode=dark  > "$i-dark.svg"
done
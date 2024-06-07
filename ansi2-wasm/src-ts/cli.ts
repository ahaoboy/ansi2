import { program, } from 'commander'
import { to_svg, to_html } from './wasm'
import { readFileSync } from 'node:fs'

async function readToString() {
  return new Promise<string>((resolve) => {
    const { stdin } = process
    const v: string[] = []
    stdin.on('data', (data: Buffer) => {
      v.push(data.toString())
    });

    stdin.on('close', () => {
      resolve(v.join(''))
    });
  })
}


function getBase64(p: string) {
  const buf = readFileSync(p)
  return buf.toString('base64')
}

async function main() {
  const a = await readToString()

  program
    .option("--format [type]", "output format", "svg")
    .option("--theme [type]", "color theme", "vscode")
    .option("--width [type]", "width", undefined)
    .option("--font [type]", "font", undefined)

  program.parse();

  const options = program.opts();
  const theme = options.theme ?? "vscode";
  const format = options.format ?? "svg";
  const width = options.width ?? +options.width;
  const font = options.font ?? getBase64(options.font)
  switch (format) {
    case "svg": {
      console.log(to_svg(a, theme, width, font))
      break
    }
    case "html": {
      console.log(to_html(a, theme, width, font))
      break
    }
  }
}

main()
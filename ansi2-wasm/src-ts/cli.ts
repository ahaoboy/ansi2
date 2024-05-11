import { program, } from 'commander'
import { to_svg, to_html } from './wasm'

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



async function main() {
  const a = await readToString()

  program
    .option("--format [type]", "output format", "svg")
    .option("--theme [type]", "color theme", "vscode")

  program.parse();

  const options = program.opts();
  const theme = options.theme ?? "vscode";
  const format = options.format ?? "svg";
  switch (format) {
    case "svg": {
      console.log(to_svg(a, theme))
      break
    }
    case "html": {
      console.log(to_html(a, theme))
      break
    }
  }
}

main()
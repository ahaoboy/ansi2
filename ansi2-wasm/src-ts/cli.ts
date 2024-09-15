import { program, } from 'commander'
import { to_svg, to_html, to_text, Theme } from './wasm'
import { readFileSync } from 'node:fs'
import { optimize } from 'svgo';

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

function getTheme(s: string): Theme {
  switch (s.toLowerCase()) {
    case "vscode": return Theme.Vscode
    case "vga": return Theme.Vga
    case "ubuntu": return Theme.Ubuntu
    default: return Theme.Vscode
  }
}

async function main() {
  const input = await readToString()

  program
    .option("--format [type]", "output format", "svg")
    .option("--theme [type]", "color theme", "vscode")
    .option("--width [type]", "width", undefined)
    .option("--font [type]", "font", undefined)

  program.parse();

  const options = program.opts();
  const theme = getTheme(options.theme ?? "vscode");
  const format = options.format ?? "svg";
  const width = typeof options.width === 'undefined' ? undefined : +options.width;
  const font = typeof options.font === 'undefined' ? undefined : getBase64(options.font)
  switch (format) {
    case "svg": {
      const s = to_svg(input, theme, width, font)
      const result = optimize(s);
      process.stdout.write(result.data)
      break
    }
    case "html": {
      process.stdout.write(to_html(input, theme, width, font))
      break
    }
    case 'text': {
      process.stdout.write(to_text(input, width))
    }
  }
}

main()
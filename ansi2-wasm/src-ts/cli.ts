import { program } from "commander"
import { to_svg, to_html, to_text, Theme } from "./wasm"
import { readFileSync, existsSync } from "node:fs"
import { optimize } from "svgo"
import { Mode } from "./wasm"

async function readToString() {
  return new Promise<string>((resolve) => {
    const { stdin } = process
    const v: string[] = []
    stdin.on("data", (data: Buffer) => {
      v.push(data.toString())
    })

    stdin.on("close", () => {
      resolve(v.join(""))
    })
  })
}

function getFontUrl(p: string) {
  if (p.startsWith("http")) {
    return p
  }
  if (!existsSync(p)) {
    return p
  }
  const buf = readFileSync(p)
  const base64 = buf.toString("base64")
  return `data:font;base64,${base64}`
}

function getTheme(s: string): Theme {
  switch (s.toLowerCase()) {
    case "vscode":
      return Theme.Vscode
    case "vga":
      return Theme.Vga
    case "ubuntu":
      return Theme.Ubuntu
    default:
      return Theme.Vscode
  }
}

function getMode(s: string | undefined): Mode | undefined {
  switch (s?.toLowerCase()) {
    case "dark":
      return Mode.Dark
    case "light":
      return Mode.Light
    default:
      return undefined
  }
}

async function main() {
  const input = await readToString()

  program
    .option("-f, --format [type]", "output format", "svg")
    .option("-t, --theme [type]", "color theme", "vscode")
    .option("-w, --width [type]", "width", undefined)
    .option("--font [type]", "font", undefined)
    .option("-m, --mode [type]", "mode", undefined)
    .option("-c, --compress [type]", "compress", undefined)
    .option("--light-bg [type]", "light-bg", undefined)
    .option("--dark-bg [type]", "dark-bg", undefined)

  program.parse()

  const options = program.opts()
  const theme = getTheme(options.theme ?? "vscode")
  const mode = getMode(options.mode)
  const format = options.format ?? "svg"
  const width =
    typeof options.width === "undefined" ? undefined : +options.width
  const font =
    typeof options.font === "undefined" ? undefined : getFontUrl(options.font)

  const compress = options.compress === "undefined" ? false : options.compress

  switch (format) {
    case "svg": {
      const s = to_svg(
        input,
        theme,
        width,
        font,
        mode,
        options.lightBg,
        options.darkBg,
      )
      const result = compress ? optimize(s).data : s
      process.stdout.write(result)
      break
    }
    case "html": {
      process.stdout.write(
        to_html(
          input,
          theme,
          width,
          font,
          mode,
          options.lightBg,
          options.darkBg,
        ),
      )
      break
    }
    case "text": {
      process.stdout.write(to_text(input, width))
    }
  }
}

main()

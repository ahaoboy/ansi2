import { program } from "commander"
import { to_svg, to_html, to_text, Theme } from "./wasm"
import { readFileSync, existsSync } from "node:fs"
import { optimize } from "svgo"
import { Mode } from "./wasm"
import { version } from "../package.json"

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
  program
    .option("-f, --format [html, svg, text]", "output format", "svg")
    .option("-t, --theme [vscode, ubuntu, vga, xterm]", "color theme", "vscode")
    .option("-w, --width [number]", "width", undefined)
    .option("--font [string]", "font", undefined)
    .option("-m, --mode [dark, light]", "mode", undefined)
    .option("-c, --compress [bool]", "compress", undefined)
    .option("--light-bg [string eg.#FFFFFF]", "light-bg", undefined)
    .option("--dark-bg [string eg.#000000]", "dark-bg", undefined)
    .option("--font-size [number]", "font-size", undefined)
    .option(
      "--length-adjust [spacing|spacingAndGlyphs]",
      "length-adjust",
      undefined,
    )
    .version(version)

  program.parse()

  const input = await readToString()

  const options = program.opts()
  const theme = getTheme(options.theme ?? "vscode")
  const mode = getMode(options.mode)
  const format = options.format ?? "svg"
  const width =
    typeof options.width === "undefined" ? undefined : +options.width
  const font =
    typeof options.font === "undefined" ? undefined : getFontUrl(options.font)

  const compress = options.compress === "undefined" ? false : options.compress
  const fontSize = options.fontSize === "undefined" ? 16 : options.fontSize
  const lengthAdjust =
    options.lengthAdjust === "undefined" ? 16 : options.lengthAdjust

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
        fontSize,
        lengthAdjust,
      )
      const result = compress
        ? optimize(s, {
            plugins: [
              {
                name: "preset-default",
                params: {
                  overrides: {
                    inlineStyles: false,
                  },
                },
              },
            ],
          }).data
        : s
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
          fontSize,
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

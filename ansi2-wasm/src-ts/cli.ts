import { to_svg, to_html, to_text, Theme, to_ans } from "./wasm"
import { readFileSync, existsSync } from "node:fs"
import { Mode } from "./wasm"
import { version } from "../package.json"
import * as t from "typanion"
import { Command, Option, Cli, Builtins } from "clipanion"

const isInteger = t.cascade(t.isNumber(), [t.isInteger()])
const isFormat = t.cascade(t.isEnum(["html", "svg", "text", "ans"]))
const isTheme = t.cascade(t.isEnum(["vscode", "ubuntu", "vga", "xterm"]))
const isLengthAdjust = t.cascade(t.isEnum(["spacing", "spacingAndGlyphs"]))
const isColor = t.cascade(
  // @ts-ignore
  t.isHexColor({ alpha: true }),
)

class AnsiCmd extends Command {
  format = Option.String("-f,--format", "svg", {
    description: "output file format",
    validator: isFormat,
  })

  theme = Option.String("-t,--theme", "vscode", {
    description: "color theme",
    validator: isTheme,
  })

  width = Option.String("-w,--width", {
    description: "width",
    validator: isInteger,
    required: false,
  })

  font = Option.String("--font", {
    description: "font",
    required: false,
  })
  mode = Option.String("-m,--mode", {
    description: "mode",
    required: false,
  })
  compress = Option.Boolean("-c,--compress", false, {
    description: "compress",
  })
  lightBg = Option.String("--light-bg", {
    description: "light-bg",
    validator: isColor,
    required: false,
  })
  darkBg = Option.String("--dark-bg", {
    description: "dark-bg",
    validator: isColor,
    required: false,
  })
  fontSize = Option.String("--font-size", {
    description: "font-size",
    validator: isInteger,
    required: false,
  })
  lengthAdjust = Option.String("--length-adjust", {
    description: "length-adjust",
    validator: isLengthAdjust,
    required: false,
  })

  async execute() {
    const theme = getTheme(this.theme)
    const mode = getMode(this.mode)
    const format = this.format
    const width = this.width
    const font = getFontUrl(this.font)
    const compress = this.compress
    const fontSize = this.fontSize
    const lengthAdjust = this.lengthAdjust
    const lightBg = this.lightBg
    const darkBg = this.darkBg

    const input = await readToString()
    switch (format) {
      case "svg": {
        const s = to_svg(
          input,
          theme,
          width,
          font,
          mode,
          lightBg,
          darkBg,
          fontSize,
          lengthAdjust,
        )
        process.stdout.write(s)
        break
      }
      case "html": {
        process.stdout.write(
          to_html(input, theme, width, font, mode, lightBg, darkBg, fontSize),
        )
        break
      }
      case "text": {
        process.stdout.write(to_text(input, width))
      }
      case "ans": {
        process.stdout.write(to_ans(input, width, compress))
      }
    }
  }
}

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

function getFontUrl(p: undefined | string) {
  if (!p?.length) {
    return undefined
  }
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
  const cli = new Cli({
    binaryName: "ansi2",
    binaryVersion: version,
  })
  cli.register(AnsiCmd)
  cli.register(Builtins.VersionCommand)
  cli.runExit(process.argv.slice(2))
}

main()

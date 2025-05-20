/* tslint:disable */
/* eslint-disable */
export function to_svg(s: string, theme: Theme, width?: number | null, font?: string | null, mode?: Mode | null, light_bg?: string | null, dark_bg?: string | null, font_size?: number | null, length_adjust?: string | null, sourcemap?: boolean | null): string;
export function to_html(s: string, theme: Theme, width?: number | null, font?: string | null, mode?: Mode | null, light_bg?: string | null, dark_bg?: string | null, font_size?: number | null, sourcemap?: boolean | null): string;
export function to_text(s: string, width?: number | null): string;
export function to_ans(s: string, width?: number | null): string;
export enum Mode {
  Dark = 0,
  Light = 1,
}
export enum Theme {
  Vscode = 0,
  Ubuntu = 1,
  Vga = 2,
  Xterm = 3,
}


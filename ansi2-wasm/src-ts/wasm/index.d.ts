/* tslint:disable */
/* eslint-disable */
/**
 * @param {string} s
 * @param {Theme} theme
 * @param {number | undefined} [width]
 * @param {string | undefined} [font]
 * @param {Mode | undefined} [mode]
 * @param {string | undefined} [light_bg]
 * @param {string | undefined} [dark_bg]
 * @param {number | undefined} [font_size]
 * @param {string | undefined} [length_adjust]
 * @returns {string}
 */
export function to_svg(s: string, theme: Theme, width?: number, font?: string, mode?: Mode, light_bg?: string, dark_bg?: string, font_size?: number, length_adjust?: string): string;
/**
 * @param {string} s
 * @param {Theme} theme
 * @param {number | undefined} [width]
 * @param {string | undefined} [font]
 * @param {Mode | undefined} [mode]
 * @param {string | undefined} [light_bg]
 * @param {string | undefined} [dark_bg]
 * @param {number | undefined} [font_size]
 * @returns {string}
 */
export function to_html(s: string, theme: Theme, width?: number, font?: string, mode?: Mode, light_bg?: string, dark_bg?: string, font_size?: number): string;
/**
 * @param {string} s
 * @param {number | undefined} [width]
 * @returns {string}
 */
export function to_text(s: string, width?: number): string;
/**
 * @param {string} s
 * @param {number | undefined} [width]
 * @param {boolean | undefined} [compress]
 * @returns {string}
 */
export function to_ans(s: string, width?: number, compress?: boolean): string;
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


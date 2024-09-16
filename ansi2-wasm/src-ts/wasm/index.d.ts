/* tslint:disable */
/* eslint-disable */
/**
* @param {string} s
* @param {Theme} theme
* @param {number | undefined} [width]
* @param {string | undefined} [font]
* @param {Mode | undefined} [mode]
* @returns {string}
*/
export function to_svg(s: string, theme: Theme, width?: number, font?: string, mode?: Mode): string;
/**
* @param {string} s
* @param {Theme} theme
* @param {number | undefined} [width]
* @param {string | undefined} [font]
* @param {Mode | undefined} [mode]
* @returns {string}
*/
export function to_html(s: string, theme: Theme, width?: number, font?: string, mode?: Mode): string;
/**
* @param {string} s
* @param {number | undefined} [width]
* @returns {string}
*/
export function to_text(s: string, width?: number): string;
/**
*/
export enum Theme {
  Vscode = 0,
  Ubuntu = 1,
  Vga = 2,
}
/**
*/
export enum Mode {
  Dark = 0,
  Light = 1,
}


import * as vscode from "vscode"
import { to_svg, Theme } from "ansi2"
import * as fs from "node:fs"

let panel: vscode.WebviewPanel | undefined
let statusBarItem: vscode.StatusBarItem | undefined

/**
 * Checks if file is ans
 * @param fileName: string
 * @returns: boolean
 */
function isAnsFile(fileName: string): boolean {
  const fileExtension = fileName.split(".").pop()?.toLowerCase() || ""
  return ["ans", "ansi"].includes(fileExtension)
}

/**
 * Generates the webview content
 * @param fileName: string
 */
function updateWebviewContent(fileName: string): void {
  if (panel) {
    const s = fs.readFileSync(fileName, "utf-8")

    // @ts-ignore
    panel.webview.html = to_svg(s, Theme.Vscode)
  }
}

/**
 * Opens the SVG file viewer
 * @param fileName: string
 */
async function openViewer(fileName: string): Promise<void> {
  panel?.reveal(vscode.ViewColumn.One)

  if (!panel) {
    panel = vscode.window.createWebviewPanel(
      "SVG-Viewer",
      "SVG-Viewer",
      vscode.ViewColumn.One,
      { enableScripts: true },
    )

    // Delete panel on dispose
    panel.onDidDispose(() => {
      panel = undefined
    })
  }

  updateWebviewContent(fileName)
}

export function activate(context: vscode.ExtensionContext): void {
  // Customize statusbar
  statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Left,
    10000,
  )

  statusBarItem.text = "SVG-Viewer"
  statusBarItem.tooltip = "Looking for SVG files"
  statusBarItem.command = "extension.openExtensionPage"
  statusBarItem.show()

  // Open extension's page on click over statusbar item
  const openExtensionPageCommand = vscode.commands.registerCommand(
    "extension.openExtensionPage",
    () => {
      const extensionPageUrl = "vscode:extension/ahaoboy.ansi-viewer"
      vscode.env.openExternal(vscode.Uri.parse(extensionPageUrl))
    },
  )

  // Open preview with command 'ctrl+shift+t' when text editor is open in an svg file
  const openPreviewOnFocusCommand = vscode.commands.registerCommand(
    "extension.openPreviewOnFocus",
    () => {
      const activeTextEditor = vscode.window.activeTextEditor

      if (activeTextEditor) {
        openViewer(activeTextEditor.document.fileName)
      } else {
        vscode.window.showInformationMessage("There's no open textfile.")
      }
    },
  )

  // Open preview with mouse's right button
  const openPreviewMenuCommand = vscode.commands.registerCommand(
    "extension.openPreviewMenu",
    (resource) => {
      if (resource) {
        openViewer(resource.fsPath)
      }
    },
  )

  // Open using editor title button
  const openPreviewOnEditorButton = vscode.commands.registerCommand(
    "extension.openPreviewOnEditorShortcut",
    (resource) => {
      if (resource) {
        openViewer(resource.fsPath)
      }
    },
  )

  // Create tab rendering svg on click in svg file
  const openTextDocDisposable = vscode.workspace.onDidOpenTextDocument(
    async (document) => {
      const fileName: string = document.fileName

      if (isAnsFile(fileName)) {
        const selectedOption = !panel
          ? await vscode.window.showInformationMessage(
              "Open preview?",
              "Yes",
              "No",
            )
          : "Yes"

        if (selectedOption !== "Yes") {
          return
        }

        openViewer(fileName)
      }
    },
  )

  context.subscriptions.push(statusBarItem)
  context.subscriptions.push(openExtensionPageCommand)
  context.subscriptions.push(openPreviewOnFocusCommand)
  context.subscriptions.push(openPreviewMenuCommand)
  context.subscriptions.push(openPreviewOnEditorButton)
  context.subscriptions.push(openTextDocDisposable)
}

export function deactivate(): void {
  panel?.dispose()
  statusBarItem?.dispose()
}

module.exports = {
  activate,
  deactivate,
}

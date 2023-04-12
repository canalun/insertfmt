import * as vscode from 'vscode'
import { format_insert_queries_wasm } from '../insertfmt_core_wasm/insertfmt_core'

export function activate(context: vscode.ExtensionContext) {
  const disposable = vscode.commands.registerCommand(
    'insertfmt.fmtInsertQueries',
    () => {
      const editor = vscode.window.activeTextEditor
      if (!editor) {
        return
      }
      const text = editor.document.getText()

      const result = format_insert_queries_wasm(text)

      // replace the entire file
      const firstLine = editor.document.lineAt(0)
      const lastLine = editor.document.lineAt(editor.document.lineCount - 1)
      const textRange = new vscode.Range(
        firstLine.range.start,
        lastLine.range.end
      )
      editor.edit((editBuilder) => {
        editBuilder.replace(textRange, result)
      })
    }
  )

  context.subscriptions.push(disposable)
}

export function deactivate() {
  return
}

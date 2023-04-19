import * as vscode from 'vscode'

export function activate(context: vscode.ExtensionContext) {
  const disposable = vscode.commands.registerCommand(
    'insertfmt.fmtInsertQueries',
    () => {
      const editor = vscode.window.activeTextEditor
      if (!editor) {
        return
      }
      const text = editor.document.getText()

      import('insertfmt').then((module) => {
        try {
          const result = module.format_insert_queries_wasm(text)

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
        } catch (e) {
          vscode.window.showErrorMessage(
            ['cannot format the file.', e].join(' ')
          )
        }
      })
    }
  )
  context.subscriptions.push(disposable)
}

export function deactivate() {
  return
}

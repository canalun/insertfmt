# INSERTfmt

fast & easy vscode extension specialized to format INSERT queries🥳

![preview](https://raw.githubusercontent.com/canalun/insertfmt/main/images/preview.gif)

## How To Use 💻

1. Install it
2. Open Command Palette (OSX: `Shift` + `Cmd` + `P`; Windows: `Shift` + `Ctrl` +
   `P`)
3. Run `Format INSERT Queries`

## Release Notes 📓

### v1.0.3

- fix bug of backslash disappearance!

### v1.0.1-1.0.2

- fix error msg

### 1.0.0

- HBD🎂
- vSQLfmt gets able to format 'INSERT' queries!

## Contribution 🌟

The extension uses insertfmt core library. Please read CONTRIBUTION.md in the
root directory👶

And if you want build and run the extension on local, at the root directory,
please run `make gen-wasm-for-extension` and run debugger in 'run extension'
mode. (Of course, don't forget `yarn` in this directory)

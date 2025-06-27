# vscode-font-patcher

🖋️ A CLI tool written in Rust for patching Visual Studio Code’s `workbench.desktop.main.css` to apply custom UI fonts like Nerd Fonts.

## 🚀 Features

* Locate and patch the `font-family` in VSCode's UI CSS
* Apply custom fonts like `JetBrainsMono Nerd Font Mono`
* Detects if font is already set
* Works directly with `workbench.desktop.main.css`

![image](https://github.com/user-attachments/assets/e3407465-4c41-4d9b-9e29-c3ec7ccfecde)


## ⚙️ Usage

```bash
cargo run --release -- \
    --font "JetBrainsMono Nerd Font Mono" \
    --workbench-css-path "C:\\path\\to\\VSCode\\resources\\app\\out\\vs\\workbench\\workbench.desktop.main.css"
```

## 🛠️ Build

```bash
cargo build --release
```

## 💡 Notes

* Works on Windows (targets `.monaco-workbench.windows`)
* Restart VSCode after patching to see changes
* Backup original CSS if you want to revert manually

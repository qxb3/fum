<h3 align="center">
  <img src="https://raw.githubusercontent.com/qxb3/fum/refs/heads/main/repo/logo.png" width="200"/>
</h3>

<h2 align="center">
  fum: a tui-based mpris music client.
</h2>

<p align="center">
  fum is a tui-based mpris music client designed to provide a simple and efficient way to display and control your music within a tui interface.
</p>

<p align="center">
  <a href="https://github.com/qxb3/fum/pulls">
    <img src="https://img.shields.io/badge/OPEN-DEFEDF?style=for-the-badge&logo=github&label=Contributions&labelColor=1C1B22" />
  </a>

  <a href="https://github.com/qxb3/fum/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/MIT-DEFEDF?style=for-the-badge&logo=Pinboard&label=License&labelColor=1C1B22" />
  </a>

  <a href="https://github.com/qxb3/fum/stargazers">
    <img src="https://img.shields.io/github/stars/qxb3/fum?style=for-the-badge&logo=Apache%20Spark&logoColor=ffffff&labelColor=1C1B22&color=DEFEDF" />
  </a>

  <a href="https://aur.archlinux.org/packages/fum">
    <img src="https://img.shields.io/aur/version/fum?style=for-the-badge&logo=archlinux&logoColor=ffffff&labelColor=1C1B22&color=DEFEDF" />
  </a>

  <a href="https://crates.io/crates/fum-player">
    <img src="https://img.shields.io/crates/v/fum-player?style=for-the-badge&logo=rust&logoColor=ffffff&labelColor=1C1B22&color=DEFEDF" />
  </a>
</p>

## Demo

<img
  width="800px"
  src="https://github.com/user-attachments/assets/97aa278e-432c-4ea5-961a-840eac1cefe7" 
/>

## Roadmap / TODO

- [ ] Make some stuff visible optional
- [x] Dont start if it cant find set players
- [x] Customization of layout
- [x] width, height on config
- [x] Config
- [x] CLI with clap

## Installation

### Arch

```bash
yay -S fum
# paru -S fum
```

### Cargo (From Source)

```bash
git clone https://github.com/qxb3/fum.git
cd fum
cargo build --release

# Either copy/move `target/release/yum` to /usr/bin
# Or add the release path to your system's path

# Moving fum binary to /usr/bin
mv target/release/fum /usr/bin
```

## Configuration

Config are located on `~/.config/fum/config.json`.

### Example config with their default values:
```json
{
    "players": ["spotify"],
    "align": "center",
    "layout": "top-to-bottom",
    "hidden": [],
    "progress": "󰝤",
    "empty": "󰁱"
}
```

### Values:

| `Name`       | `Type`    | `Description` |
|--------------|-----------|---------------|
| `players`    | string[]  | String of player names that will be detected by fum. |
| `align`      | string    | Where in the terminal fum will be. Values: `center` `top` `left` `bottom` `right` `top-left` `top-right` `bottom-left` `bottom-right`. |
| `layout`     | string    | Which layout fum will use. Values: `top-to-bottom` `bottom-to-top` `left-to-right` `right-to-right`. |
| `hidden`     | string[]  | Hide some stuff from the ui. Values: `title` `artists` `buttons` `progress-bar` `progress-text`. |
| `progress`   | string    | The char that will be displayed on current progress. |
| `empty`      | string    | The char that will be displayed on empty progress. |

## Compability

Some terminals will have some issues of rendering the image as those don't support an image protocol yet.
See [Compability](https://github.com/benjajaja/ratatui-image?tab=readme-ov-file#compatibility-matrix) For compatible terminals.

## Contributing

Thank you for considering contributing to fum! Contributions are welcome and appreciated.

## LICENSE

```
MIT License

Copyright (c) 2025 qxb3

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

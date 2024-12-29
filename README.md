<div>
  <img
    style="margin-right: 32px;"
    align="left"
    src="https://raw.githubusercontent.com/qxb3/fum/refs/heads/main/repo/logo.png"
    width="128px"
    height="128px"
  />

  <div>
    <h2>fum</h2>
    <p>A tui-based mpris music client. Designed to provide a simple and efficient way to display and control your music within a tui interface.</p>
  </div>
</div>

![](https://badgers.space/github/open-issues/qxb3/fum)
![](https://badgers.space/github/closed-issues/qxb3/fum)
![](https://badgers.space/github/license/qxb3/fum)
![](https://badgers.space/crates/name/fum-player)
![](https://badgers.space/crates/version/fum-player)

## Demo

https://github.com/user-attachments/assets/2d26a2e8-becb-4ec0-a490-e1c59ab3948e

## Roadmap

- [ ] Customization of layout
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

## Contributing

Thank you for considering contributing to fum! Contributions are welcome and appreciated.

## LICENSE

```
MIT License

Copyright (c) 2024 qxb3

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

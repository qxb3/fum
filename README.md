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

## Installation

### Arch

```bash
yay -S fum
# paru -S fum
```

### Cargo (From Source)

> [!CAUTION]
> Installing from source is typically not recommended as it will probably have breaking stuff.

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
    "use_active_player": false,
    "align": "center",
    "layout": "top-to-bottom",
    "hidden": [],
    "progress": "󰝤",
    "empty": "󰁱"
}
```

### Values:

| `Name`                | `Type`    | `Description` |
|-----------------------|-----------|---------------|
| `players`             | string[]  | String of player names that will be detected by fum. This can be the identity or the bus_name of the player. Note that identity is case `INSESITIVE` and bus_name are `NOT`. |
| `use_active_player`   | boolean   | Wether to use the most likely active player when there it can't find players on the `players` array. |
| `align`               | string    | Where in the terminal fum will be. Values: `center` `top` `left` `bottom` `right` `top-left` `top-right` `bottom-left` `bottom-right`. |
| `layout`              | string    | Which layout fum will use. Values: `top-to-bottom` `bottom-to-top` `left-to-right` `right-to-right`. |
| `hidden`              | string[]  | Hide some stuff from the ui. Values: `title` `artists` `buttons` `progress-bar` `progress-text`. |
| `progress`            | string    | The char that will be displayed on current progress. |
| `empty`               | string    | The char that will be displayed on empty progress. |

## Keyboard shortcuts

| Key               | Action                          |
|-------------------|---------------------------------|
| <kbd>q</kbd>      | Quits out of Fum.              |
| <kbd>space</kbd>  | Stops the current track.       |
| <kbd>n</kbd>      | Plays the next track (if available). |
| <kbd>p</kbd>      | Plays the previous track (if available). |

## Compability

Some terminals will have some issues of rendering the image as those don't support an image protocol yet.
See [Compability](https://github.com/benjajaja/ratatui-image?tab=readme-ov-file#compatibility-matrix) For compatible terminals.

## Showcase on a rice

<img src="https://github.com/qxb3/fum/blob/main/repo/showcase.png" />

## Roadmap / TODO

- [x] Make some stuff visible optional
- [x] Dont start if it cant find set players
- [x] Customization of layout
- [x] width, height on config
- [x] Config
- [x] CLI with clap

## FaQ

####  Q: Why is there a delay in updating/changing the music?
- Two things: your internet & cpu. Everytime the song/music has been updated fum will fetch/download the image so depending on your internet speed
  this might take a while. after the image has been fetched fum will also decode the image to properly render the image from your terminal and this decoding
  is done thru your cpu.

####  Q: Why is there a slight or huge cpu spike whenever music is updated/changed?
- As stated in the answer above fum will also require to decode the image to properly render the image, And this decoding part is expensive.

## Contributing

Thank you for considering contributing to fum! Contributions are welcome and appreciated.

## LICENSE

[MIT](https://github.com/qxb3/fum/blob/main/LICENSE)

---
title: Getting Started
next: /docs/configuring:Configuring
---

# Getting Started

---

## Installation

To get started with fum. You will need to install fum to your system.

<br>

[![Packaging status](https://repology.org/badge/vertical-allrepos/fum.svg)](https://repology.org/project/fum/versions)

<br>

### Arch

```bash
yay -S fum-bin
# paru -S fum-bin
```

### Nix (Home Manager)

```nix
{ pkgs, ... }: {
  home.packages = with pkgs; [ fum ];
}
```

```bash
home-manager switch
```

### Debian based systems

Download the latest deb from [releases](https://github.com/qxb3/fum/releases) first and:

```bash
sudo dpkg -i fum-x86-64_v{DOC_VERSION}.deb
```

### RPM based systems

Download the latest rpm from [releases](https://github.com/qxb3/fum/releases) first and:

```bash
sudo dnf install fum-x86-64_v{DOC_VERSION}.rpm
```

### Build from source

```bash
git clone https://github.com/qxb3/fum.git
cd fum
cargo build --release
# Either copy/move `target/release/yum` to /usr/bin
# Or add the release path to your system's path
# Moving fum binary to /usr/bin
mv target/release/fum /usr/bin
```

## Usage

NOTE: if it says "No Music" and you don't use spotify (default), See Below.

```bash
fum
```

### Detecting other music players.

If you use different music player pass it on fum to detect:

```bash
fum --players player_identity_name,other.player.bus_name # Seperated by comma and can use identity name or bus name.
```

### List Players

To check what the music player's *identity* or *bus* name is, use:

```bash
fum list-players
Active Players:
* spotify ~> org.mpris.MediaPlayer2.spotify
```

### Compatibility

Some terminals will have some issues of rendering the image as those don't support an image protocol yet. See [Compatibility](https://github.com/benjajaja/ratatui-image#compatibility-matrix) For compatible terminals.

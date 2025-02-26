---
title: Getting Started
---

# Getting Started

To get started with fum. You will need to install fum to your system. {DOC_VERSION}

---

## Installation

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

```bash
sudo dpkg -i fum-x86-64_v{DOC_VERSION}.deb
```

### RPM based systems

```bash
sudo dnf install fum-x86-64_v{DOC_VERSION}.rpm
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

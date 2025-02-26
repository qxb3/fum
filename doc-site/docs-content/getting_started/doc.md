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
# Or use whatever latest version is.
sudo dpkg -i fum-x86-64_v{DOC_VERSION}.deb
```

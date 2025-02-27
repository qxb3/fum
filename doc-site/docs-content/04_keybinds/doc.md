---
title: Keybinds
---

# Keybinds

## Overview

keybinds are defined into `key` and `action`.

Example:

```jsonc
{
  "keybinds": {
    "esc;q;ctrl+c": "quit()"
  }
}
```

The example above tell fum to **if pressed** esc or q (You can define many key as you want) to run the `quit()` action. See [#configuring/Actions](./configuring#actions). to see the actions available.

Below is the list of available keys you can use.

## Keys

- `backspace`
- `enter`
- `left` - left arrow key.
- `right` - right arrow key.
- `down` - down arrow key.
- `up` - up arrow key.
- `end`
- `page_up`
- `page_down`
- `tab`
- `back_tab` - shift + tab key.
- `delete`
- `insert`
- `caps` - Capslock.
- `esc`
- `f1-f12`
- `all` available keyboard character - (a - z, {[]{}|;:'",<.>/? and space (" ").

### Modifiers

- `shift`
- `ctrl`
- `alt`
- `super`
- `hyper`
- `meta`

Examples: `ctrl+c`, `alt+ctrl+c`, `esc;q;ctrl+c`
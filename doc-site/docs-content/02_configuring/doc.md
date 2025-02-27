---
title: Configuring
prev: /docs/getting_started:Getting Started
next: /docs/faq:FAQ
---

# Configuring

This entire section will be covering how to configure fum.

---

## Overview

Fum's configuration is located on `$HOME/.config/fum/config.jsonc`.
This config file will be containing all of fum's functionality, look, and behavior.

Also take a note that whatever you put in here will be overwritten by the cli.
So for example you have `spotify` in the `players` in the config but have `mpd`
in the cli, `mpd` will be the one in used. Also if you don't know know `jsonc` its just a normal json with comments support.

---

### Example Config

```jsonc
{
  "players": ["spotify", "lollypop", "org.mpris.MediaPlayer2.mpv"], // List of music players to be detected.
  "use_active_player": false,                                       // Whether to detect other active mpris player.
  "fps": 10,                                                        // Fps of fum.
  "keybinds": {                                                     // Keybinds.
    "esc;q": "quit()",                                                // escape or q key to quit.
    "h": "prev()",                                                    // h key to previous music.
    "l": "next()",                                                    // l key to next music.
    " ": "play_pause()"                                               // space key to play or pause music.
  },
  "align": "center",                                                // Where in the terminal should fum be placed.
  "direction": "vertical",                                          // The parent direction of layout.
  "flex": "start",                                                  // The parent flex of layout.
  "width": 20,                                                      // The width allocated.
  "height": 18,                                                     // The height allocated.
  "border": false,                                                  // Whether to render a border around.
  "padding": [0, 0],                                                // Whether to add horizontal,vertical padding.
  "bg": "reset",                                                    // The parent background color.
  "fg": "reset",                                                    // The parent foreground color.
  "cover_art_ascii": "~/.config/fum/ascii.txt",                     // The ascii art or text to be displayed in place of cover-art if it doesn't exists.
  "layout": []                                                      // Where we define our ui layout.
}
```

---

### Config Properties

#### * `players` (Optional)

\- List of player names that will be detected by fum. This can be the name or the bus_name of the player.
Note that identity is case insensitive and bus_name are not.
<br>

- Type: `string[]`
- Default: `["spotify"]`

#### * `use_active_player` (Optional)

\- Whether to use the most likely active player when there it can't find players on the players array.
<br>

- Type: `boolean`
- Default: `false`

#### * `keybinds` (Optional)

\- Keybinds to do [#Actions](#actions). Keybinds are defined into `key` and `action`.

The dropdown below is the list of available keys you can use

<details>
  <summary>Keys List</summary>

  - `backspace`
  - `enter`
  - `left` - `left arrow key`
  - `right` - `right arrow key`
  - `down` - `down arrow key`
  - `up` - `up arrow key`
  - `end`
  - `page_up`
  - `page_down`
  - `tab`
  - `back_tab` - `shift + tab key`
  - `delete`
  - `insert`
  - `caps` - `capslock key`
  - `esc`
  - `f1` to `f12`
  - `a-z, A-Z` - `alphabetical characters`
  - `{}[]|;:'"&#96;,.<>/?` - `keyboard symbols`
</details>

<br>

- Type: `Object`
- Default:

```jsonc
{
  "esc;q": "quit()",
  "h": "prev()",
  "l": "next()",
  " ": "play_pause()"
}
```

#### * `align` (Optional)

\- Where in the terminal fum will be positioned in.
<br>

- Type: `string`
- Values:
  - `left`
  - `top`
  - `right`
  - `bottom`
  - `center`
  - `top-left`
  - `top-right`
  - `bottom-left`
  - `bottom-right`
- Default: `center`

#### * `direction` (Optional)

\- See [#Direction](#direction).
<br>

- Type: `string`
- Default: `horizontal`

#### * `flex` (Optional)

\- See [#ContainerFlex](#containerflex).
<br>

- Type: `string`
- Default: `start`

#### * `width` (Optional)

\- The allocated width.
<br>

- Type: `number`
- Default: `19`

#### * `height` (Optional)

\- The allocated height.
<br>

- Type: `number`
- Default: `15`

#### * `border` (Optional)

\- Whether to render a border around.
<br>

- Type: `boolean`
- Default: `false`

#### * `padding` (Optional)

\- Whether to add horizontal,vertical padding.
<br>

- Type: `[number, number]`
- Default: `[0, 0]`

#### * `cover_art_ascii` (Optional)

\- The ascii art or text to be displayed in place of cover-art if it doesn't exists or there is no current music.
<br>

- Type: `string`
- Default: `""`

#### * `layout` (Optional)

\- Layout ui to be rendered. See [#Widgets](#widgets) For all the available widgets.
<br>

- Type: `widget[]`
- Default: [#Example Full Config](#example-full-config)

---

### Widgets

List of available widgets.

#### `Container`

\- A container containing widgets.
<br>

- Fields:
  - `width`: `number` (optional). Specifies the width of the container. See [#Width & Height](#width--height).
  - `height`: `number` (optional). Specifies the height of the container. See [#Width & Height](#width--height).
  - `border`: `boolean` (Optional). Whether to draw a border around the widget. Default: `false`
  - `padding`: `[number, number]` (Optional). Whether to add padding on the container. Default: `[0, 0]`
  - `direction`: `string` (Optional). Specifies the layout direction of child widgets. See [#Direction](#direction).
  - `flex`: `string` (Optional). Specifies how space is distributed among child widgets. See [#ContainerFlex](#containerflex).
  - `bg`: `string` (Optional). The background color of this container area. See [#Bg & Fg](#bg--fg).
  - `fg`: `string` (Optional). The foreground color of the children. See [#Bg & Fg](#bg--fg).
  - `children`: `widget[]` (Required). The childrens of the container.

<br>

- Example:

```jsonc
{
  "type": "container",
  "width": 20,
  "height": 20,
  "border": false,
  "padding": [0, 0],
  "direction": "vertical",
  "flex": "start",
  "bg": "blue",
  "fg": "black",
  "children": [
    {
      "type": "empty",
      "size": 1
    }
  ]
}
```

#### `CoverArt`

\- Displays music cover art.
<br>

- Fields:
  - `width`: `number` (optional). Specifies the width of the container. See [#Width & Height](#width--height).
  - `height`: `number` (optional). Specifies the height of the container. See [#Width & Height](#width--height).
  - `border`: `boolean` (Optional). Whether to draw a border around the widget. Default: `false`
  - `resize`: `string` (Optional). Specifies which resize method to use.
    - Values:
      - `fit` - If the width or height is smaller than the area, the image will be resized maintaining proportions.
      - `crop` - If the width or height is smaller than the area, the image will be cropped.
      - `scale` - Scale the image.
    - Default: `scale`
  - `bg`: `string` (Optional). The background color of this container area. See [#Bg & Fg](#bg--fg).
  - `fg`: `string` (Optional). The foreground color of the children. See [#Bg & Fg](#bg--fg).

<br>

- Example:

```jsonc
{
  "type": "cover-art",
  "width": 20,
  "height": 20,
  "border": false,
  "resize": "scale",
  "bg": "red",
  "fg": "#000000"
}
```

#### `Label`

\- Displays a text label.
<br>

- Fields:
  - `text`: `string` (Required). The text to display in the label. See [#Text](#text).
  - `align`: `string` (Optional). Specifies the alignment of the text. See [#LabelAlignment](#labelalignment).
  - `truncate`: `boolean` (Optional). Specifies whether to truncate the text if it exceeds the available space.
    - Default: `true`
  - `bold`: `boolean` (Optional). Makes the label text bold. .
    - Default: `false`
  - `bg`: `string` (Optional). The background color of the label. See [#Bg & Fg](#bg--fg).
  - `fg`: `string` (Optional). The foreground color of the label. See [#Bg & Fg](#bg--fg).

<br>

- Example:

```jsonc
{
  "type": "label",
  "text": "$title",
  "align": "center",
  "truncate": true,
  "bold": false,
  "bg": "black",
  "fg": "white"
}
```

#### `Button`

\- Very similar on [#Label](#label) in terms of display but this one is interactable.
<br>

- Fields:
  - `text`: `string` (Required). The text to display in the button. See [#Text](#text).
  - `action`: `string` (Optional). Specifies an action to perform when the button is clicked. See [#Actions](#actions).
  - `exec`: `string` (Optional). Spawns a shell command to execute when the button is clicked (Note that this will quietly execute and will not notify you if it errors).
  - `bold`: `boolean` (Optional). Makes the label text bold. .
    - Default: `false`
  - `bg`: `string` (Optional). The background color of the button. See [#Bg & Fg](#bg--fg).
  - `fg`: `string` (Optional). The foreground color of the button. See [#Bg & Fg](#bg--fg).

<br>

- Example:

```jsonc
{
  "type": "button",
  "text": "$status_icon",
  "action": "play_pause()",
  "exec": "echo hi",
  "bold": false,
  "bg": "reset",
  "fg": "magenta"
}
```

#### `Progress`

\- Displays a progress bar.
<br>

- Fields:
  - `size`: `number` (optional). Specifies the width of the progress bar. See [#Width & Height](#width--height).
  - `direction`: `string` (Optional). Whether to display the progress bar horizontally or vertically. See [#Direction](#direction).
  - `progress`: string (Required).
    - `char`: The character used to represent the progress portion of the progress bar.
    - `bg`: The background color of the progress. See [#Bg & Fg](#bg--fg).
    - `fg`: The foreground color of the progress. See [#Bg & Fg](#bg--fg).
  - `empty`: string (Required).
    - `char`: The character used to represent the empty portion of the progress bar.
    - `bg`: The background color of the empty. See [#Bg & Fg](#bg--fg).
    - `fg`: The foreground color of the empty. See [#Bg & Fg](#bg--fg).

<br>

- Example:

```jsonc
{
  "type": "progress",
  "size": 10,
  "direction": "horizontal"
  "progress": {
    "char": "",
     "fg": "red",
     "bg": "blue"
  },
  "empty": {
    "char": "-",
     "fg": "blue",
     "bg": "red"
  }
}
```

#### `Volume`

\- Displays a volume bar.
<br>

- Fields:
  - `size`: `number` (optional). Specifies the width of the volume bar. See [#Width & Height](#width--height).
  - `direction`: `string` (Optional). Whether to display the volume bar horizontally or vertically. See [#Direction](#direction).
  - `volume`: string (Required).
    - `char`: The character used to represent the volume portion of the volume bar.
    - `bg`: The background color of the volume. See [#Bg & Fg](#bg--fg).
    - `fg`: The foreground color of the volume. See [#Bg & Fg](#bg--fg).
  - `empty`: string (Required).
    - `char`: The character used to represent the empty portion of the volume bar.
    - `bg`: The background color of the empty. See [#Bg & Fg](#bg--fg).
    - `fg`: The foreground color of the empty. See [#Bg & Fg](#bg--fg).

<br>

- Example:

```jsonc
{
  "type": "volume",
  "size": 10,
  "direction": "vertical",
  "volume": {
    "char": "/",
     "fg": "red",
     "bg": "blue"
  },
  "empty": {
    "char": " ",
     "fg": "blue",
     "bg": "red"
  }
}
```

#### `Empty`

\- Displays an empty area. Useful for spacing.
<br>

- Fields:
  - `size`: `number` (optional). Specifies the width of the empty space. See [#Width & Height](#width--height).

<br>

- Example:

```jsonc
{
  "type": "empty",
  "size": 1
}
```

---

### Widget Properties

List of widget properties that will be used on widget fields.

#### Width & Height

\- width and height are often optional properties. When not defined, the widget will automatically fill the remaining available space.

#### Direction

\- This property specifies the layout direction of the component.
<br>

- Values:
  - `horizontal`
  - `vertical`
- Default: `horizontal`

#### LabelAlignment

\- Specifies the alignment of text within a label.
<br>

- Values:
  - `left`
  - `center`
  - `right`
Default: `left`

#### ContainerFlex

\- Defines how space is distributed among items in a container.
<br>

- Values:
  - `start`
  - `center`
  - `end`
  - `space-around`
  - `space-between`
- Default: `start`

#### Bg & Fg

Variants:
  - `reset` - The default color.
  - `black`
  - `white`
  - `green` / `lightgreen`
  - `yellow` / `lightyellow`
  - `blue` / `lightblue`
  - `red` / `lightred`
  - `magenta` / `lightmagenta`
  - `cyan` / `lightcyan`
  - `gray` / `darkgray`
  - `rgb` - An RGB color. Example: `"fg": {"Rgb": [255, 0, 255]}`
  - `indexed` - An 8-bit 256 color. Example {"fg": {"Indexed":10}}
- Default: `reset`

#### Text

- Available variables:
  - `$title` - Title of the music.
  - `$artists` - Artists of the music.
  - `$album` - Album name of the music.
  - `$status-icon` - A single ascii icon that represents the status / playback state.
  - `$status-text` - Similar to $status-icon but in text format instead of nerdfonts icon.
  - `$position` - The current position / progress of the music.
  - `$position-ext` - Same as $position but prepended 0 at the start.
  - `$length` - The total length of the music.
  - `$length-ext` - Same as $length but prepended 0 at the start.
  - `$remaining-length` - The remaining length of the music.
  - `$remaining-length-ext` - Same as $remaining-length but prepended 0 at the start.
  - `$volume` - The current player volume (0 - 100).
<br>

- Text functions:
  - `get_meta(key: string)` - Get a specific metadata that is not available in the variables above.
  - `var($foo, $length)` - Define a mutable variable, where $foo is the variable name & $length is the variable value. You can use toggle() & set() actions to mutate it. See [#Actions](#actions). For those actions.

#### Actions

- Available actions:
  - `quit()` - Quits fum.
  - `stop()` - Stops the player.
  - `play()` - Play the music.
  - `pause()` - Pause the music.
  - `prev()` - Back the music.
  - `play_pause()` - Play / Pause the music.
  - `next()` - Skips the music.
  - `shuffle_off()` - Turn off the shuffle.
  - `shuffle_toggle()` - Toggles the shuffle on / off.
  - `shuffle_on()` - Turn on the shuffle.
  - `loop_none()` - Set the loop to none.
  - `loop_playlist()` - Set the loop to playlist.
  - `loop_track()` - Set the loop to track.
  - `loop_cycle()` - Cycle loop: none -> playlist -> track -> none.
  - `forward(2500)` - Fast forward the music 2500 milliseconds.
  - `backward(2500)` - Step backward the music 2500 milliseconds.
  - `forward(-1)` - If -1 used in forward(-1) it will go to the end of the track.
  - `backward(-1)` - If -1 used in backward(-1) it will go to the start of the track.
  - `volume(+10)` - Increases the volume +10.
  - `volume(-10)` - Decreases the volume -10.
  - `volume(50)` - Sets the volume to 50 (0 - 100).
  - `toggle($foo, $length, $remaining-length)` - Toggles the value of a variable, where $foo is the name and $length, $remaining-length is the values that will be toggled.
  - `set($foo, $title)` - Set the value of a variable, where $foo is the name and $title is the value to set.

---

### Example Full Config

```jsonc
{
  "players": ["spotify", "lollypop", "org.mpris.MediaPlayer2.mpv"],
  "use_active_player": false,
  "debug": false,
  "width": 20,
  "height": 18,
  "layout": [
    { "type": "cover-art" },
    {
      "type": "container",
      "direction": "vertical",
      "children": [
        { "type": "label", "text": "$title", "align": "center" },
        { "type": "label", "text": "$artists", "align": "center" },
        { "type": "empty", "size": 1 },
        {
          "type": "container",
          "height": 1,
          "flex": "space-around",
          "children": [
            { "type": "button", "text": "󰒮", "action": "prev()" },
            { "type": "button", "text": "$status-icon", "action": "play_pause()" },
            { "type": "button", "text": "󰒭", "action": "next()" }
          ]
        },
        { "type": "empty", "size": 1 },
        { "type": "progress", "progress": { "char": "󰝤" }, "empty": { "char": "󰁱" } },
        {
          "type": "container",
          "height": 1,
          "flex": "space-between",
          "children": [
            { "type": "label", "text": "$position", "align": "left" },
            { "type": "label", "text": "$length", "align": "right" }
          ]
        }
      ]
    }
  ]
}
```

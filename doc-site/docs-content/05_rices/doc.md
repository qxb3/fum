---
title: Rices
---

# Rices

## danielwerg - lowfi clone

![preview](/rices/danielwerg_lowfi_clone.png)

<details>
<summary>Layout Config</summary>

NOTE: Volume bar is never show ([#68](https://github.com/qxb3/fum/issues/68))

```jsonc
{
  "players": ["lowfi"],
  "keybinds": {
    "s;S": "next()",
    "n;N": "next()",
    "p;P": "play_pause()",
    "-;_;down": "volume(-5)",
    "left": "volume(-1)",
    "+;=;up": "volume(+5)",
    "right": "volume(+1)",
    "q;Q;ctrl+c": "quit()"
  },
  "width": 31,
  "height": 5,
  "border": true,
  "padding": [2, 1],
  "layout": [
    {
      "type": "container",
      "direction": "vertical",
      "children": [
        {
          "type": "container",
          "children": [
            {
              "type": "container",
              "width": 7,
              "children": [
                {
                  "type": "button",
                  "text": "$status-text",
                  "action": "play_pause()"
                }
              ]
            },
            { "type": "empty", "size": 1 },
            { "type": "label", "text": "$title", "bold": true }
          ]
        },
        {
          "type": "container",
          "children": [
            { "type": "empty", "size": 1 },
            {
              "type": "container",
              "children": [
                { "type": "button", "text": "[" },
                {
                  "type": "progress",
                  "progress": { "char": "/" },
                  "empty": { "char": " " }
                },
                { "type": "button", "text": "]" }
              ]
            },
            { "type": "empty", "size": 1 },
            {
              "type": "container",
              "width": 11,
              "children": [
                { "type": "button", "text": "$position-ext" },
                { "type": "button", "text": "/" },
                {
                  "type": "button",
                  "text": "var($length-style, $length-ext)",
                  "action": "toggle($length-style, $length-ext, $remaining-length-ext)"
                }
              ]
            }
          ]
        },
        {
          "type": "container",
          "children": [
            { "type": "empty", "size": 1 },
            {
              "type": "container",
              "width": 7,
              "children": [{ "type": "label", "text": "volume:" }]
            },
            { "type": "empty", "size": 1 },
            {
              "type": "container",
              "children": [
                { "type": "button", "text": "[" },
                {
                  "type": "volume",
                  "volume": { "char": "/" },
                  "empty": { "char": " " }
                },
                { "type": "button", "text": "]" }
              ]
            },
            { "type": "empty", "size": 1 },
            {
              "type": "container",
              "width": 4,
              "children": [{ "type": "label", "text": "$volume%" }]
            }
          ]
        },
        {
          "type": "container",
          "flex": "space-between",
          "children": [
            // { "type": "button", "text": "[s]kip", "action": "next()" },
            // { "type": "button", "text": "[p]ause", "action": "play_pause()" },
            // { "type": "button", "text": "[q]uit", "action": "quit()" }
            {
              "type": "container",
              "width": 6,
              "children": [
                {
                  "type": "button",
                  "text": "[s]",
                  "action": "next()",
                  "bold": true
                },
                { "type": "button", "text": "kip", "action": "next()" }
              ]
            },
            {
              "type": "container",
              "width": 7,
              "children": [
                {
                  "type": "button",
                  "text": "[p]",
                  "action": "play_pause()",
                  "bold": true
                },
                { "type": "button", "text": "ause", "action": "play_pause()" }
              ]
            },
            {
              "type": "container",
              "width": 6,
              "children": [
                {
                  "type": "button",
                  "text": "[q]",
                  "action": "quit()",
                  "bold": true
                },
                { "type": "button", "text": "uit", "action": "quit()" }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

</details>

## qxb3

![preview](/rices/preconfig_06.png)

<details>
<summary>Layout Config</summary>

```jsonc
{
  "players": ["spotify"],
  "debug": false,
  "keybinds": {
    "esc;q": "quit()",
    "h": "prev()",
    "l": "next()",
    " ": "play_pause()",
    "-": "volume(-5)",
    "+": "volume(+5)",
    "left": "backward(2500)",
    "right": "forward(2500)"
  },
  "width": 33,
  "height": 16,
  "direction": "vertical",
  "layout": [
    {
      "type": "container",
      "width": 33,
      "height": 11,
      "children": [
        { "type": "label", "text": "$title", "direction": "vertical", "fg": "green", "bold": true },
        { "type": "empty", "size": 2 },
        { "type": "cover-art" }
      ]
    },
    {
      "type": "container",
      "height": 4,
      "direction": "vertical",
      "children": [
        {
          "type": "container",
          "children": [
            { "type": "empty", "size": 3 },
            { "type": "button", "text": "P: " },
            { "type": "button", "text": "[" },
            { "type": "progress", "progress": { "char": "=" }, "empty": { "char": " " } },
            { "type": "button", "text": "]" }
          ]
        },
        {
          "type": "container",
          "children": [
            { "type": "empty", "size": 3 },
            { "type": "button", "text": "V: " },
            { "type": "button", "text": "[" },
            { "type": "volume", "volume": { "char": "=" }, "empty": { "char": " " } },
            { "type": "button", "text": "]" }
          ]
        },
        {
          "type": "container",
          "children": [
            { "type": "empty", "size": 3 },
            { "type": "button", "text": "[󰒮 prev]" },
            { "type": "button", "text": "[$status-icon play/pause]" },
            { "type": "button", "text": "[󰒭 next]" }
          ]
        }
      ]
    }
  ]
}
```

</details>

## qxb3

![preview](/rices/preconfig_05.png)

<details>
<summary>Layout Config</summary>

```jsonc
{
  "players": ["spotify"],
  "width": 40,
  "height": 14,
  "layout": [
    {
      "type": "container",
      "height": 3,
      "direction": "vertical",
      "children": [
        {
          "type": "label",
          "text": "==[ $title ]==",
          "align": "center",
          "bold": true,
          "fg": "yellow"
        },
        {
          "type": "progress",
          "progress": { "char": "=", "fg": "green" },
          "empty": { "char": "-", "fg": "gray" }
        }
      ]
    },
    { "type": "empty", "size": 1 },
    {
      "type": "container",
      "children": [
        { "type": "cover-art" },
        { "type": "button", "direction": "vertical", "text": "prev", "action": "prev()" },
        { "type": "empty", "size": 2 },
        { "type": "button", "direction": "vertical", "text": "$status-text", "action": "play_pause()" },
        { "type": "empty", "size": 2 },
        { "type": "button", "direction": "vertical", "text": "next", "action": "next()" }
      ]
    }
  ]
}
```

</details>

## qxb3

![preview](/rices/preconfig_04.png)

<details>
<summary>Layout Config</summary>

```jsonc
{
  "players": ["spotify", "mpv"],
  "use_active_player": true,
  "width": 30,
  "height": 5,
  "layout": [
    { "type": "label", "text": "$title", "align": "center", "bold": true },
    { "type": "label", "text": "$artists", "align": "center" },
    { "type": "empty", "size": 1 },
    {
      "type": "container",
      "flex": "space-between",
      "children": [
        { "type": "button", "text": "prev", "action": "prev()" },
        { "type": "button", "text": "play/pause", "action": "play_pause()" },
        { "type": "button", "text": "next", "action": "next()" }
      ]
    },
    { "type": "progress", "progress": { "char": "■", "fg": "white" }, "empty": { "char": "□", "fg": "gray" } }
  ]
}
```

</details>

## qxb3

![preview](/rices/preconfig_03.png)

<details>
<summary>Layout Config</summary>

```jsonc
{
  "players": ["spotify"],
  "debug": false,
  "keybinds": {
    "esc;q": "quit()",
    "h": "prev()",
    "l": "next()",
    " ": "play_pause()",
    "-": "volume(-5)",
    "+": "volume(+5)",
    "left": "backward(2500)",
    "right": "forward(2500)"
  },
  "width": 21,
  "height": 15,
  "direction": "vertical",
  "layout": [
    { "type": "label", "text": "> $title <", "align": "center" },
    { "type": "label", "text": "> $artists <", "align": "center" },
    { "type": "empty", "size": 1 },
    { "type": "cover-art" },
    { "type": "empty", "size": 1 },
    {
      "type": "container",
      "height": 1,
      "flex": "space-around",
      "children": [
        { "type": "button", "text": "󰒝", "action": "shuffle_toggle()" },
        { "type": "button", "text": "󰒮", "action": "prev()" },
        { "type": "button", "text": "$status-icon", "action": "play_pause()" },
        { "type": "button", "text": "󰒭", "action": "next()" },
        { "type": "button", "text": "󰑐", "action": "loop_track()" }
      ]
    },
    { "type": "empty", "size": 1 },
    { "type": "progress", "progress": { "char": ">" }, "empty": { "char": "<" } }
  ]
}
```

</details>

## qxb3

![preview](/rices/preconfig_02.png)

<details>
<summary>Layout Config</summary>

```jsonc
{
  "players": ["spotify"],
  "debug": false,
  "keybinds": {
    "esc;q": "quit()",
    "h": "prev()",
    "l": "next()",
    " ": "play_pause()",
    "-": "volume(-5)",
    "+": "volume(+5)",
    "left": "backward(2500)",
    "right": "forward(2500)"
  },
  "width": 43,
  "height": 8,
  "direction": "horizontal",
  "layout": [
    { "type": "cover-art" },
    { "type": "empty", "size": 2 },
    {
      "type": "container",
      "direction": "vertical",
      "children": [
        { "type": "label", "text": "󰝚 $title" },
        { "type": "label", "text": "󰠃 $artists" },
        { "type": "label", "text": "󰓎 get_meta(xesam:autoRating)" },
        { "type": "label", "text": " get_meta(xesam:discNumber)" },
        { "type": "container", "children": [] },
        {
          "type": "container",
          "height": 1,
          "children": [
            { "type": "button", "text": "󰒮", "action": "prev()" },
            { "type": "empty", "size": 3 },
            { "type": "button", "text": "$status-icon", "action": "play_pause()" },
            { "type": "empty", "size": 3 },
            { "type": "button", "text": "󰒭", "action": "next()" }
          ]
        },
        { "type": "progress", "progress": { "char": "" }, "empty": { "char": "-" } },
        {
          "type": "container",
          "flex": "space-between",
          "height": 1,
          "children": [
            { "type": "button", "text": "$position" },
            { "type": "button", "text": "var($len-style, $length)", "action": "toggle($len-style, $length, $remaining-length)" }
          ]
        }
      ]
    }
  ]
}
```

</details>

## qxb3

![preview](/rices/preconfig_01.png)

<details>
<summary>Layout Config</summary>

```jsonc
{
  "players": ["spotify"],
  "debug": false,
  "keybinds": {
    "esc;q": "quit()",
    "h": "prev()",
    "l": "next()",
    " ": "play_pause()",
    "-": "volume(-5)",
    "+": "volume(+5)",
    "left": "backward(2500)",
    "right": "forward(2500)"
  },
  "width": 22,
  "height": 10,
  "layout": [
    {
      "type": "container",
      "direction": "horizontal",
      "children": [
        {
          "type": "container",
          "direction": "vertical",
          "width": 20,
          "children": [
            { "type": "label", "text": "$title" },
            { "type": "cover-art" },
            { "type": "progress", "progress": { "char": "󰝤" }, "empty": { "char": "󰁱" } }
          ]
        },
        { "type": "empty", "size": 1 },
        {
          "type": "container",
          "direction": "vertical",
          "children": [
            { "type": "empty", "size": 1 },
            { "type": "button", "text": "󰒮", "action": "prev()" },
            { "type": "empty", "size": 1 },
            { "type": "button", "text": "$status-icon", "action": "play_pause()" },
            { "type": "empty", "size": 1 },
            { "type": "button", "text": "󰒭", "action": "next()" }
          ]
        }
      ]
    }
  ]
}
```

</details>
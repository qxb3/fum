import{H as _}from"../chunks/CYgJF_JY.js";import{b as F,c as S,t as d,a as B}from"../chunks/BRmKbYT_.js";import{i as T,j as m,k as D,o as $,q as x,P as q,Q as z,R as O,s as W,T as j,V as L,F as R,J as P,I as g,$ as H,K as t,L as c,M as f,W as V}from"../chunks/DFNjw4_i.js";import{h as M,s as v}from"../chunks/DBoNFlRm.js";import{i as w}from"../chunks/XeNX5MbG.js";import{s as E}from"../chunks/LWKitYLE.js";function I(o,s,p,r,b){var A=o,e="",C;T(()=>{if(e===(e=s()??"")){m&&D();return}C!==void 0&&(L(C),C=void 0),e!==""&&(C=$(()=>{if(m){x.data;for(var a=D(),i=a;a!==null&&(a.nodeType!==8||a.data!=="");)i=a,a=q(a);if(a===null)throw z(),O;F(x,i),A=W(a);return}var h=e+"",n=S(h);F(j(n),n.lastChild),A.before(n)}))})}function N(o,s){throw new _(o,s)}new TextEncoder;var k=[{url:"/docs/getting_started",raw:`---
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

\`\`\`bash
yay -S fum-bin
# paru -S fum-bin
\`\`\`

### Nix (Home Manager)

\`\`\`nix
{ pkgs, ... }: {
  home.packages = with pkgs; [ fum ];
}
\`\`\`

\`\`\`bash
home-manager switch
\`\`\`

### Debian based systems

Download the latest deb from [releases](https://github.com/qxb3/fum/releases) first and:

\`\`\`bash
sudo dpkg -i fum-x86-64_v{DOC_VERSION}.deb
\`\`\`

### RPM based systems

Download the latest rpm from [releases](https://github.com/qxb3/fum/releases) first and:

\`\`\`bash
sudo dnf install fum-x86-64_v{DOC_VERSION}.rpm
\`\`\`

### Build from source

\`\`\`bash
git clone https://github.com/qxb3/fum.git
cd fum
cargo build --release
# Either copy/move \`target/release/yum\` to /usr/bin
# Or add the release path to your system's path
# Moving fum binary to /usr/bin
mv target/release/fum /usr/bin
\`\`\`

## Usage

NOTE: if it says "No Music" and you don't use spotify (default), See Below.

\`\`\`bash
fum
\`\`\`

### Detecting other music players.

If you use different music player pass it on fum to detect:

\`\`\`bash
fum --players player_identity_name,other.player.bus_name # Seperated by comma and can use identity name or bus name.
\`\`\`

### List Players

To check what the music player's *identity* or *bus* name is, use:

\`\`\`bash
fum list-players
Active Players:
* spotify ~> org.mpris.MediaPlayer2.spotify
\`\`\`

### Compatibility

Some terminals will have some issues of rendering the image as those don't support an image protocol yet. See [Compatibility](https://github.com/benjajaja/ratatui-image#compatibility-matrix) For compatible terminals.
`,title:"Getting Started",next:{url:"/docs/configuring",title:"Configuring"},html:`<script context="module">
	export const metadata = {"title":"Getting Started","next":"/docs/configuring:Configuring"};
	const { title, next } = metadata;
<\/script>

<h1 id="getting-started"><a href="#getting-started">Getting Started</a></h1>
<hr>
<h2 id="installation"><a href="#installation">Installation</a></h2>
<p>To get started with fum. You will need to install fum to your system.</p>
<br>
<p><a href="https://repology.org/project/fum/versions" rel="nofollow"><img
  src="https://repology.org/badge/vertical-allrepos/fum.svg"
  alt="Packaging status"
></a></p>
<br>
<h3 id="arch"><a href="#arch">Arch</a></h3>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#7AA89F">yay</span><span style="color:#98BB6C"> -S</span><span style="color:#98BB6C"> fum-bin</span></span>
<span class="line"><span style="color:#727169"># paru -S fum-bin</span></span></code></pre>
<h3 id="nix-home-manager"><a href="#nix-home-manager">Nix (Home Manager)</a></h3>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span><span style="color:#DCD7BA"> pkgs</span><span style="color:#E6C384">,</span><span style="color:#E6C384"> ... </span><span style="color:#9CABCA">&#125;:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#957FB8">  home</span><span style="color:#DCD7BA">.</span><span style="color:#957FB8">packages</span><span style="color:#E6C384"> =</span><span style="color:#957FB8"> with</span><span style="color:#DCD7BA"> pkgs; </span><span style="color:#9CABCA">[</span><span style="color:#DCD7BA"> fum </span><span style="color:#9CABCA">];</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#7AA89F">home-manager</span><span style="color:#98BB6C"> switch</span></span></code></pre>
<h3 id="debian-based-systems"><a href="#debian-based-systems">Debian based systems</a></h3>
<p>Download the latest deb from <a
  href="https://github.com/qxb3/fum/releases"
  rel="nofollow"
>releases</a> first and:</p>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#7AA89F">sudo</span><span style="color:#98BB6C"> dpkg</span><span style="color:#98BB6C"> -i</span><span style="color:#98BB6C"> fum-x86-64_v1.3.0.deb</span></span></code></pre>
<h3 id="rpm-based-systems"><a href="#rpm-based-systems">RPM based systems</a></h3>
<p>Download the latest rpm from <a
  href="https://github.com/qxb3/fum/releases"
  rel="nofollow"
>releases</a> first and:</p>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#7AA89F">sudo</span><span style="color:#98BB6C"> dnf</span><span style="color:#98BB6C"> install</span><span style="color:#98BB6C"> fum-x86-64_v1.3.0.rpm</span></span></code></pre>
<h3 id="build-from-source"><a href="#build-from-source">Build from source</a></h3>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#7AA89F">git</span><span style="color:#98BB6C"> clone</span><span style="color:#98BB6C"> https://github.com/qxb3/fum.git</span></span>
<span class="line"><span style="color:#7E9CD8">cd</span><span style="color:#98BB6C"> fum</span></span>
<span class="line"><span style="color:#7AA89F">cargo</span><span style="color:#98BB6C"> build</span><span style="color:#98BB6C"> --release</span></span>
<span class="line"><span style="color:#727169"># Either copy/move &#96;target/release/yum&#96; to /usr/bin</span></span>
<span class="line"><span style="color:#727169"># Or add the release path to your system's path</span></span>
<span class="line"><span style="color:#727169"># Moving fum binary to /usr/bin</span></span>
<span class="line"><span style="color:#7AA89F">mv</span><span style="color:#98BB6C"> target/release/fum</span><span style="color:#98BB6C"> /usr/bin</span></span></code></pre>
<h2 id="usage"><a href="#usage">Usage</a></h2>
<p>NOTE: if it says “No Music” and you don’t use spotify (default), See Below.</p>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#7AA89F">fum</span></span></code></pre>
<h3 id="detecting-other-music-players"><a href="#detecting-other-music-players">Detecting other music players.</a></h3>
<p>If you use different music player pass it on fum to detect:</p>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#7AA89F">fum</span><span style="color:#98BB6C"> --players</span><span style="color:#98BB6C"> player_identity_name,other.player.bus_name</span><span style="color:#727169"> # Seperated by comma and can use identity name or bus name.</span></span></code></pre>
<h3 id="list-players"><a href="#list-players">List Players</a></h3>
<p>To check what the music player’s <em>identity</em> or <em>bus</em> name is, use:</p>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#7AA89F">fum</span><span style="color:#98BB6C"> list-players</span></span>
<span class="line"><span style="color:#7AA89F">Active</span><span style="color:#98BB6C"> Players:</span></span>
<span class="line"><span style="color:#E6C384">*</span><span style="color:#DCD7BA"> spotify </span><span style="color:#E6C384">~></span><span style="color:#DCD7BA"> org.mpris.MediaPlayer2.spotify</span></span></code></pre>
<h3 id="compatibility"><a href="#compatibility">Compatibility</a></h3>
<p>Some terminals will have some issues of rendering the image as those don’t support an image protocol yet. See <a
  href="https://github.com/benjajaja/ratatui-image#compatibility-matrix"
  rel="nofollow"
>Compatibility</a> For compatible terminals.</p>
`},{url:"/docs/configuring",raw:`---
title: Configuring
prev: /docs/getting_started:Getting Started
next: /docs/faq:FAQ
---

# Configuring

This entire section will be covering how to configure fum.

---

## Overview

Fum's configuration is located on \`$HOME/.config/fum/config.jsonc\`.
This config file will be containing all of fum's functionality, look, and behavior.

Also take a note that whatever you put in here will be overwritten by the cli.
So for example you have \`spotify\` in the \`players\` in the config but have \`mpd\`
in the cli, \`mpd\` will be the one in used. Also if you don't know know \`jsonc\` its just a normal json with comments support.

---

### Example Config

\`\`\`jsonc
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
\`\`\`

---

### Config Properties

#### * \`players\` (Optional)

\\- List of player names that will be detected by fum. This can be the name or the bus_name of the player.
Note that identity is case insensitive and bus_name are not.
<br>

- Type: \`string[]\`
- Default: \`["spotify"]\`

#### * \`use_active_player\` (Optional)

\\- Whether to use the most likely active player when there it can't find players on the players array.
<br>

- Type: \`boolean\`
- Default: \`false\`

#### * \`keybinds\` (Optional)

\\- Keybinds to do [#Actions](#actions). Keybinds are defined into \`key\` and \`action\`.

The dropdown below is the list of available keys you can use

<details>
  <summary>Keys List</summary>

  - \`backspace\`
  - \`enter\`
  - \`left\` - \`left arrow key\`
  - \`right\` - \`right arrow key\`
  - \`down\` - \`down arrow key\`
  - \`up\` - \`up arrow key\`
  - \`end\`
  - \`page_up\`
  - \`page_down\`
  - \`tab\`
  - \`back_tab\` - \`shift + tab key\`
  - \`delete\`
  - \`insert\`
  - \`caps\` - \`capslock key\`
  - \`esc\`
  - \`f1\` to \`f12\`
  - \`a-z, A-Z\` - \`alphabetical characters\`
  - \`{}[]|;:'"&#96;,.<>/?\` - \`keyboard symbols\`
</details>

<br>

- Type: \`Object\`
- Default:

\`\`\`jsonc
{
  "esc;q": "quit()",
  "h": "prev()",
  "l": "next()",
  " ": "play_pause()"
}
\`\`\`

#### * \`align\` (Optional)

\\- Where in the terminal fum will be positioned in.
<br>

- Type: \`string\`
- Values:
  - \`left\`
  - \`top\`
  - \`right\`
  - \`bottom\`
  - \`center\`
  - \`top-left\`
  - \`top-right\`
  - \`bottom-left\`
  - \`bottom-right\`
- Default: \`center\`

#### * \`direction\` (Optional)

\\- See [#Direction](#direction).
<br>

- Type: \`string\`
- Default: \`horizontal\`

#### * \`flex\` (Optional)

\\- See [#ContainerFlex](#containerflex).
<br>

- Type: \`string\`
- Default: \`start\`

#### * \`width\` (Optional)

\\- The allocated width.
<br>

- Type: \`number\`
- Default: \`19\`

#### * \`height\` (Optional)

\\- The allocated height.
<br>

- Type: \`number\`
- Default: \`15\`

#### * \`border\` (Optional)

\\- Whether to render a border around.
<br>

- Type: \`boolean\`
- Default: \`false\`

#### * \`padding\` (Optional)

\\- Whether to add horizontal,vertical padding.
<br>

- Type: \`[number, number]\`
- Default: \`[0, 0]\`

#### * \`cover_art_ascii\` (Optional)

\\- The ascii art or text to be displayed in place of cover-art if it doesn't exists or there is no current music.
<br>

- Type: \`string\`
- Default: \`""\`

#### * \`layout\` (Optional)

\\- Layout ui to be rendered. See [#Widgets](#widgets) For all the available widgets.
<br>

- Type: \`widget[]\`
- Default: [#Example Full Config](#example-full-config)

---

### Widgets

List of available widgets.

#### \`Container\`

\\- A container containing widgets.
<br>

- Fields:
  - \`width\`: \`number\` (optional). Specifies the width of the container. See [#Width & Height](#width--height).
  - \`height\`: \`number\` (optional). Specifies the height of the container. See [#Width & Height](#width--height).
  - \`border\`: \`boolean\` (Optional). Whether to draw a border around the widget. Default: \`false\`
  - \`padding\`: \`[number, number]\` (Optional). Whether to add padding on the container. Default: \`[0, 0]\`
  - \`direction\`: \`string\` (Optional). Specifies the layout direction of child widgets. See [#Direction](#direction).
  - \`flex\`: \`string\` (Optional). Specifies how space is distributed among child widgets. See [#ContainerFlex](#containerflex).
  - \`bg\`: \`string\` (Optional). The background color of this container area. See [#Bg & Fg](#bg--fg).
  - \`fg\`: \`string\` (Optional). The foreground color of the children. See [#Bg & Fg](#bg--fg).
  - \`children\`: \`widget[]\` (Required). The childrens of the container.

<br>

- Example:

\`\`\`jsonc
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
\`\`\`

#### \`CoverArt\`

\\- Displays music cover art.
<br>

- Fields:
  - \`width\`: \`number\` (optional). Specifies the width of the container. See [#Width & Height](#width--height).
  - \`height\`: \`number\` (optional). Specifies the height of the container. See [#Width & Height](#width--height).
  - \`border\`: \`boolean\` (Optional). Whether to draw a border around the widget. Default: \`false\`
  - \`resize\`: \`string\` (Optional). Specifies which resize method to use.
    - Values:
      - \`fit\` - If the width or height is smaller than the area, the image will be resized maintaining proportions.
      - \`crop\` - If the width or height is smaller than the area, the image will be cropped.
      - \`scale\` - Scale the image.
    - Default: \`scale\`
  - \`bg\`: \`string\` (Optional). The background color of this container area. See [#Bg & Fg](#bg--fg).
  - \`fg\`: \`string\` (Optional). The foreground color of the children. See [#Bg & Fg](#bg--fg).

<br>

- Example:

\`\`\`jsonc
{
  "type": "cover-art",
  "width": 20,
  "height": 20,
  "border": false,
  "resize": "scale",
  "bg": "red",
  "fg": "#000000"
}
\`\`\`

#### \`Label\`

\\- Displays a text label.
<br>

- Fields:
  - \`text\`: \`string\` (Required). The text to display in the label. See [#Text](#text).
  - \`align\`: \`string\` (Optional). Specifies the alignment of the text. See [#LabelAlignment](#labelalignment).
  - \`truncate\`: \`boolean\` (Optional). Specifies whether to truncate the text if it exceeds the available space.
    - Default: \`true\`
  - \`bold\`: \`boolean\` (Optional). Makes the label text bold. .
    - Default: \`false\`
  - \`bg\`: \`string\` (Optional). The background color of the label. See [#Bg & Fg](#bg--fg).
  - \`fg\`: \`string\` (Optional). The foreground color of the label. See [#Bg & Fg](#bg--fg).

<br>

- Example:

\`\`\`jsonc
{
  "type": "label",
  "text": "$title",
  "align": "center",
  "truncate": true,
  "bold": false,
  "bg": "black",
  "fg": "white"
}
\`\`\`

#### \`Button\`

\\- Very similar on [#Label](#label) in terms of display but this one is interactable.
<br>

- Fields:
  - \`text\`: \`string\` (Required). The text to display in the button. See [#Text](#text).
  - \`action\`: \`string\` (Optional). Specifies an action to perform when the button is clicked. See [#Actions](#actions).
  - \`exec\`: \`string\` (Optional). Spawns a shell command to execute when the button is clicked (Note that this will quietly execute and will not notify you if it errors).
  - \`bold\`: \`boolean\` (Optional). Makes the label text bold. .
    - Default: \`false\`
  - \`bg\`: \`string\` (Optional). The background color of the button. See [#Bg & Fg](#bg--fg).
  - \`fg\`: \`string\` (Optional). The foreground color of the button. See [#Bg & Fg](#bg--fg).

<br>

- Example:

\`\`\`jsonc
{
  "type": "button",
  "text": "$status_icon",
  "action": "play_pause()",
  "exec": "echo hi",
  "bold": false,
  "bg": "reset",
  "fg": "magenta"
}
\`\`\`

#### \`Progress\`

\\- Displays a progress bar.
<br>

- Fields:
  - \`size\`: \`number\` (optional). Specifies the width of the progress bar. See [#Width & Height](#width--height).
  - \`direction\`: \`string\` (Optional). Whether to display the progress bar horizontally or vertically. See [#Direction](#direction).
  - \`progress\`: string (Required).
    - \`char\`: The character used to represent the progress portion of the progress bar.
    - \`bg\`: The background color of the progress. See [#Bg & Fg](#bg--fg).
    - \`fg\`: The foreground color of the progress. See [#Bg & Fg](#bg--fg).
  - \`empty\`: string (Required).
    - \`char\`: The character used to represent the empty portion of the progress bar.
    - \`bg\`: The background color of the empty. See [#Bg & Fg](#bg--fg).
    - \`fg\`: The foreground color of the empty. See [#Bg & Fg](#bg--fg).

<br>

- Example:

\`\`\`jsonc
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
\`\`\`

#### \`Volume\`

\\- Displays a volume bar.
<br>

- Fields:
  - \`size\`: \`number\` (optional). Specifies the width of the volume bar. See [#Width & Height](#width--height).
  - \`direction\`: \`string\` (Optional). Whether to display the volume bar horizontally or vertically. See [#Direction](#direction).
  - \`volume\`: string (Required).
    - \`char\`: The character used to represent the volume portion of the volume bar.
    - \`bg\`: The background color of the volume. See [#Bg & Fg](#bg--fg).
    - \`fg\`: The foreground color of the volume. See [#Bg & Fg](#bg--fg).
  - \`empty\`: string (Required).
    - \`char\`: The character used to represent the empty portion of the volume bar.
    - \`bg\`: The background color of the empty. See [#Bg & Fg](#bg--fg).
    - \`fg\`: The foreground color of the empty. See [#Bg & Fg](#bg--fg).

<br>

- Example:

\`\`\`jsonc
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
\`\`\`

#### \`Empty\`

\\- Displays an empty area. Useful for spacing.
<br>

- Fields:
  - \`size\`: \`number\` (optional). Specifies the width of the empty space. See [#Width & Height](#width--height).

<br>

- Example:

\`\`\`jsonc
{
  "type": "empty",
  "size": 1
}
\`\`\`

---

### Widget Properties

List of widget properties that will be used on widget fields.

#### Width & Height

\\- width and height are often optional properties. When not defined, the widget will automatically fill the remaining available space.

#### Direction

\\- This property specifies the layout direction of the component.
<br>

- Values:
  - \`horizontal\`
  - \`vertical\`
- Default: \`horizontal\`

#### LabelAlignment

\\- Specifies the alignment of text within a label.
<br>

- Values:
  - \`left\`
  - \`center\`
  - \`right\`
Default: \`left\`

#### ContainerFlex

\\- Defines how space is distributed among items in a container.
<br>

- Values:
  - \`start\`
  - \`center\`
  - \`end\`
  - \`space-around\`
  - \`space-between\`
- Default: \`start\`

#### Bg & Fg

Variants:
  - \`reset\` - The default color.
  - \`black\`
  - \`white\`
  - \`green\` / \`lightgreen\`
  - \`yellow\` / \`lightyellow\`
  - \`blue\` / \`lightblue\`
  - \`red\` / \`lightred\`
  - \`magenta\` / \`lightmagenta\`
  - \`cyan\` / \`lightcyan\`
  - \`gray\` / \`darkgray\`
  - \`rgb\` - An RGB color. Example: \`"fg": {"Rgb": [255, 0, 255]}\`
  - \`indexed\` - An 8-bit 256 color. Example {"fg": {"Indexed":10}}
- Default: \`reset\`

#### Text

- Available variables:
  - \`$title\` - Title of the music.
  - \`$artists\` - Artists of the music.
  - \`$album\` - Album name of the music.
  - \`$status-icon\` - A single ascii icon that represents the status / playback state.
  - \`$status-text\` - Similar to $status-icon but in text format instead of nerdfonts icon.
  - \`$position\` - The current position / progress of the music.
  - \`$position-ext\` - Same as $position but prepended 0 at the start.
  - \`$length\` - The total length of the music.
  - \`$length-ext\` - Same as $length but prepended 0 at the start.
  - \`$remaining-length\` - The remaining length of the music.
  - \`$remaining-length-ext\` - Same as $remaining-length but prepended 0 at the start.
  - \`$volume\` - The current player volume (0 - 100).
<br>

- Text functions:
  - \`get_meta(key: string)\` - Get a specific metadata that is not available in the variables above.
  - \`var($foo, $length)\` - Define a mutable variable, where $foo is the variable name & $length is the variable value. You can use toggle() & set() actions to mutate it. See [#Actions](#actions). For those actions.

#### Actions

- Available actions:
  - \`quit()\` - Quits fum.
  - \`stop()\` - Stops the player.
  - \`play()\` - Play the music.
  - \`pause()\` - Pause the music.
  - \`prev()\` - Back the music.
  - \`play_pause()\` - Play / Pause the music.
  - \`next()\` - Skips the music.
  - \`shuffle_off()\` - Turn off the shuffle.
  - \`shuffle_toggle()\` - Toggles the shuffle on / off.
  - \`shuffle_on()\` - Turn on the shuffle.
  - \`loop_none()\` - Set the loop to none.
  - \`loop_playlist()\` - Set the loop to playlist.
  - \`loop_track()\` - Set the loop to track.
  - \`loop_cycle()\` - Cycle loop: none -> playlist -> track -> none.
  - \`forward(2500)\` - Fast forward the music 2500 milliseconds.
  - \`backward(2500)\` - Step backward the music 2500 milliseconds.
  - \`forward(-1)\` - If -1 used in forward(-1) it will go to the end of the track.
  - \`backward(-1)\` - If -1 used in backward(-1) it will go to the start of the track.
  - \`volume(+10)\` - Increases the volume +10.
  - \`volume(-10)\` - Decreases the volume -10.
  - \`volume(50)\` - Sets the volume to 50 (0 - 100).
  - \`toggle($foo, $length, $remaining-length)\` - Toggles the value of a variable, where $foo is the name and $length, $remaining-length is the values that will be toggled.
  - \`set($foo, $title)\` - Set the value of a variable, where $foo is the name and $title is the value to set.

---

### Example Full Config

\`\`\`jsonc
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
\`\`\`
`,title:"Configuring",prev:{url:"/docs/getting_started",title:"Getting Started"},next:{url:"/docs/faq",title:"FAQ"},html:`<script context="module">
	export const metadata = {"title":"Configuring","prev":"/docs/getting_started:Getting Started","next":"/docs/faq:FAQ"};
	const { title, prev, next } = metadata;
<\/script>

<h1 id="configuring"><a href="#configuring">Configuring</a></h1>
<p>This entire section will be covering how to configure fum.</p>
<hr>
<h2 id="overview"><a href="#overview">Overview</a></h2>
<p>Fum’s configuration is located on <code>$HOME/.config/fum/config.jsonc</code>.
This config file will be containing all of fum’s functionality, look, and behavior.</p>
<p>Also take a note that whatever you put in here will be overwritten by the cli.
So for example you have <code>spotify</code> in the <code>players</code> in the config but have <code>mpd</code>
in the cli, <code>mpd</code> will be the one in used. Also if you don’t know know <code>jsonc</code> its just a normal json with comments support.</p>
<hr>
<h3 id="example-config"><a href="#example-config">Example Config</a></h3>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"spotify"</span><span style="color:#9CABCA">,</span><span style="color:#98BB6C"> "lollypop"</span><span style="color:#9CABCA">,</span><span style="color:#98BB6C"> "org.mpris.MediaPlayer2.mpv"</span><span style="color:#9CABCA">],</span><span style="color:#727169"> // List of music players to be detected.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">use_active_player</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                       // Whether to detect other active mpris player.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">fps</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 10</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                        // Fps of fum.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">keybinds</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#727169">                                                     // Keybinds.</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">esc;q</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                // escape or q key to quit.</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">h</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                    // h key to previous music.</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">l</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                    // l key to next music.</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#9CABCA"> "</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#727169">                                               // space key to play or pause music.</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                // Where in the terminal should fum be placed.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                          // The parent direction of layout.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">flex</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "start"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                  // The parent flex of layout.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 20</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                      // The width allocated.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 18</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                     // The height allocated.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">border</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                  // Whether to render a border around.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">padding</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#D27E99">0</span><span style="color:#9CABCA">,</span><span style="color:#D27E99"> 0</span><span style="color:#9CABCA">],</span><span style="color:#727169">                                                // Whether to add horizontal,vertical padding.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "reset"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                    // The parent background color.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "reset"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                                                    // The parent foreground color.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">cover_art_ascii</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "~/.config/fum/ascii.txt"</span><span style="color:#9CABCA">,</span><span style="color:#727169">                     // The ascii art or text to be displayed in place of cover-art if it doesn't exists.</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> []</span><span style="color:#727169">                                                      // Where we define our ui layout.</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<hr>
<h3 id="config-properties"><a href="#config-properties">Config Properties</a></h3>
<h4 id="-players-optional"><a href="#-players-optional">* <code>players</code> (Optional)</a></h4>
<p>- List of player names that will be detected by fum. This can be the name or the bus_name of the player.
Note that identity is case insensitive and bus_name are not.</p>
<br>
<ul>
<li>Type: <code>string[]</code></li>
<li>Default: <code>["spotify"]</code></li>
</ul>
<h4 id="-use_active_player-optional"><a href="#-use_active_player-optional">* <code>use_active_player</code> (Optional)</a></h4>
<p>- Whether to use the most likely active player when there it can’t find players on the players array.</p>
<br>
<ul>
<li>Type: <code>boolean</code></li>
<li>Default: <code>false</code></li>
</ul>
<h4 id="-keybinds-optional"><a href="#-keybinds-optional">* <code>keybinds</code> (Optional)</a></h4>
<p>- Keybinds to do <a href="#actions">#Actions</a>. Keybinds are defined into <code>key</code> and <code>action</code>.</p>
<p>The dropdown below is the list of available keys you can use</p>
<details>
  <summary>Keys List</summary>
<ul>
<li><code>backspace</code></li>
<li><code>enter</code></li>
<li><code>left</code> - <code>left arrow key</code></li>
<li><code>right</code> - <code>right arrow key</code></li>
<li><code>down</code> - <code>down arrow key</code></li>
<li><code>up</code> - <code>up arrow key</code></li>
<li><code>end</code></li>
<li><code>page_up</code></li>
<li><code>page_down</code></li>
<li><code>tab</code></li>
<li><code>back_tab</code> - <code>shift + tab key</code></li>
<li><code>delete</code></li>
<li><code>insert</code></li>
<li><code>caps</code> - <code>capslock key</code></li>
<li><code>esc</code></li>
<li><code>f1</code> to <code>f12</code></li>
<li><code>a-z, A-Z</code> - <code>alphabetical characters</code></li>
<li><code>&#123;&#125;[]|;:'"&#96;,.&lt;&gt;/?</code> - <code>keyboard symbols</code>
</details>
</li>
</ul>
<br>
<ul>
<li>Type: <code>Object</code></li>
<li>Default:</li>
</ul>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">esc;q</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">h</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">l</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#9CABCA"> "</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<h4 id="-align-optional"><a href="#-align-optional">* <code>align</code> (Optional)</a></h4>
<p>- Where in the terminal fum will be positioned in.</p>
<br>
<ul>
<li>Type: <code>string</code></li>
<li>Values:
<ul>
<li><code>left</code></li>
<li><code>top</code></li>
<li><code>right</code></li>
<li><code>bottom</code></li>
<li><code>center</code></li>
<li><code>top-left</code></li>
<li><code>top-right</code></li>
<li><code>bottom-left</code></li>
<li><code>bottom-right</code></li>
</ul>
</li>
<li>Default: <code>center</code></li>
</ul>
<h4 id="-direction-optional"><a href="#-direction-optional">* <code>direction</code> (Optional)</a></h4>
<p>- See <a href="#direction">#Direction</a>.</p>
<br>
<ul>
<li>Type: <code>string</code></li>
<li>Default: <code>horizontal</code></li>
</ul>
<h4 id="-flex-optional"><a href="#-flex-optional">* <code>flex</code> (Optional)</a></h4>
<p>- See <a href="#containerflex">#ContainerFlex</a>.</p>
<br>
<ul>
<li>Type: <code>string</code></li>
<li>Default: <code>start</code></li>
</ul>
<h4 id="-width-optional"><a href="#-width-optional">* <code>width</code> (Optional)</a></h4>
<p>- The allocated width.</p>
<br>
<ul>
<li>Type: <code>number</code></li>
<li>Default: <code>19</code></li>
</ul>
<h4 id="-height-optional"><a href="#-height-optional">* <code>height</code> (Optional)</a></h4>
<p>- The allocated height.</p>
<br>
<ul>
<li>Type: <code>number</code></li>
<li>Default: <code>15</code></li>
</ul>
<h4 id="-border-optional"><a href="#-border-optional">* <code>border</code> (Optional)</a></h4>
<p>- Whether to render a border around.</p>
<br>
<ul>
<li>Type: <code>boolean</code></li>
<li>Default: <code>false</code></li>
</ul>
<h4 id="-padding-optional"><a href="#-padding-optional">* <code>padding</code> (Optional)</a></h4>
<p>- Whether to add horizontal,vertical padding.</p>
<br>
<ul>
<li>Type: <code>[number, number]</code></li>
<li>Default: <code>[0, 0]</code></li>
</ul>
<h4 id="-cover_art_ascii-optional"><a href="#-cover_art_ascii-optional">* <code>cover_art_ascii</code> (Optional)</a></h4>
<p>- The ascii art or text to be displayed in place of cover-art if it doesn’t exists or there is no current music.</p>
<br>
<ul>
<li>Type: <code>string</code></li>
<li>Default: <code>""</code></li>
</ul>
<h4 id="-layout-optional"><a href="#-layout-optional">* <code>layout</code> (Optional)</a></h4>
<p>- Layout ui to be rendered. See <a href="#widgets">#Widgets</a> For all the available widgets.</p>
<br>
<ul>
<li>Type: <code>widget[]</code></li>
<li>Default: <a href="#example-full-config">#Example Full Config</a></li>
</ul>
<hr>
<h3 id="widgets"><a href="#widgets">Widgets</a></h3>
<p>List of available widgets.</p>
<h4 id="container"><a href="#container"><code>Container</code></a></h4>
<p>- A container containing widgets.</p>
<br>
<ul>
<li>Fields:
<ul>
<li><code>width</code>: <code>number</code> (optional). Specifies the width of the container. See <a
  href="#width--height"
>#Width & Height</a>.</li>
<li><code>height</code>: <code>number</code> (optional). Specifies the height of the container. See <a
  href="#width--height"
>#Width & Height</a>.</li>
<li><code>border</code>: <code>boolean</code> (Optional). Whether to draw a border around the widget. Default: <code>false</code></li>
<li><code>padding</code>: <code>[number, number]</code> (Optional). Whether to add padding on the container. Default: <code>[0, 0]</code></li>
<li><code>direction</code>: <code>string</code> (Optional). Specifies the layout direction of child widgets. See <a
  href="#direction"
>#Direction</a>.</li>
<li><code>flex</code>: <code>string</code> (Optional). Specifies how space is distributed among child widgets. See <a
  href="#containerflex"
>#ContainerFlex</a>.</li>
<li><code>bg</code>: <code>string</code> (Optional). The background color of this container area. See <a
  href="#bg--fg"
>#Bg & Fg</a>.</li>
<li><code>fg</code>: <code>string</code> (Optional). The foreground color of the children. See <a
  href="#bg--fg"
>#Bg & Fg</a>.</li>
<li><code>children</code>: <code>widget[]</code> (Required). The childrens of the container.</li>
</ul>
</li>
</ul>
<br>
<ul>
<li>Example:</li>
</ul>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 20</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 20</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">border</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">padding</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#D27E99">0</span><span style="color:#9CABCA">,</span><span style="color:#D27E99"> 0</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">flex</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "start"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "blue"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "black"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<h4 id="coverart"><a href="#coverart"><code>CoverArt</code></a></h4>
<p>- Displays music cover art.</p>
<br>
<ul>
<li>Fields:
<ul>
<li><code>width</code>: <code>number</code> (optional). Specifies the width of the container. See <a
  href="#width--height"
>#Width & Height</a>.</li>
<li><code>height</code>: <code>number</code> (optional). Specifies the height of the container. See <a
  href="#width--height"
>#Width & Height</a>.</li>
<li><code>border</code>: <code>boolean</code> (Optional). Whether to draw a border around the widget. Default: <code>false</code></li>
<li><code>resize</code>: <code>string</code> (Optional). Specifies which resize method to use.
<ul>
<li>Values:
<ul>
<li><code>fit</code> - If the width or height is smaller than the area, the image will be resized maintaining proportions.</li>
<li><code>crop</code> - If the width or height is smaller than the area, the image will be cropped.</li>
<li><code>scale</code> - Scale the image.</li>
</ul>
</li>
<li>Default: <code>scale</code></li>
</ul>
</li>
<li><code>bg</code>: <code>string</code> (Optional). The background color of this container area. See <a
  href="#bg--fg"
>#Bg & Fg</a>.</li>
<li><code>fg</code>: <code>string</code> (Optional). The foreground color of the children. See <a
  href="#bg--fg"
>#Bg & Fg</a>.</li>
</ul>
</li>
</ul>
<br>
<ul>
<li>Example:</li>
</ul>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "cover-art"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 20</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 20</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">border</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">resize</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "scale"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "red"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "#000000"</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<h4 id="label"><a href="#label"><code>Label</code></a></h4>
<p>- Displays a text label.</p>
<br>
<ul>
<li>Fields:
<ul>
<li><code>text</code>: <code>string</code> (Required). The text to display in the label. See <a href="#text">#Text</a>.</li>
<li><code>align</code>: <code>string</code> (Optional). Specifies the alignment of the text. See <a
  href="#labelalignment"
>#LabelAlignment</a>.</li>
<li><code>truncate</code>: <code>boolean</code> (Optional). Specifies whether to truncate the text if it exceeds the available space.
<ul>
<li>Default: <code>true</code></li>
</ul>
</li>
<li><code>bold</code>: <code>boolean</code> (Optional). Makes the label text bold. .
<ul>
<li>Default: <code>false</code></li>
</ul>
</li>
<li><code>bg</code>: <code>string</code> (Optional). The background color of the label. See <a href="#bg--fg">#Bg & Fg</a>.</li>
<li><code>fg</code>: <code>string</code> (Optional). The foreground color of the label. See <a href="#bg--fg">#Bg & Fg</a>.</li>
</ul>
</li>
</ul>
<br>
<ul>
<li>Example:</li>
</ul>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$title"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">truncate</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "black"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "white"</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<h4 id="button"><a href="#button"><code>Button</code></a></h4>
<p>- Very similar on <a href="#label">#Label</a> in terms of display but this one is interactable.</p>
<br>
<ul>
<li>Fields:
<ul>
<li><code>text</code>: <code>string</code> (Required). The text to display in the button. See <a href="#text">#Text</a>.</li>
<li><code>action</code>: <code>string</code> (Optional). Specifies an action to perform when the button is clicked. See <a
  href="#actions"
>#Actions</a>.</li>
<li><code>exec</code>: <code>string</code> (Optional). Spawns a shell command to execute when the button is clicked (Note that this will quietly execute and will not notify you if it errors).</li>
<li><code>bold</code>: <code>boolean</code> (Optional). Makes the label text bold. .
<ul>
<li>Default: <code>false</code></li>
</ul>
</li>
<li><code>bg</code>: <code>string</code> (Optional). The background color of the button. See <a href="#bg--fg">#Bg & Fg</a>.</li>
<li><code>fg</code>: <code>string</code> (Optional). The foreground color of the button. See <a href="#bg--fg">#Bg & Fg</a>.</li>
</ul>
</li>
</ul>
<br>
<ul>
<li>Example:</li>
</ul>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$status_icon"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">exec</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "echo hi"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "reset"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "magenta"</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<h4 id="progress"><a href="#progress"><code>Progress</code></a></h4>
<p>- Displays a progress bar.</p>
<br>
<ul>
<li>Fields:
<ul>
<li><code>size</code>: <code>number</code> (optional). Specifies the width of the progress bar. See <a
  href="#width--height"
>#Width & Height</a>.</li>
<li><code>direction</code>: <code>string</code> (Optional). Whether to display the progress bar horizontally or vertically. See <a
  href="#direction"
>#Direction</a>.</li>
<li><code>progress</code>: string (Required).
<ul>
<li><code>char</code>: The character used to represent the progress portion of the progress bar.</li>
<li><code>bg</code>: The background color of the progress. See <a href="#bg--fg">#Bg & Fg</a>.</li>
<li><code>fg</code>: The foreground color of the progress. See <a href="#bg--fg">#Bg & Fg</a>.</li>
</ul>
</li>
<li><code>empty</code>: string (Required).
<ul>
<li><code>char</code>: The character used to represent the empty portion of the progress bar.</li>
<li><code>bg</code>: The background color of the empty. See <a href="#bg--fg">#Bg & Fg</a>.</li>
<li><code>fg</code>: The foreground color of the empty. See <a href="#bg--fg">#Bg & Fg</a>.</li>
</ul>
</li>
</ul>
</li>
</ul>
<br>
<ul>
<li>Example:</li>
</ul>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 10</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "horizontal"</span></span>
<span class="line"><span style="color:#98BB6C">  "progress"</span><span style="color:#E82424">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> ""</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">     "</span><span style="color:#E6C384">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "red"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">     "</span><span style="color:#E6C384">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "blue"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "-"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">     "</span><span style="color:#E6C384">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "blue"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">     "</span><span style="color:#E6C384">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "red"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<h4 id="volume"><a href="#volume"><code>Volume</code></a></h4>
<p>- Displays a volume bar.</p>
<br>
<ul>
<li>Fields:
<ul>
<li><code>size</code>: <code>number</code> (optional). Specifies the width of the volume bar. See <a
  href="#width--height"
>#Width & Height</a>.</li>
<li><code>direction</code>: <code>string</code> (Optional). Whether to display the volume bar horizontally or vertically. See <a
  href="#direction"
>#Direction</a>.</li>
<li><code>volume</code>: string (Required).
<ul>
<li><code>char</code>: The character used to represent the volume portion of the volume bar.</li>
<li><code>bg</code>: The background color of the volume. See <a href="#bg--fg">#Bg & Fg</a>.</li>
<li><code>fg</code>: The foreground color of the volume. See <a href="#bg--fg">#Bg & Fg</a>.</li>
</ul>
</li>
<li><code>empty</code>: string (Required).
<ul>
<li><code>char</code>: The character used to represent the empty portion of the volume bar.</li>
<li><code>bg</code>: The background color of the empty. See <a href="#bg--fg">#Bg & Fg</a>.</li>
<li><code>fg</code>: The foreground color of the empty. See <a href="#bg--fg">#Bg & Fg</a>.</li>
</ul>
</li>
</ul>
</li>
</ul>
<br>
<ul>
<li>Example:</li>
</ul>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 10</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">volume</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "/"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">     "</span><span style="color:#E6C384">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "red"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">     "</span><span style="color:#E6C384">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "blue"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> " "</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">     "</span><span style="color:#E6C384">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "blue"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">     "</span><span style="color:#E6C384">bg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "red"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<h4 id="empty"><a href="#empty"><code>Empty</code></a></h4>
<p>- Displays an empty area. Useful for spacing.</p>
<br>
<ul>
<li>Fields:
<ul>
<li><code>size</code>: <code>number</code> (optional). Specifies the width of the empty space. See <a
  href="#width--height"
>#Width & Height</a>.</li>
</ul>
</li>
</ul>
<br>
<ul>
<li>Example:</li>
</ul>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
<hr>
<h3 id="widget-properties"><a href="#widget-properties">Widget Properties</a></h3>
<p>List of widget properties that will be used on widget fields.</p>
<h4 id="width--height"><a href="#width--height">Width & Height</a></h4>
<p>- width and height are often optional properties. When not defined, the widget will automatically fill the remaining available space.</p>
<h4 id="direction"><a href="#direction">Direction</a></h4>
<p>- This property specifies the layout direction of the component.</p>
<br>
<ul>
<li>Values:
<ul>
<li><code>horizontal</code></li>
<li><code>vertical</code></li>
</ul>
</li>
<li>Default: <code>horizontal</code></li>
</ul>
<h4 id="labelalignment"><a href="#labelalignment">LabelAlignment</a></h4>
<p>- Specifies the alignment of text within a label.</p>
<br>
<ul>
<li>Values:
<ul>
<li><code>left</code></li>
<li><code>center</code></li>
<li><code>right</code>
Default: <code>left</code></li>
</ul>
</li>
</ul>
<h4 id="containerflex"><a href="#containerflex">ContainerFlex</a></h4>
<p>- Defines how space is distributed among items in a container.</p>
<br>
<ul>
<li>Values:
<ul>
<li><code>start</code></li>
<li><code>center</code></li>
<li><code>end</code></li>
<li><code>space-around</code></li>
<li><code>space-between</code></li>
</ul>
</li>
<li>Default: <code>start</code></li>
</ul>
<h4 id="bg--fg"><a href="#bg--fg">Bg & Fg</a></h4>
<p>Variants:</p>
<ul>
<li><code>reset</code> - The default color.</li>
<li><code>black</code></li>
<li><code>white</code></li>
<li><code>green</code> / <code>lightgreen</code></li>
<li><code>yellow</code> / <code>lightyellow</code></li>
<li><code>blue</code> / <code>lightblue</code></li>
<li><code>red</code> / <code>lightred</code></li>
<li><code>magenta</code> / <code>lightmagenta</code></li>
<li><code>cyan</code> / <code>lightcyan</code></li>
<li><code>gray</code> / <code>darkgray</code></li>
<li><code>rgb</code> - An RGB color. Example: <code>"fg": &#123;"Rgb": [255, 0, 255]&#125;</code></li>
<li><code>indexed</code> - An 8-bit 256 color. Example {“fg”: {“Indexed”:10}}</li>
<li>Default: <code>reset</code></li>
</ul>
<h4 id="text"><a href="#text">Text</a></h4>
<ul>
<li>
<p>Available variables:</p>
<ul>
<li><code>$title</code> - Title of the music.</li>
<li><code>$artists</code> - Artists of the music.</li>
<li><code>$album</code> - Album name of the music.</li>
<li><code>$status-icon</code> - A single ascii icon that represents the status / playback state.</li>
<li><code>$status-text</code> - Similar to $status-icon but in text format instead of nerdfonts icon.</li>
<li><code>$position</code> - The current position / progress of the music.</li>
<li><code>$position-ext</code> - Same as $position but prepended 0 at the start.</li>
<li><code>$length</code> - The total length of the music.</li>
<li><code>$length-ext</code> - Same as $length but prepended 0 at the start.</li>
<li><code>$remaining-length</code> - The remaining length of the music.</li>
<li><code>$remaining-length-ext</code> - Same as $remaining-length but prepended 0 at the start.</li>
<li><code>$volume</code> - The current player volume (0 - 100).
<br>
</li>
</ul>
</li>
<li>
<p>Text functions:</p>
<ul>
<li><code>get_meta(key: string)</code> - Get a specific metadata that is not available in the variables above.</li>
<li><code>var($foo, $length)</code> - Define a mutable variable, where $foo is the variable name & $length is the variable value. You can use toggle() & set() actions to mutate it. See <a
  href="#actions"
>#Actions</a>. For those actions.</li>
</ul>
</li>
</ul>
<h4 id="actions"><a href="#actions">Actions</a></h4>
<ul>
<li>Available actions:
<ul>
<li><code>quit()</code> - Quits fum.</li>
<li><code>stop()</code> - Stops the player.</li>
<li><code>play()</code> - Play the music.</li>
<li><code>pause()</code> - Pause the music.</li>
<li><code>prev()</code> - Back the music.</li>
<li><code>play_pause()</code> - Play / Pause the music.</li>
<li><code>next()</code> - Skips the music.</li>
<li><code>shuffle_off()</code> - Turn off the shuffle.</li>
<li><code>shuffle_toggle()</code> - Toggles the shuffle on / off.</li>
<li><code>shuffle_on()</code> - Turn on the shuffle.</li>
<li><code>loop_none()</code> - Set the loop to none.</li>
<li><code>loop_playlist()</code> - Set the loop to playlist.</li>
<li><code>loop_track()</code> - Set the loop to track.</li>
<li><code>loop_cycle()</code> - Cycle loop: none -> playlist -> track -> none.</li>
<li><code>forward(2500)</code> - Fast forward the music 2500 milliseconds.</li>
<li><code>backward(2500)</code> - Step backward the music 2500 milliseconds.</li>
<li><code>forward(-1)</code> - If -1 used in forward(-1) it will go to the end of the track.</li>
<li><code>backward(-1)</code> - If -1 used in backward(-1) it will go to the start of the track.</li>
<li><code>volume(+10)</code> - Increases the volume +10.</li>
<li><code>volume(-10)</code> - Decreases the volume -10.</li>
<li><code>volume(50)</code> - Sets the volume to 50 (0 - 100).</li>
<li><code>toggle($foo, $length, $remaining-length)</code> - Toggles the value of a variable, where $foo is the name and $length, $remaining-length is the values that will be toggled.</li>
<li><code>set($foo, $title)</code> - Set the value of a variable, where $foo is the name and $title is the value to set.</li>
</ul>
</li>
</ul>
<hr>
<h3 id="example-full-config"><a href="#example-full-config">Example Full Config</a></h3>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"spotify"</span><span style="color:#9CABCA">,</span><span style="color:#98BB6C"> "lollypop"</span><span style="color:#9CABCA">,</span><span style="color:#98BB6C"> "org.mpris.MediaPlayer2.mpv"</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">use_active_player</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">debug</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 20</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 18</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "cover-art"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$title"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$artists"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">flex</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "space-around"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒮"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$status-icon"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒭"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">progress</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰝤"</span><span style="color:#9CABCA"> &#125;,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰁱"</span><span style="color:#9CABCA"> &#125;</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">flex</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "space-between"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$position"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "left"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$length"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "right"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
`},{url:"/docs/faq",raw:`---
title: FAQ
prev: /docs/configuring:Configuring
next: /docs/compability:Compability
---

# FAQ

---

## Why is there a delay in updating/changing the music?

> Two things: your internet & cpu. Everytime the song/music has been updated fum will fetch/download the image so depending on your internet speed this might take a while. after the image has been fetched fum will also decode the image to properly render the image from your terminal and this decoding is done thru your cpu.

## Why is there a slight or huge cpu spike whenever music is updated/changed?

> As stated in the answer above fum will also require to decode the image to properly render the image, And this decoding part is expensive.
`,title:"FAQ",prev:{url:"/docs/configuring",title:"Configuring"},next:{url:"/docs/compability",title:"Compability"},html:`<script context="module">
	export const metadata = {"title":"FAQ","prev":"/docs/configuring:Configuring","next":"/docs/compability:Compability"};
	const { title, prev, next } = metadata;
<\/script>

<h1 id="faq"><a href="#faq">FAQ</a></h1>
<hr>
<h2 id="why-is-there-a-delay-in-updatingchanging-the-music"><a
  href="#why-is-there-a-delay-in-updatingchanging-the-music"
>Why is there a delay in updating/changing the music?</a></h2>
<blockquote>
<p>Two things: your internet & cpu. Everytime the song/music has been updated fum will fetch/download the image so depending on your internet speed this might take a while. after the image has been fetched fum will also decode the image to properly render the image from your terminal and this decoding is done thru your cpu.</p>
</blockquote>
<h2
  id="why-is-there-a-slight-or-huge-cpu-spike-whenever-music-is-updatedchanged"
><a
  href="#why-is-there-a-slight-or-huge-cpu-spike-whenever-music-is-updatedchanged"
>Why is there a slight or huge cpu spike whenever music is updated/changed?</a></h2>
<blockquote>
<p>As stated in the answer above fum will also require to decode the image to properly render the image, And this decoding part is expensive.</p>
</blockquote>
`},{url:"/docs/compability",raw:`---
title: Compability
prev: /docs/faq:FAQ
next: /docs/rices:Rices
---

# Compability

---

Some terminals will have some issues of rendering the image as those don't support an image protocol yet.
See [Compability](https://github.com/benjajaja/ratatui-image?tab=readme-ov-file#compatibility-matrix) For compatible terminals.
`,title:"Compability",prev:{url:"/docs/faq",title:"FAQ"},next:{url:"/docs/rices",title:"Rices"},html:`<script context="module">
	export const metadata = {"title":"Compability","prev":"/docs/faq:FAQ","next":"/docs/rices:Rices"};
	const { title, prev, next } = metadata;
<\/script>

<h1 id="compability"><a href="#compability">Compability</a></h1>
<hr>
<p>Some terminals will have some issues of rendering the image as those don’t support an image protocol yet.
See <a
  href="https://github.com/benjajaja/ratatui-image?tab=readme-ov-file#compatibility-matrix"
  rel="nofollow"
>Compability</a> For compatible terminals.</p>
`},{url:"/docs/rices",raw:`---
title: Rices
prev: /docs/compability:Compability
---

# Rices

Compilation of rices / customization of fum.

---

### * danielwerg - lowfi clone

![preview](/rices/danielwerg_lowfi_clone.png)

<details>
<summary>Layout Config</summary>

NOTE: Volume bar is never show ([#68](https://github.com/qxb3/fum/issues/68))

\`\`\`jsonc
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
\`\`\`
</details>

### * qxb3

![preview](/rices/preconfig_06.png)

<details>
<summary>Layout Config</summary>

\`\`\`jsonc
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
\`\`\`

</details>

### * qxb3

![preview](/rices/preconfig_05.png)

<details>
<summary>Layout Config</summary>

\`\`\`jsonc
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
\`\`\`

</details>

### * qxb3

![preview](/rices/preconfig_04.png)

<details>
<summary>Layout Config</summary>

\`\`\`jsonc
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
\`\`\`

</details>

### * qxb3

![preview](/rices/preconfig_03.png)

<details>
<summary>Layout Config</summary>

\`\`\`jsonc
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
\`\`\`

</details>

### * qxb3

![preview](/rices/preconfig_02.png)

<details>
<summary>Layout Config</summary>

\`\`\`jsonc
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
\`\`\`

</details>

### * qxb3

![preview](/rices/preconfig_01.png)

<details>
<summary>Layout Config</summary>

\`\`\`jsonc
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
\`\`\`

</details>
`,title:"Rices",prev:{url:"/docs/compability",title:"Compability"},html:`<script context="module">
	export const metadata = {"title":"Rices","prev":"/docs/compability:Compability"};
	const { title, prev } = metadata;
<\/script>

<h1 id="rices"><a href="#rices">Rices</a></h1>
<p>Compilation of rices / customization of fum.</p>
<hr>
<h3 id="-danielwerg---lowfi-clone"><a href="#-danielwerg---lowfi-clone">* danielwerg - lowfi clone</a></h3>
<p><img src="/rices/danielwerg_lowfi_clone.png" alt="preview"></p>
<details>
<summary>Layout Config</summary>
<p>NOTE: Volume bar is never show (<a
  href="https://github.com/qxb3/fum/issues/68"
  rel="nofollow"
>#68</a>)</p>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"lowfi"</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">keybinds</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">s;S</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">n;N</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">p;P</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">-;_;down</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(-5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">left</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(-1)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">+;=;up</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(+5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">right</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(+1)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">q;Q;ctrl+c</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 31</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 5</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">border</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">padding</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#D27E99">2</span><span style="color:#9CABCA">,</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 7</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$status-text"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span></span>
<span class="line"><span style="color:#9CABCA">                &#125;</span></span>
<span class="line"><span style="color:#9CABCA">              ]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$title"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "["</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">progress</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#7E9CD8">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "/"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#7E9CD8">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> " "</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">                &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "]"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">              ]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 11</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$position-ext"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "/"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "var($length-style, $length-ext)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "toggle($length-style, $length-ext, $remaining-length-ext)"</span></span>
<span class="line"><span style="color:#9CABCA">                &#125;</span></span>
<span class="line"><span style="color:#9CABCA">              ]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 7</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [&#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume:"</span><span style="color:#9CABCA"> &#125;]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "["</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">volume</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#7E9CD8">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "/"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#7E9CD8">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> " "</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">                &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "]"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">              ]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 4</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [&#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$volume%"</span><span style="color:#9CABCA"> &#125;]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">flex</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "space-between"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#727169">            // &#123; "type": "button", "text": "[s]kip", "action": "next()" &#125;,</span></span>
<span class="line"><span style="color:#727169">            // &#123; "type": "button", "text": "[p]ause", "action": "play_pause()" &#125;,</span></span>
<span class="line"><span style="color:#727169">            // &#123; "type": "button", "text": "[q]uit", "action": "quit()" &#125;</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 6</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "[s]"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span></span>
<span class="line"><span style="color:#9CABCA">                &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "kip"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">              ]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 7</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "[p]"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span></span>
<span class="line"><span style="color:#9CABCA">                &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "ause"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">              ]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 6</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">              "</span><span style="color:#FF5D62">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "[q]"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">                  "</span><span style="color:#FFA066">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span></span>
<span class="line"><span style="color:#9CABCA">                &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">                &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "uit"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">              ]</span></span>
<span class="line"><span style="color:#9CABCA">            &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
</details>
<h3 id="-qxb3"><a href="#-qxb3">* qxb3</a></h3>
<p><img src="/rices/preconfig_06.png" alt="preview"></p>
<details>
<summary>Layout Config</summary>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"spotify"</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">debug</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">keybinds</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">esc;q</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">h</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">l</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#9CABCA"> "</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">-</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(-5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">+</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(+5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">left</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "backward(2500)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">right</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "forward(2500)"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 33</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 16</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 33</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 11</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$title"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "green"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 2</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "cover-art"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 4</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 3</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "P: "</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "["</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">progress</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "="</span><span style="color:#9CABCA"> &#125;,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> " "</span><span style="color:#9CABCA"> &#125;</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "]"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 3</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "V: "</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "["</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">volume</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "="</span><span style="color:#9CABCA"> &#125;,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> " "</span><span style="color:#9CABCA"> &#125;</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "]"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 3</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "[󰒮 prev]"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "[$status-icon play/pause]"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "[󰒭 next]"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
</details>
<h3 id="-qxb3-1"><a href="#-qxb3-1">* qxb3</a></h3>
<p><img src="/rices/preconfig_05.png" alt="preview"></p>
<details>
<summary>Layout Config</summary>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"spotify"</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 40</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 14</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 3</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "==[ $title ]=="</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "yellow"</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">progress</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "="</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "green"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "-"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "gray"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "cover-art"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 2</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$status-text"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 2</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
</details>
<h3 id="-qxb3-2"><a href="#-qxb3-2">* qxb3</a></h3>
<p><img src="/rices/preconfig_04.png" alt="preview"></p>
<details>
<summary>Layout Config</summary>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"spotify"</span><span style="color:#9CABCA">,</span><span style="color:#98BB6C"> "mpv"</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">use_active_player</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 30</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 5</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$title"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">bold</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> true</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$artists"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">flex</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "space-between"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play/pause"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">progress</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "■"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "white"</span><span style="color:#9CABCA"> &#125;,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "□"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">fg</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "gray"</span><span style="color:#9CABCA"> &#125;</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
</details>
<h3 id="-qxb3-3"><a href="#-qxb3-3">* qxb3</a></h3>
<p><img src="/rices/preconfig_03.png" alt="preview"></p>
<details>
<summary>Layout Config</summary>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"spotify"</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">debug</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">keybinds</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">esc;q</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">h</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">l</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#9CABCA"> "</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">-</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(-5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">+</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(+5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">left</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "backward(2500)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">right</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "forward(2500)"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 21</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 15</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "> $title &#x3C;"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "> $artists &#x3C;"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">align</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "center"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "cover-art"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">flex</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "space-around"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒝"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "shuffle_toggle()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒮"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$status-icon"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒭"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰑐"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "loop_track()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">progress</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> ">"</span><span style="color:#9CABCA"> &#125;,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "&#x3C;"</span><span style="color:#9CABCA"> &#125;</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
</details>
<h3 id="-qxb3-4"><a href="#-qxb3-4">* qxb3</a></h3>
<p><img src="/rices/preconfig_02.png" alt="preview"></p>
<details>
<summary>Layout Config</summary>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"spotify"</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">debug</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">keybinds</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">esc;q</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">h</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">l</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#9CABCA"> "</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">-</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(-5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">+</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(+5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">left</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "backward(2500)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">right</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "forward(2500)"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 43</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 8</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "horizontal"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "cover-art"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#E6C384">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 2</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰝚 $title"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰠃 $artists"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰓎 get_meta(xesam:autoRating)"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> " get_meta(xesam:discNumber)"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> []</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒮"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 3</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$status-icon"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 3</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒭"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">progress</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> ""</span><span style="color:#9CABCA"> &#125;,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "-"</span><span style="color:#9CABCA"> &#125;</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">flex</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "space-between"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$position"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "var($len-style, $length)"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "toggle($len-style, $length, $remaining-length)"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
</details>
<h3 id="-qxb3-5"><a href="#-qxb3-5">* qxb3</a></h3>
<p><img src="/rices/preconfig_01.png" alt="preview"></p>
<details>
<summary>Layout Config</summary>
<pre class="shiki kanagawa-wave" style="background-color:#1F1F28;color:#DCD7BA" tabindex="0"><code><span class="line"><span style="color:#9CABCA">&#123;</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">players</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span><span style="color:#98BB6C">"spotify"</span><span style="color:#9CABCA">],</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">debug</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#FFA066"> false</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">keybinds</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">esc;q</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "quit()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">h</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">l</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#9CABCA"> "</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">-</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(-5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">+</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "volume(+5)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">left</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "backward(2500)"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">    "</span><span style="color:#E6C384">right</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "forward(2500)"</span></span>
<span class="line"><span style="color:#9CABCA">  &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 22</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">height</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 10</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">  "</span><span style="color:#D27E99">layout</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">    &#123;</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "horizontal"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">      "</span><span style="color:#E6C384">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">width</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 20</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "label"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$title"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "cover-art"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "progress"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">progress</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰝤"</span><span style="color:#9CABCA"> &#125;,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">empty</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">char</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰁱"</span><span style="color:#9CABCA"> &#125;</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FFA066">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">        &#123;</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "container"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">direction</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "vertical"</span><span style="color:#9CABCA">,</span></span>
<span class="line"><span style="color:#9CABCA">          "</span><span style="color:#FFA066">children</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#9CABCA"> [</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒮"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "prev()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "$status-icon"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "play_pause()"</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "empty"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">size</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#D27E99"> 1</span><span style="color:#9CABCA"> &#125;,</span></span>
<span class="line"><span style="color:#9CABCA">            &#123;</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">type</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "button"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">text</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "󰒭"</span><span style="color:#9CABCA">,</span><span style="color:#9CABCA"> "</span><span style="color:#FF5D62">action</span><span style="color:#9CABCA">"</span><span style="color:#9CABCA">:</span><span style="color:#98BB6C"> "next()"</span><span style="color:#9CABCA"> &#125;</span></span>
<span class="line"><span style="color:#9CABCA">          ]</span></span>
<span class="line"><span style="color:#9CABCA">        &#125;</span></span>
<span class="line"><span style="color:#9CABCA">      ]</span></span>
<span class="line"><span style="color:#9CABCA">    &#125;</span></span>
<span class="line"><span style="color:#9CABCA">  ]</span></span>
<span class="line"><span style="color:#9CABCA">&#125;</span></span></code></pre>
</details>
`}];const Q=async({params:o})=>{const{title:s}=o,p=k.find(r=>r.title.toLowerCase().replaceAll(" ","_")===s);if(!p)throw N(404,"Documentation Not Found.");return{doc:p}},G=()=>k.map(o=>({title:o.title.toLowerCase().replaceAll(" ","_")})),K=!0,ps=Object.freeze(Object.defineProperty({__proto__:null,entries:G,load:Q,prerender:K},Symbol.toStringTag,{value:"Module"}));var Y=d('<a class="self-start font-bold text-fg! text-4xl cursor-pointer hover:text-background duration-300 transition-colors"><span><i class="fa-solid fa-chevron-left"></i></span> <span> </span></a>'),U=d("<div></div>"),Z=d('<a class="font-bold text-fg! text-4xl cursor-pointer hover:text-background duration-300 transition-colors"><span> </span> <span><i class="fa-solid fa-chevron-right"></i></span></a>'),J=d('<main class="doc p-8"><div class="overflow-x-hidden"><!></div> <hr> <div class="flex justify-between items-center"><!> <!></div></main>');function es(o,s){R(s,!0);var p=J();M(n=>{g(()=>H.title=s.data.doc.title)});var r=t(p),b=t(r);I(b,()=>s.data.doc.html),c(r);var A=f(r,4),e=t(A);{var C=n=>{var l=Y(),y=f(t(l),2),u=t(y,!0);c(y),c(l),g(()=>{E(l,"href",s.data.doc.prev.url),v(u,s.data.doc.prev.title)}),B(n,l)},a=n=>{var l=U();B(n,l)};w(e,n=>{s.data.doc.prev?n(C):n(a,!1)})}var i=f(e,2);{var h=n=>{var l=Z(),y=t(l),u=t(y,!0);c(y),V(2),c(l),g(()=>{E(l,"href",s.data.doc.next.url),v(u,s.data.doc.next.title)}),B(n,l)};w(i,n=>{s.data.doc.next&&n(h)})}c(A),c(p),B(o,p),P()}export{es as component,ps as universal};

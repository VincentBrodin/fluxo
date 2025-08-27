
# fluxo

`fluxo` is a tiny tool that reacts to monitor changes in **Hyprland**.
Instead of cloning displays or manually toggling configs, you can define per monitor rules in a simple JSON file.

When a monitor connects or disconnects, `fluxo` runs the commands you’ve set for example, disabling your laptop screen when an external monitor is plugged in.

## Install

### AUR

Coming soon™

### From Source

```bash
git clone https://github.com/VincentBrodin/fluxo.git
cd fluxo
cargo build --release
```

Then add this to your Hyprland config:

```
exec-once = PATH/TO/fluxo/target/release/fluxo
```

## Usage

In your config folder, add `fluxo/config.json`.
Here’s an example (my personal setup):

```json
{
  "HDMI-A-1": {
    "on_added": [
      "eDP-1,disabled"
    ],
    "on_removed": [
      "eDP-1,1920x1080@60,0x0,1"
    ]
  }
}
```

### What it does

* When `HDMI-A-1` gets plugged in, it runs the `on_added` command → disables `eDP-1`.
* On startup, `fluxo` checks all monitors listed in your config.

  * If a monitor is connected, it runs `on_added`.
  * If not, it runs `on_removed`.

## Syntax

The commands use Hyprland’s built in monitor syntax. [Read more here](https://wiki.hypr.land/Configuring/Monitors/).


## Contributing

Contributions are very welcomeu
Bug reports, feature requests, PRs all appreciated.

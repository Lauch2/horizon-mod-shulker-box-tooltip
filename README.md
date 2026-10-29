# ShulkerBoxTooltip for Horizon

A native mod for the Horizon Minecraft client that shows previews of shulker boxes and other containers.

## Features

- **Corner HUD Preview**: Hold the preview key (default: Left Shift) while holding a container to see its contents in a corner of the screen
- **Command Preview**: Use `/shulkerpreview` to print the held item's contents to chat
- **Configurable**: Settings page in Options > Mods > ShulkerBoxTooltip
  - Corner position (Top Left, Top Right, Bottom Left, Bottom Right)
  - Show/hide key hints
  - Compact mode (hide empty slots)

## Supported Containers

- Shulker Boxes (all 16 colors)
- Chests (normal, trapped)
- Barrels
- Ender Chests
- Bundles (all colors)
- Furnaces (normal, blast, smoker)
- Hoppers
- Dispensers & Droppers
- Brewing Stands
- Chiseled Bookshelves
- Decorated Pots
- Lecterns

## Installation

1. Copy the `shulker-box-tooltip` folder to your `horizon-mods/native/` directory
2. Run the Horizon launcher
3. Go to Mods → ShulkerBoxTooltip → Install
4. Go to Settings → Game → Native mods (experimental) → Build
5. Select "Native mods" instance and play

## Usage

- **Hold Shift** (or configured key) while holding a container to show preview in corner HUD
- **Release key** to close preview
- **`/shulkerpreview`** in chat to see held item contents
- **Options > Mods > ShulkerBoxTooltip > Settings** to configure

## Limitations

This is a partial port of the Java [ShulkerBoxTooltip](https://github.com/MisterPeModder/ShulkerBoxTooltip) mod. Due to missing native hook points in Horizon's API, the following features are not available:

- ❌ True hover preview (only works on held item)
- ❌ Preview rendered adjacent to tooltip
- ❌ Full item model rendering (text-only)
- ❌ Lock preview with separate key
- ❌ Compact/full toggle via key combo

A feature request for the required `native_tooltip` hook has been filed (see `FEATURE_REQUEST.md`).

## Building from Source

Requires Horizon's source tree (for `horizon-client`, `horizon-core`, `horizon-world` dependencies).

```bash
cd horizon-mods/native/shulker-box-tooltip
cargo check --lib
```

Or use the Horizon mod tools:

```bash
python3 native/tools/native.py check shulker-box-tooltip
python3 native/tools/native.py package shulker-box-tooltip
```

## License

MIT
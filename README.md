# ShulkerBoxTooltip for Horizon

A native mod that shows previews of shulker boxes and other containers in a corner HUD when you hold the preview key (default: **Left Shift**) while holding a container. Also provides a `/shulkerpreview` command to print contents to chat.

> **Note:** Screenshots/GIFs can be added to `docs/preview.gif` — contributions welcome!

---

## ⚠️ Current Status (2026-10-29)

**Repo is PRIVATE** (only the owner can see it). Work in progress — not yet release-ready.

| Area | Status |
|------|--------|
| Mod compiles & builds in Horizon launcher | ✅ (`native-build` läuft durch) |
| Mod listed in Options → Mods (switch active) | ✅ |
| All hooks fire: `native_tick`, `native_key`, `native_hud`, `native_command` | ✅ (per chat debug output) |
| Shift-key detection (`key.keyboard.left.shift`) | ✅ (`[SBT] key down/up` in chat) |
| `/shulkerpreview` command handler | ✅ (`[SBT] command: ...` in chat) |
| Corner HUD drawing code | ✅ (ready) |
| **Container content extraction** | ❌ **OPEN ISSUE** — a held shulker box is not recognized as a container |

### The open bug: extraction returns `data=false`

`extract_container_contents()` reads `stack.data()?.container` / `bundle_contents` / `block_entity_data`. In testing, a held shulker box reports:

```
id=minecraft:shulker_box data=false container_slots=0 other=[]
```

`data=false` means the item has **no `StackData` at all** (no components). This happens for items from the **creative inventory** or `/give` without NBT. A shulker box that was **mined in survival** (place → fill → break → pick up) should carry `minecraft:container` in `StackData.container` — that path has not yet been confirmed working.

**Next step:** test with a survival-mined filled shulker box and report the full debug line:
```
[SBT] hud: not a container (id=… data=… container_slots=… other=[…])
```
or via `/shulkerpreview`:
```
minecraft:shulker_box is not a container (id=… data=… container_slots=… other=[…])
```

### Temporary debug output (to be removed before release)

While diagnosing, the mod prints chat lines:
- `[SBT] tick` — every ~5 s while in a world (proves `native_tick` runs)
- `[SBT] key down/up` — preview key press/release (proves `native_key` runs)
- `[SBT] command: /…` — every typed command (proves `native_command` runs)
- `[SBT] hud: …` — HUD diagnostics every ~5 s while Shift is held
- `[SBT] preview shown` — once when a preview appears

These will be removed once extraction works.

### Useful commands during testing

```bash
/shulkerpreview          # print held item contents to chat (+ debug reason)
/shulkerpreview force    # toggle corner preview without holding Shift
```

---

## Features

| Feature | Description |
|---------|-------------|
| **Corner HUD Preview** | Hold preview key while holding a container → contents shown in configurable screen corner |
| **Command Preview** | `/shulkerpreview` prints held item contents to chat |
| **Settings UI** | Options → Mods → ShulkerBoxTooltip → Settings |
| **16 Container Types** | Shulker boxes (all 16 colors), chests, barrels, ender chests, bundles, furnaces, hoppers, dispensers, droppers, brewing stands, chiseled bookshelves, decorated pots, lecterns |
| **Dyed Colors** | Shulker box window color matches the dye |
| **Compact Mode** | Hide empty slots in preview |

---

## Installation

### Prerequisites
- Horizon launcher installed (from [playhorizon.dev](https://playhorizon.dev))
- Minecraft Java Edition 26.3 (owned, launcher prepares automatically)
- Native mods enabled in Horizon Settings → Game → Native mods (experimental)

### Quick Install

```bash
# Clone the repo
git clone https://github.com/Lauch2/horizon-mod-shulker-box-tooltip.git

# Copy to Horizon native mods folder
mkdir -p ~/.local/share/horizon/mods/native
cp -r horizon-mod-shulker-box-tooltip ~/.local/share/horizon/mods/native/
```

### Via Horizon Launcher
1. Open Horizon launcher
2. Go to **Mods** tab
3. Find **ShulkerBoxTooltip** → click **Install**
4. Go to **Settings** → **Game** → **Native mods (experimental)** → toggle **ON**
5. Click **Build** → read warning → click **"I trust these mods: build"**
6. Wait 20–30 minutes (first build downloads Rust and compiles Horizon)
7. Select **"Native mods"** instance and click **Play**

---

## Usage

| Action | Key / Command |
|--------|---------------|
| Show preview | Hold **Left Shift** (default) while holding a container |
| Hide preview | Release the key |
| Chat preview | `/shulkerpreview` |
| Configure | Options → Mods → ShulkerBoxTooltip → Settings |

### Settings

| Setting | Options | Default |
|---------|---------|---------|
| Corner | Top Left, Top Right, Bottom Left, Bottom Right | Top Right |
| Key Hints | ON / OFF | ON |
| Compact Mode | ON / OFF | OFF |

---

## Supported Containers

| Container | Slots | Preview Color |
|-----------|-------|---------------|
| Shulker Box (all 16 colors) | 27 | Matches dye |
| Chest / Trapped Chest | 27 | Brown |
| Barrel | 27 | Brown |
| Ender Chest | 27 | Purple |
| Bundle (all colors) | 64 | Orange |
| Furnace / Blast Furnace / Smoker | 3 | Gray |
| Hopper | 5 | Gray |
| Dispenser / Dropper | 9 | Gray |
| Brewing Stand | 5 | Gray |
| Chiseled Bookshelf | 6 | Brown |
| Decorated Pot | 5 | Tan |
| Lectern | 1 | Brown |

---

## AI Agent Integration (MCP)

Horizon includes a built-in MCP server for AI agents (Claude Code, Codex, Cursor, etc.).

### Connect Your Agent

**Claude Code:**
```bash
claude mcp add --scope user horizon -- /path/to/horizon-launcher mcp
```

**Codex:**
```bash
codex mcp add horizon -- /path/to/horizon-launcher mcp
```

**Cursor / Claude Desktop / Any MCP Client:**
```json
{
  "mcpServers": {
    "horizon": {
      "command": "/path/to/horizon-launcher",
      "args": ["mcp"]
    }
  }
}
```

### Agent Capabilities with This Mod

| Task | MCP Tools |
|------|-----------|
| Start game with mod | `horizon-ctl launch --world "My World"` |
| Check mod loaded | `horizon-ctl state` → check mods list |
| Test preview | `horizon-ctl chat "/shulkerpreview"` |
| Read HUD preview | `horizon-ctl ui` → find corner HUD widgets |
| Screenshot preview | `horizon-ctl screenshot preview.png` |

### Example Agent Workflow

```python
# Agent script to test the mod
async def test_shulkerbox_tooltip(agent):
    # 1. Start game
    await agent.game_launch(world="Test World")
    await agent.game_wait_ready()
    
    # 2. Give player a shulker box with items
    await agent.game_give("shulker_box")
    await agent.game_command("item replace inventory.0 with shulker_box{BlockEntityTag:{Items:[{Slot:0b,id:\"diamond\",Count:64}]}}")
    
    # 3. Hold preview key
    await agent.game_key("key.keyboard.left.shift", "down")
    await agent.game_wait(10)  # let preview render
    
    # 4. Read UI for preview window
    ui = await agent.game_ui()
    preview_widgets = [w for w in ui if "shulker" in w.label.lower() or "preview" in w.label.lower()]
    
    # 5. Screenshot
    await agent.game_screenshot("shulker_preview.png")
    
    # 6. Release key
    await agent.game_key("key.keyboard.left.shift", "up")
```

---

## Development

### Project Structure

```
shulker-box-tooltip/
├── Cargo.toml                    # Workspace root
├── compatibility.json            # API version + Horizon revision
├── README.md                     # This file
├── FEATURE_REQUEST.md            # Upstream hook request
├── LICENSE                       # MIT
└── mixins/shulker_box_tooltip/
    ├── Cargo.toml                # Library config
    ├── horizon-mixin.toml        # 7 injections (tick, hud, key, command, list, settings, click)
    ├── runtime.rs                # Core logic, container parsing, settings
    ├── tick.rs                   # Tick hook (debug heartbeat)
    ├── hud.rs                    # Corner HUD preview rendering
    ├── key.rs                    # Preview key tracking
    ├── command.rs                # /shulkerpreview command
    ├── list.rs                   # Mods page registration
    ├── settings.rs               # Settings UI rendering
    └── click.rs                  # Settings click handling
```

### Building from Source

Requires Horizon source tree (for `horizon-client`, `horizon-core`, `horizon-world` dependencies).

```bash
cd shulker-box-tooltip
cargo check --lib

# Or use Horizon mod tools
python3 ../native/tools/native.py check shulker-box-tooltip
python3 ../native/tools/native.py package shulker-box-tooltip
```

### Hook Points Used

| Hook | Target | Purpose |
|------|--------|---------|
| `native_tick` | `Game::native_tick` | Debug heartbeat (to be removed) |
| `native_hud` | `Game::native_hud` | Draw corner preview window |
| `native_key` | `Game::native_key` | Track preview key (Shift) state |
| `native_command` | `Game::native_command` | Handle `/shulkerpreview` |
| `native_mods` | `Game::native_mods` | Register in Mods page |
| `native_settings` | `Game::native_settings` | Render settings buttons |
| `native_settings_click` | `Game::native_settings_click` | Handle settings clicks |

---

## Limitations

This is a **partial port** of the Java [ShulkerBoxTooltip](https://github.com/MisterPeModder/ShulkerBoxTooltip) mod. Due to missing native hook points in Horizon's API v1/v2:

| Missing Feature | Required Hook |
|-----------------|---------------|
| True hover preview (inventory hover) | `native_tooltip` / `native_hovered_stack` |
| Preview adjacent to tooltip | `native_tooltip` |
| Full item model rendering | `native_tooltip` + item render API |
| Lock preview (separate key) | `native_tooltip` + key combos |
| Compact/full toggle via key combo | `native_tooltip` |

A feature request for `native_tooltip` hook is in [`FEATURE_REQUEST.md`](FEATURE_REQUEST.md).

---

## Roadmap

- [ ] **Full hover preview** — requires `native_tooltip` / `native_hovered_stack` hook (see [FEATURE_REQUEST.md](FEATURE_REQUEST.md))
- [ ] **Item model rendering** — render actual item icons instead of text
- [ ] **Lock preview** — separate key to pin preview window
- [ ] **Compact/full toggle** — key combo to switch between modes
- [ ] **More container support** — bundles with nested items, decorated pot patterns
- [ ] **Configuration file** — TOML config for advanced users

---

## Acknowledgments

- **MisterPeModder** — Original [ShulkerBoxTooltip](https://github.com/MisterPeModder/ShulkerBoxTooltip) Java mod for Fabric/Forge/NeoForge
- **Horizon team** — Rust rewrite of Minecraft Java Edition and native mod system
- **horizon-rust/horizon-mods** — Reference native mods (FPS Overlay, Minimap, Keystrokes, Zoom, Coords HUD)

---

## License

MIT — see [LICENSE](LICENSE)

---

## Links

- **Repository**: https://github.com/Lauch2/horizon-mod-shulker-box-tooltip
- **Horizon Docs**: https://playhorizon.dev/docs/make-a-mod
- **Original Java Mod**: https://github.com/MisterPeModder/ShulkerBoxTooltip
- **Horizon MCP/Agent Control**: https://playhorizon.dev/docs/agents
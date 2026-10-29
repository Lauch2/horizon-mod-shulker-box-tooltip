# Horizon Native Mod Feature Request: Tooltip Hook Point

## Summary
Request for a new native hook point (`native_tooltip` or `native_hovered_stack`) to allow native mods to detect and modify item tooltips, enabling ports of popular tooltip mods like ShulkerBoxTooltip.

## Motivation
The ShulkerBoxTooltip mod (https://github.com/MisterPeModder/ShulkerBoxTooltip) is a popular Fabric/Forge/NeoForge mod that shows a preview window of container contents (shulker boxes, chests, barrels, bundles, ender chests, lecterns, decorated pots, shelves, etc.) when hovering over them in an inventory while holding Shift.

Currently, Horizon's native mod API (v1/v2) provides hooks for:
- HUD drawing (`native_hud`)
- Key input (`native_key`)
- Commands (`native_command`)
- Mod listing/settings (`native_mods`, `native_settings`, `native_settings_click`)
- Recipe panel (`native_panel_items`, `native_panel_recipes`) - v2 only

**Missing:** There is no hook for:
1. Detecting the currently hovered ItemStack in inventory/container screens
2. Injecting custom rendering adjacent to or replacing the vanilla tooltip
3. Modifying tooltip content (adding/removing lines)

This makes it impossible to port tooltip-based mods natively.

## Proposed API
Add a new hook point (API v3) in `crates/horizon-client/src/game/native_hooks.rs`:

```rust
/// Called when rendering an item tooltip. The mod can:
/// - Read the hovered stack via `stack`
/// - Add/remove/modify tooltip lines via `lines`
/// - Request a custom preview render via `preview` (return Some to override)
/// - `screen` indicates which screen is open (inventory, container, creative, etc.)
pub(super) fn native_tooltip(
    &mut self,
    stack: &ItemStack,
    lines: &mut Vec<(String, u32)>,  // (text, color)
    preview: &mut Option<NativeTooltipPreview>,
    screen: TooltipScreen,
) {
    let _ = (stack, lines, preview, screen);
}

/// Preview data a mod can provide to render a custom tooltip window
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeTooltipPreview {
    pub items: Vec<NativeStack>,  // Items to show in preview grid
    pub title: String,
    pub color: u32,  // ARGB
    pub max_cols: u32,
}

/// Which screen the tooltip is being rendered in
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TooltipScreen {
    Inventory,
    Container,
    Creative,
    RecipeBook,
    Other,
}
```

Also add a helper to get the currently hovered stack:
```rust
/// The ItemStack currently hovered by the mouse cursor, if any.
/// Only valid during `native_tooltip` hook.
pub(super) fn native_hovered_stack(&self) -> Option<&ItemStack>;
```

## Current Workaround
A partial port exists using available hooks:
- **Corner HUD preview** (`native_hud` + `native_key`): Shows held item contents in a screen corner when preview key (default Shift) is pressed
- **Command preview** (`native_command`): `/shulkerpreview` prints held item contents to chat

This provides ~60% of the original mod's utility but lacks:
- True hover detection (only works on held item)
- Preview rendering adjacent to tooltip
- Full item model rendering (text-only fallback)
- Lock preview, compact/full toggle via key combo

## Related Files
- Mod implementation (partial port): `native/shulker-box-tooltip/` in horizon-mods
- Original Java mod: https://github.com/MisterPeModder/ShulkerBoxTooltip

## Priority
High - This would unlock a whole category of tooltip mods (WAILA/HWYLA, Item Tooltips, ShulkerBoxTooltip, etc.) for Horizon native modding.
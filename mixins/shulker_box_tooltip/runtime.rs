use std::cell::{Cell, RefCell};
use std::sync::Mutex;
use horizon_client::items::ItemStack;
use horizon_world::nbt::{Compound, JavaString, Tag};

pub const ID: &str = "shulker_box_tooltip";
pub const NAME: &str = "ShulkerBoxTooltip";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Preview shulker boxes and containers in a corner HUD. Press the preview key (default Shift) while holding a container.";

pub const BACKGROUND: u32 = 0x7800_0000;
pub const INK: u32 = 0xFF_FFFF;
pub const SLOT_SIZE: f32 = 18.0;
pub const SLOT_PADDING: f32 = 1.0;
pub const MAX_COLS: usize = 9;

const SHULKER_COLORS: &[(&str, u32)] = &[
    ("white", 0xFF_F0F0F0),
    ("orange", 0xFF_F8761A),
    ("magenta", 0xFF_E066D0),
    ("light_blue", 0xFF_3AB3DA),
    ("yellow", 0xFF_FED83D),
    ("lime", 0xFF_80C71F),
    ("pink", 0xFF_F98AAA),
    ("gray", 0xFF_474F52),
    ("light_gray", 0xFF_9D9D97),
    ("cyan", 0xFF_169C9C),
    ("purple", 0xFF_8932B8),
    ("blue", 0xFF_3C44AA),
    ("brown", 0xFF_835432),
    ("green", 0xFF_5E7C16),
    ("red", 0xFF_B02E26),
    ("black", 0xFF_1D1D21),
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub corner: usize,
    pub show_key_hints: bool,
    pub compact_mode: bool,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let corner = get("corner")
            .and_then(|c| c.trim().parse().ok())
            .unwrap_or(1);
        let show_key_hints = get("show_key_hints")
            .map(|v| v.trim() == "true")
            .unwrap_or(true);
        let compact_mode = get("compact_mode")
            .map(|v| v.trim() == "true")
            .unwrap_or(false);
        Self { corner, show_key_hints, compact_mode }
    }

    pub fn labels(&self) -> Vec<String> {
        const CORNERS: [&str; 4] = ["Top left", "Top right", "Bottom left", "Bottom right"];
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Corner: {}", CORNERS[self.corner.min(3)]),
            format!("Key hints: {}", on(self.show_key_hints)),
            format!("Compact mode: {}", on(self.compact_mode)),
        ]
    }

    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.corner = (self.corner + 1) % 4,
            1 => self.show_key_hints = !self.show_key_hints,
            2 => self.compact_mode = !self.compact_mode,
            _ => {}
        }
    }

    pub fn stored(&self) -> [(&'static str, String); 3] {
        [
            ("corner", self.corner.to_string()),
            ("show_key_hints", self.show_key_hints.to_string()),
            ("compact_mode", self.compact_mode.to_string()),
        ]
    }
}

thread_local! {
    static SETTINGS: Cell<Option<Settings>> = const { Cell::new(None) };
    static PREVIEW_STATE: RefCell<Option<PreviewState>> = const { RefCell::new(None) };
    static PREVIEW_KEY_PRESSED: Cell<bool> = const { Cell::new(false) };
    static PREVIEW_FORCED: Cell<bool> = const { Cell::new(false) };
    static DEBUG_TICK: Cell<u32> = const { Cell::new(0) };
    static DEBUG_HUD: Cell<u32> = const { Cell::new(0) };
}

/// Shared between the window's game (HUD, command) and, in single player, the
/// server's game thread, where the authoritative inventory lives. The client
/// sync does not carry `minecraft:container`, so the server extracts the
/// preview and the window reads it.
static SHARED: Mutex<Option<PreviewState>> = Mutex::new(None);

/// The server-side game pushes the preview of the held stack here.
pub fn server_push(preview: Option<PreviewState>) {
    if let Ok(mut g) = SHARED.lock() {
        *g = preview;
    }
}

/// The window-side game reads the server's preview.
pub fn server_pull() -> Option<PreviewState> {
    SHARED.lock().ok().and_then(|g| g.clone())
}

/// TEMP DEBUG: true every 100 calls (tick debug, ~5 s).
pub fn debug_tick() -> bool {
    DEBUG_TICK.with(|c| {
        let n = c.get().wrapping_add(1);
        c.set(n);
        n % 100 == 0
    })
}

/// TEMP DEBUG: true every 100 calls (hud debug).
pub fn debug_hud() -> bool {
    DEBUG_HUD.with(|c| {
        let n = c.get().wrapping_add(1);
        c.set(n);
        n % 100 == 0
    })
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with(|s| {
        let found = s.get().unwrap_or_else(|| Settings::read(get));
        s.set(Some(found));
        found
    })
}

pub fn set_settings(new: Settings) {
    SETTINGS.with(|s| s.set(Some(new)));
}

pub fn preview_state() -> Option<PreviewState> {
    PREVIEW_STATE.with(|s| s.borrow().clone())
}

pub fn set_preview_state(v: Option<PreviewState>) {
    PREVIEW_STATE.with(|s| *s.borrow_mut() = v);
}

pub fn preview_key_pressed() -> bool {
    PREVIEW_KEY_PRESSED.with(|s| s.get())
}

pub fn set_preview_key_pressed(v: bool) {
    PREVIEW_KEY_PRESSED.with(|s| s.set(v));
}

pub fn preview_forced() -> bool {
    PREVIEW_FORCED.with(|s| s.get())
}

pub fn set_preview_forced(v: bool) {
    PREVIEW_FORCED.with(|s| s.set(v));
}

#[derive(Clone, Debug)]
pub struct PreviewState {
    pub items: Vec<Option<ItemStackPreview>>,
    pub title: String,
    pub color: u32,
}

#[derive(Clone, Debug)]
pub struct ItemStackPreview {
    pub item_id: String,
    pub count: u32,
}

impl PreviewState {
    pub fn dimensions(&self, settings: &Settings) -> (f32, f32) {
        let visible = if settings.compact_mode {
            self.items.iter().filter(|i| i.is_some()).count()
        } else {
            self.items.len()
        };
        let cols = visible.min(MAX_COLS);
        let rows = (visible + MAX_COLS - 1) / MAX_COLS;
        let w = 14.0 + cols as f32 * (SLOT_SIZE + SLOT_PADDING);
        let h = 14.0 + rows as f32 * (SLOT_SIZE + SLOT_PADDING) + 16.0;
        (w, h)
    }
}

pub fn preview_key(game: &crate::game::Game<'_>) -> String {
    game.native_mod_key(ID, "preview", "key.keyboard.left.shift")
}

/// Get a key from a compound map (`BTreeMap<JavaString, Tag>`) by `&str`.
fn compound_get<'a>(map: &'a Compound, key: &str) -> Option<&'a Tag> {
    map.get(&JavaString::from(key))
}

pub fn extract_container_contents(stack: &ItemStack) -> Option<PreviewState> {
    let (title, max_slots, color) = match &*stack.item {
        "minecraft:shulker_box" => ("Shulker Box", 27, 0xFF_404040),
        "minecraft:chest" => ("Chest", 27, 0xFF_8B4513),
        "minecraft:trapped_chest" => ("Trapped Chest", 27, 0xFF_8B4513),
        "minecraft:barrel" => ("Barrel", 27, 0xFF_8B4513),
        "minecraft:ender_chest" => ("Ender Chest", 27, 0xFF_800080),
        "minecraft:bundle" => ("Bundle", 64, 0xFF_D2691E),
        "minecraft:furnace" => ("Furnace", 3, 0xFF_696969),
        "minecraft:blast_furnace" => ("Blast Furnace", 3, 0xFF_696969),
        "minecraft:smoker" => ("Smoker", 3, 0xFF_696969),
        "minecraft:hopper" => ("Hopper", 5, 0xFF_696969),
        "minecraft:dispenser" => ("Dispenser", 9, 0xFF_696969),
        "minecraft:dropper" => ("Dropper", 9, 0xFF_696969),
        "minecraft:brewing_stand" => ("Brewing Stand", 5, 0xFF_696969),
        "minecraft:chiseled_bookshelf" => ("Chiseled Bookshelf", 6, 0xFF_8B4513),
        "minecraft:decorated_pot" => ("Decorated Pot", 5, 0xFF_DEB887),
        "minecraft:lectern" => ("Lectern", 1, 0xFF_8B4513),
        _ if stack.item.starts_with("minecraft:") && stack.item.ends_with("_shulker_box") => {
            let mut color = 0xFF_404040;
            for (name, c) in SHULKER_COLORS {
                if &*stack.item == &*format!("minecraft:{}_shulker_box", name) {
                    color = *c;
                    break;
                }
            }
            ("Shulker Box", 27, color)
        }
        _ => return None,
    };

    let data = stack.data()?;
    let mut slots: Vec<Option<ItemStackPreview>> = Vec::with_capacity(max_slots.min(64));

    for (slot, inner) in &data.container {
        let s = usize::from(*slot);
        while slots.len() <= s {
            slots.push(None);
        }
        slots[s] = Some(ItemStackPreview {
            item_id: inner.item.to_string(),
            count: inner.count,
        });
    }
    if slots.is_empty() {
        for (i, inner) in data.bundle_contents.iter().enumerate() {
            while slots.len() <= i {
                slots.push(None);
            }
            slots[i] = Some(ItemStackPreview {
                item_id: inner.item.to_string(),
                count: inner.count,
            });
        }
    }

    // Fallback: vanilla BlockEntityTag item list in minecraft:block_entity_data.
    if slots.is_empty()
        && let Some(tag) = data.other("minecraft:block_entity_data")
        && let Tag::Compound(be) = tag
        && let Some(Tag::List(items)) = compound_get(be, "Items")
    {
        for item in items {
            let Tag::Compound(ic) = item else { continue };
            let Some(Tag::String(id)) = compound_get(ic, "id") else { continue };
            if id.to_string_lossy() == "minecraft:air" {
                continue;
            }
            let count = match compound_get(ic, "Count") {
                Some(Tag::Byte(b)) => u32::try_from(*b).unwrap_or(1),
                _ => 1,
            };
            let slot = match compound_get(ic, "Slot") {
                Some(Tag::Byte(b)) => usize::try_from(*b).unwrap_or(0),
                _ => 0,
            };
            while slots.len() <= slot {
                slots.push(None);
            }
            slots[slot] = Some(ItemStackPreview {
                item_id: id.to_string_lossy().to_string(),
                count,
            });
        }
    }

    while slots.last().is_some_and(|s| s.is_none()) {
        slots.pop();
    }

    Some(PreviewState {
        items: slots,
        title: title.to_string(),
        color,
    })
}

/// TEMP DEBUG: why extraction failed for a stack, as a short label.
pub fn extract_debug(stack: &ItemStack) -> String {
    let id = &*stack.item;
    let data = stack.data().is_some();
    let container_len = stack.data().map_or(0, |d| d.container.len());
    let other_keys = stack.data().map_or_else(String::new, |d| {
        d.other
            .iter()
            .map(|(k, _)| k.as_str())
            .collect::<Vec<_>>()
            .join(",")
    });
    format!("id={id} data={data} container_slots={container_len} other=[{other_keys}]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_defaults() {
        let s = Settings::read(|_| None);
        assert_eq!(s.corner, 1);
        assert!(s.show_key_hints);
        assert!(!s.compact_mode);
    }

    #[test]
    fn test_settings_click_cycle() {
        let mut s = Settings::read(|_| None);
        s.click(0);
        assert_eq!(s.corner, 2);
        s.click(1);
        assert!(!s.show_key_hints);
        s.click(2);
        assert!(s.compact_mode);
    }

    #[test]
    fn test_shulker_colors_count() {
        assert_eq!(SHULKER_COLORS.len(), 16);
    }
}

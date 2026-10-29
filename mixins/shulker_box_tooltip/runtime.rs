use std::cell::Cell;
use horizon_client::items::ItemStack;
use horizon_world::nbt::Tag;

pub const ID: &str = "shulker_box_tooltip";
pub const NAME: &str = "ShulkerBoxTooltip";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Preview shulker boxes and containers in a corner HUD. Press the preview key (default Shift) while holding a container.";

pub const BACKGROUND: u32 = 0x7800_0000;
pub const INK: u32 = 0xFF_FFFF;
pub const SLOT_SIZE: f32 = 18.0;
pub const SLOT_PADDING: f32 = 1.0;
pub const MAX_COLS: usize = 9;

const CONTAINER_TYPES: &[(&str, usize, u32)] = &[
    ("minecraft:shulker_box", 27, 0xFF_404040),
    ("minecraft:chest", 27, 0xFF_8B4513),
    ("minecraft:trapped_chest", 27, 0xFF_8B4513),
    ("minecraft:barrel", 27, 0xFF_8B4513),
    ("minecraft:ender_chest", 27, 0xFF_800080),
    ("minecraft:bundle", 64, 0xFF_D2691E),
    ("minecraft:furnace", 3, 0xFF_696969),
    ("minecraft:blast_furnace", 3, 0xFF_696969),
    ("minecraft:smoker", 3, 0xFF_696969),
    ("minecraft:hopper", 5, 0xFF_696969),
    ("minecraft:dispenser", 9, 0xFF_696969),
    ("minecraft:dropper", 9, 0xFF_696969),
    ("minecraft:brewing_stand", 5, 0xFF_696969),
    ("minecraft:chiseled_bookshelf", 6, 0xFF_8B4513),
    ("minecraft:decorated_pot", 5, 0xFF_DEB887),
    ("minecraft:lectern", 1, 0xFF_8B4513),
];

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

thread_local! {
    static SETTINGS: Cell<Option<Settings>> = const { Cell::new(None) };
    static PREVIEW_STATE: Cell<Option<PreviewState>> = const { Cell::new(None) };
    static PREVIEW_KEY_PRESSED: Cell<bool> = const { Cell::new(false) };
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
    pub components: String,
}

impl PreviewState {
    pub fn empty() -> Self {
        Self { items: vec![], title: String::new(), color: 0xFF_404040 }
    }

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

pub fn is_preview_key_pressed(game: &crate::game::Game<'_>) -> bool {
    let key = preview_key(game);
    if key == "key.keyboard.left.shift" {
        game.native_mapping("key.sneak").map(|(d, _)| d).unwrap_or(false)
    } else {
        game.native_mapping(&key).map(|(d, _)| d).unwrap_or(false)
    }
}

pub fn extract_container_contents(stack: &ItemStack) -> Option<PreviewState> {
    let (title, max_slots, color) = match stack.item.as_str() {
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
        _ => return None,
    };

    let mut final_color = color;
    if stack.item.starts_with("minecraft:") && stack.item.ends_with("_shulker_box") {
        for (name, c) in SHULKER_COLORS {
            if stack.item == format!("minecraft:{}_shulker_box", name) {
                final_color = *c;
                break;
            }
        }
    }

    let data = stack.data()?;
    let container_tag = data.other("minecraft:container")?;
    let Tag::List(items) = container_tag else { return None };

    let mut preview_items = Vec::with_capacity(max_slots);
    for tag in items {
        if let Tag::Compound(comp) = tag {
            let item_id = comp.get("id")
                .and_then(|v| v.as_string())
                .unwrap_or_default();
            let count = comp.get("count")
                .and_then(|v| v.as_i64())
                .unwrap_or(1) as u32;
            let slot = comp.get("slot")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as usize;

            while preview_items.len() <= slot {
                preview_items.push(None);
            }

            if !item_id.is_empty() && item_id != "minecraft:air" {
                let components = comp.iter()
                    .filter(|(k, _)| *k != "id" && *k != "count" && *k != "slot")
                    .map(|(k, _)| k.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");

                preview_items[slot] = Some(ItemStackPreview {
                    item_id,
                    count,
                    components,
                });
            }
        }
    }

    while preview_items.last().is_some_and(|s| s.is_none()) {
        preview_items.pop();
    }

    Some(PreviewState {
        items: preview_items,
        title: title.to_string(),
        color: final_color,
    })
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
}
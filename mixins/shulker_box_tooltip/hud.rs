fn shulker_box_tooltip_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::shulker_box_tooltip as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) {
        return;
    }

    if game.native_player().is_none() {
        m::set_preview_state(None);
        return;
    }

    let settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };

    let pressed = m::preview_key_pressed();

    if !pressed {
        m::set_preview_state(None);
        return;
    }

    let held_stack = match game.inventory.get(game.selected).and_then(|s| s.as_ref()) {
        Some(s) => s,
        None => return,
    };

    let preview = match m::extract_container_contents(held_stack) {
        Some(p) if !p.items.is_empty() => p,
        _ => {
            m::set_preview_state(None);
            return;
        }
    };

    m::set_preview_state(Some(preview.clone()));

    let corner = match settings.corner {
        0 => Corner::TopLeft,
        1 => Corner::TopRight,
        2 => Corner::BottomLeft,
        _ => Corner::BottomRight,
    };
    if !game.native_corner_shown(corner) {
        return;
    }

    let (w, h) = preview.dimensions(&settings);
    let (x, y) = corners.place(corner, w, h);

    game.native_fill(batch, [x, y, w, h], preview.color | 0x7800_0000);

    let title_text = format!("{} ({} slots)", preview.title, preview.items.len());
    let title_w = game.native_text_width(&title_text) + 4.0;
    game.native_fill(batch, [x, y, title_w, 14.0], preview.color);
    game.native_text(batch, &title_text, [x + 2.0, y + 2.0], m::INK, true);

    let mut slot_idx = 0;
    let start_y = y + 16.0;

    for item_opt in &preview.items {
        if settings.compact_mode && item_opt.is_none() {
            continue;
        }

        let col = slot_idx % m::MAX_COLS;
        let row = slot_idx / m::MAX_COLS;
        let sx = x + 7.0 + col as f32 * (m::SLOT_SIZE + m::SLOT_PADDING);
        let sy = start_y + 2.0 + row as f32 * (m::SLOT_SIZE + m::SLOT_PADDING);

        game.native_fill(batch, [sx, sy, m::SLOT_SIZE, m::SLOT_SIZE], 0x40_000000);

        if let Some(item) = item_opt {
            let display = if item.count > 1 {
                format!("{} x{}", item.item_id.strip_prefix("minecraft:").unwrap_or(&item.item_id), item.count)
            } else {
                item.item_id.strip_prefix("minecraft:").unwrap_or(&item.item_id).to_string()
            };
            let text_w = game.native_text_width(&display);
            let tx = sx + (m::SLOT_SIZE - text_w) / 2.0;
            let ty = sy + (m::SLOT_SIZE - 10.0) / 2.0;
            game.native_text(batch, &display, [tx, ty], m::INK, true);
        }

        slot_idx += 1;
    }

    if settings.show_key_hints {
        let hint = "Release key to close";
        let hint_w = game.native_text_width(hint) + 4.0;
        let hx = x + (w - hint_w) / 2.0;
        let hy = y + h + 4.0;
        game.native_fill(batch, [hx - 2.0, hy - 1.0, hint_w, 12.0], 0x78_000000);
        game.native_text(batch, hint, [hx, hy], 0xFF_AAAAAA, true);
    }
}
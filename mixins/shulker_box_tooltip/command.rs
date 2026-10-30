fn shulker_box_tooltip_command(
    game: &mut crate::game::Game<'_>,
    command: &str,
    ran: &mut bool,
) {
    use crate::game::shulker_box_tooltip as m;
    // TEMP DEBUG: proves the hook receives the command.
    game.native_chat(&format!("[SBT] command: /{command}"));
    let Some(args) = command.strip_prefix("shulkerpreview") else {
        return;
    };
    if !args.is_empty() && !args.starts_with(' ') {
        return;
    }

    if !game.native_on(m::ID) {
        return;
    }
    *ran = true;

    // /shulkerpreview force: toggle the corner preview without holding a key.
    if args.trim() == "force" {
        let on = !m::preview_forced();
        m::set_preview_forced(on);
        game.native_chat(&format!("Preview forced: {}", if on { "ON" } else { "OFF" }));
        return;
    }

    let held = match game.inventory.get(game.selected).and_then(|s| s.as_ref()) {
        Some(s) => s,
        None => {
            game.native_chat("No item in hand");
            return;
        }
    };

    match m::extract_container_contents(held) {
        Some(preview) => {
            if preview.items.is_empty() {
                game.native_chat(&format!("{} is empty", preview.title));
            } else {
                let count = preview.items.iter().filter(|i| i.is_some()).count();
                let items: Vec<String> = preview.items.iter()
                    .filter_map(|i| i.as_ref())
                    .map(|i| {
                        let name = i.item_id.strip_prefix("minecraft:").unwrap_or(&i.item_id);
                        if i.count > 1 { format!("{} x{}", name, i.count) } else { name.to_string() }
                    })
                    .collect();
                game.native_chat(&format!("{}: {} - [{}]", preview.title, count, items.join(", ")));
            }
        }
        None => {
            game.native_chat(&format!("{} is not a container", held.item));
        }
    }
}
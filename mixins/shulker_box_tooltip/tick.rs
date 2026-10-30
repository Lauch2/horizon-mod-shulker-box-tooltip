fn shulker_box_tooltip_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::shulker_box_tooltip as m;
    if !game.native_on(m::ID) {
        return;
    }
    if !game.native_window() {
        // Single player's server game: the authoritative inventory lives here
        // (the client sync does not carry minecraft:container). Push the
        // preview of the held stack for the window's HUD to read.
        let held = game.inventory.get(game.selected).and_then(|s| s.as_ref());
        let preview = held.and_then(m::extract_container_contents);
        m::server_push(preview);
        return;
    }
    // Window game: debug heartbeat.
    if m::debug_tick() {
        game.native_chat("[SBT] tick");
    }
}
fn shulker_box_tooltip_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::shulker_box_tooltip as m;
    if !game.native_on(m::ID) {
        return;
    }
    if !game.native_window() {
        // Single player's server game: the authoritative inventory lives here
        // (the client sync does not carry minecraft:container). Extract the
        // preview of the stack in the window's selected slot.
        let sel = m::window_selected();
        let held = game.inventory.get(sel).and_then(|s| s.as_ref());
        let preview = held.and_then(m::extract_container_contents);
        m::server_push(preview);
        return;
    }
    // Window game: tell the server which hotbar slot is selected, so the
    // preview follows slot changes instead of sticking to the first one.
    m::set_window_selected(game.selected);
}
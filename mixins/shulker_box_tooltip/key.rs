fn shulker_box_tooltip_key(
    game: &mut crate::game::Game<'_>,
    key: &str,
    pressed: bool,
) {
    use crate::game::shulker_box_tooltip as m;
    if !game.native_on(m::ID) {
        return;
    }

    let preview_key = m::preview_key(game);
    if key == preview_key {
        // TEMP DEBUG: proves the key hook receives the preview key.
        game.native_chat(&format!(
            "[SBT] key {}",
            if pressed { "down" } else { "up" }
        ));
        m::set_preview_key_pressed(pressed);
    }
}
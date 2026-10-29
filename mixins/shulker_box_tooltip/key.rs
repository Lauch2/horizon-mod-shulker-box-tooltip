use crate::game::shulker_box_tooltip as m;

fn shulker_box_tooltip_key(
    game: &mut crate::game::Game<'_>,
    key: &str,
    pressed: bool,
) {
    if !game.native_on(m::ID) {
        return;
    }

    let preview_key = m::preview_key(game);
    if key == preview_key {
        m::PREVIEW_KEY_PRESSED.with(|s| s.set(pressed));
    }
}
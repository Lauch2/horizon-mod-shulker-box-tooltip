fn shulker_box_tooltip_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::shulker_box_tooltip as m;
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    // TEMP DEBUG: one chat line every 100 ticks (~5 s) while in a world.
    if m::debug_tick() {
        game.native_chat("[SBT] tick");
    }
}
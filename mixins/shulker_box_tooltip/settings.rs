use crate::game::shulker_box_tooltip as m;

pub fn shulker_box_tooltip_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    if mod_id != m::ID {
        return;
    }

    let settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };

    for label in settings.labels() {
        buttons.push(label);
    }
}
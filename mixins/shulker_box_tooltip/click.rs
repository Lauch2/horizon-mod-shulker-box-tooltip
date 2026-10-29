use crate::game::shulker_box_tooltip as m;

pub fn shulker_box_tooltip_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    if mod_id != m::ID {
        return;
    }

    let mut settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };

    settings.click(button);

    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }

    m::set_settings(settings);
}
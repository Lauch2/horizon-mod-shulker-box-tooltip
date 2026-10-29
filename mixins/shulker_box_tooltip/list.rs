use crate::game::shulker_box_tooltip as m;

pub fn shulker_box_tooltip_listed(
    mods: &mut Vec<crate::game::native_hooks::NativeMod>,
) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![
            ["preview".into(), "Preview key".into(), "key.keyboard.left.shift".into()],
        ],
    });
}
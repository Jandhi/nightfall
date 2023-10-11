use bevy::prelude::*;

#[derive(Resource)]
pub struct Palette {
    pub dark: Color,
    pub red: Color,
    pub orange: Color,
    pub white: Color,
}

pub const DARK_HEX: &str = "#171726";
pub const RED_HEX: &str = "#804055";
pub const ORANGE_HEX: &str = "#d99d62";
pub const WHITE_HEX: &str = "#fff2d9";

impl Palette {
    fn new() -> Self {
        Self {
            dark: Color::hex(DARK_HEX).unwrap(),
            red: Color::hex(RED_HEX).unwrap(),
            orange: Color::hex(ORANGE_HEX).unwrap(),
            white: Color::hex(WHITE_HEX).unwrap(),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

//PLUGIN
pub struct PalettePlugin;

impl Plugin for PalettePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Palette::new());
    }
}

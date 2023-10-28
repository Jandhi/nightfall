use bevy::prelude::Plugin;

use self::rng::GlobalSeed;

pub mod radians;
pub mod rng;


pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GlobalSeed("test".into()));
    }
}
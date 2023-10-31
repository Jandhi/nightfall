use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Team {
    None,
    Player,
    Enemy,
}

#[derive(Component)]
pub struct TeamMember {
    pub team: Team,
}

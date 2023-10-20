use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Team {
    Player,
    Enemy
}

#[derive(Component)]
pub struct TeamMember {
    pub team : Team
}
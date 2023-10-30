use bevy::prelude::*;

use crate::loading::AbilityTextures;



#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Ability {
    BigBullets,
    Crossbow,
    DoubleBarrel,
    FlamingBullets,
    Shells,
    Sniper,
}

impl Ability {
    pub fn all() -> Vec<Ability> {
        vec![
            Self::BigBullets, 
            Self::Crossbow,
            Self::DoubleBarrel,
            Self::FlamingBullets,
            Self::Shells,
            Self::Sniper
        ]
    }

    pub fn get_texture(&self, textures : &Res<AbilityTextures>) -> Handle<Image> {
        match self {
            Ability::BigBullets => textures.big_bullets.clone(),
            Ability::Crossbow => textures.crossbow.clone(),
            Ability::DoubleBarrel => textures.double_barrel.clone(),
            Ability::FlamingBullets => textures.flaming_bullets.clone(),
            Ability::Shells => textures.shells.clone(),
            Ability::Sniper => textures.sniper.clone(),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Ability::BigBullets => "Big Bullets",
            Ability::Crossbow => "Crossbow",
            Ability::DoubleBarrel => "Double Barrel",
            Ability::FlamingBullets => "Flaming Bullets",
            Ability::Shells => "Shells",
            Ability::Sniper => "Sniper",
        }
    }
}


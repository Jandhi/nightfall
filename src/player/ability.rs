use bevy::{prelude::*, utils::HashSet};

use crate::{loading::AbilityTextures, combat::health::HealthType};



#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Ability {
    BigBullets,
    Crossbow,
    DoubleBarrel,
    TripleBarrel,
    FlamingBullets,
    Shells,
    Sniper,
    Shotgun,
    MegaShotgun,
}

impl Ability {
    pub fn all() -> Vec<Ability> {
        vec![
            Self::BigBullets, 
            Self::Crossbow,
            Self::DoubleBarrel,
            Self::TripleBarrel,
            Self::FlamingBullets,
            Self::Shells,
            Self::Sniper,
            Self::Shotgun,
            Self::MegaShotgun,
        ]
    }

    pub fn get_texture(&self, textures : &Res<AbilityTextures>) -> Handle<Image> {
        match self {
            Ability::BigBullets => textures.big_bullets.clone(),
            Ability::Crossbow => textures.crossbow.clone(),
            Ability::DoubleBarrel => textures.double_barrel.clone(),
            Ability::TripleBarrel => textures.triple_barrel.clone(),
            Ability::FlamingBullets => textures.flaming_bullets.clone(),
            Ability::Shells => textures.shells.clone(),
            Ability::Sniper => textures.sniper.clone(),
            Ability::Shotgun => textures.shotgun.clone(),
            Ability::MegaShotgun => textures.mega_shotgun.clone(),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Ability::BigBullets => "Big Bullets",
            Ability::Crossbow => "Crossbow",
            Ability::DoubleBarrel => "Double Barrel",
            Ability::TripleBarrel => "Triple Barrel",
            Ability::FlamingBullets => "Flaming Bullets",
            Ability::Shells => "Shells",
            Ability::Sniper => "Sniper",
            Ability::Shotgun => "Shotgun",
            Ability::MegaShotgun => "Mega Shotgun",
        }
    }

    pub fn is_available(&self, player_abilities : &Vec<Ability>) -> bool {
        match self {
            Ability::BigBullets => !player_abilities.contains(&Ability::BigBullets),
            Ability::Crossbow => !player_abilities.contains(&Ability::Crossbow),
            Ability::DoubleBarrel => !player_abilities.contains(&Ability::DoubleBarrel),
            Ability::TripleBarrel => {
                !player_abilities.contains(&Ability::TripleBarrel)
                    && player_abilities.contains(&Ability::DoubleBarrel)
            },
            Ability::FlamingBullets => !player_abilities.contains(&Ability::FlamingBullets),
            Ability::Shells => !player_abilities.contains(&Ability::Shells),
            Ability::Sniper => !player_abilities.contains(&Ability::Sniper),
            Ability::Shotgun => {
                !player_abilities.contains(&Ability::Shotgun)
                    && player_abilities.contains(&Ability::TripleBarrel)
            },
            Ability::MegaShotgun => {
                !player_abilities.contains(&Ability::MegaShotgun)
                    && player_abilities.contains(&Ability::Shotgun)
            },
        }
    }

    pub fn damage_mult(&self) -> f32 {
        match self {
            Ability::DoubleBarrel => 0.7,
            Ability::TripleBarrel => 0.9,
            Ability::Shotgun => 0.9,
            Ability::BigBullets => 2.0,
            _ => 1.
        }
    }

    pub fn knockback_mult(&self) -> f32 {
        match self {
            Ability::BigBullets => 2.0,
            _ => 1.
        }
    }

    pub fn reload_mult(&self) -> f32 {
        match self {
            _ => 1.
        }
    }

    pub fn shoot_speed_mult(&self) -> f32 {
        match self {
            Ability::BigBullets => 0.7,
            _ => 1.
        }
    }
}

impl Into<String> for &Ability {
    fn into(self) -> String {
        String::from(self.get_name())
    }
}


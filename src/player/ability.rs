use bevy::prelude::*;

use crate::loading::AbilityTextures;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Ability {
    BigBullets,
    BiggestBullets,
    BulletsGalore,
    Crossbow,
    DoubleBarrel,
    Faster,
    FlamingBullets,
    HotterFire,
    MaxHp,
    MediumBullets,
    MegaShotgun,
    Reload,
    Shells,
    ShootingSpeed,
    Shotgun,
    Sixfold,
    Sniper,
    Thorns,
    TripleBarrel,
    Potion,
}

impl Ability {
    pub fn all() -> Vec<Ability> {
        vec![
            Self::BigBullets,
            Self::BiggestBullets,
            Self::BulletsGalore,
            Self::Crossbow,
            Self::DoubleBarrel,
            Self::Faster,
            Self::FlamingBullets,
            Self::HotterFire,
            Self::MediumBullets,
            Self::MegaShotgun,
            Self::Reload,
            Self::Shells,
            Self::ShootingSpeed,
            Self::Shotgun,
            Self::Sixfold,
            Self::Sniper,
            Self::Thorns,
            Self::TripleBarrel,
            Self::MaxHp,
            Self::Potion,
        ]
    }

    pub fn get_texture(&self, textures: &Res<AbilityTextures>) -> Handle<Image> {
        match self {
            Ability::BigBullets => textures.big_bullets.clone(),
            Ability::BiggestBullets => textures.biggest_bullets.clone(),
            Ability::Crossbow => textures.crossbow.clone(),
            Ability::DoubleBarrel => textures.double_barrel.clone(),
            Ability::TripleBarrel => textures.triple_barrel.clone(),
            Ability::FlamingBullets => textures.flaming_bullets.clone(),
            Ability::Shells => textures.shells.clone(),
            Ability::Sniper => textures.sniper.clone(),
            Ability::Shotgun => textures.shotgun.clone(),
            Ability::MegaShotgun => textures.mega_shotgun.clone(),
            Ability::BulletsGalore => textures.bullets_galore.clone(),
            Ability::Faster => textures.faster.clone(),
            Ability::HotterFire => textures.hotter_fire.clone(),
            Ability::MediumBullets => textures.medium_bullets.clone(),
            Ability::Reload => textures.reload.clone(),
            Ability::ShootingSpeed => textures.shooting_speed.clone(),
            Ability::Sixfold => textures.sixfold.clone(),
            Ability::Thorns => textures.thorns.clone(),
            Ability::MaxHp => textures.max_hp.clone(),
            Ability::Potion => textures.potion.clone(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Ability::BigBullets => "Bigger Bullets",
            Ability::BiggestBullets => "Biggest Bullets",
            Ability::BulletsGalore => "Bullets Galore",
            Ability::Crossbow => "Crossbow",
            Ability::DoubleBarrel => "Double Barrel",
            Ability::Faster => "Faster",
            Ability::FlamingBullets => "Flaming Bullets",
            Ability::HotterFire => "Hotter Fire",
            Ability::MaxHp => "Hearty",
            Ability::MediumBullets => "Big Bullets",
            Ability::MegaShotgun => "Mega Shotgun",
            Ability::Reload => "Reload",
            Ability::Shells => "Shell",
            Ability::ShootingSpeed => "Quick Chamber",
            Ability::Shotgun => "Shotgun",
            Ability::Sixfold => "Sixfold",
            Ability::Sniper => "Sniper",
            Ability::Thorns => "Thorns",
            Ability::TripleBarrel => "Triple Barrel",
            Ability::Potion => "Potion",
        }
        .to_string()
    }

    pub fn get_description(&self) -> String {
        match self {
            Ability::BigBullets => "x2 Damage\n +50% Knockback\n -20% Shoot Speed\n -20% Reload Speed\n -20% Bullet Speed",
            Ability::BiggestBullets => "x2 Damage\n +50% Knockback\n -20% Shoot Speed\n -20% Reload Speed\n -20% Bullet Speed",
            Ability::BulletsGalore => "+3 Max Ammo",
            Ability::Crossbow => "Piercing Bullets",
            Ability::DoubleBarrel => "2 Bullets\n-30% Shoot Speed",
            Ability::Faster => "+50 Move Speed",
            Ability::FlamingBullets => "2 damage every 2 seconds",
            Ability::HotterFire => "+2 fire damage",
            Ability::MaxHp => "+1 Max HP",
            Ability::MediumBullets => "x2 Damage\n +50% Knockback\n -20% Shoot Speed\n -20% Reload Speed",
            Ability::MegaShotgun => "7 Bullets",
            Ability::Reload => "+75% Reload Speed",
            Ability::Shells => "+50% Dmg",
            Ability::ShootingSpeed => "+40% Shoot Speed",
            Ability::Shotgun => "5 Bullets\n-10% Shoot Speed",
            Ability::Sixfold => "Shoot 6 bullets on reload",
            Ability::Sniper => "x2 Bullet Speed",
            Ability::Thorns => "Kills Enemy that you touch",
            Ability::TripleBarrel => "3 Bullets\n-10% Shoot Speed",
            Ability::Potion => "Heal 2 hearts",
        }.to_string()
    }

    pub fn is_available(&self, player_abilities: &Vec<Ability>) -> bool {
        match self {
            Ability::BigBullets => {
                !player_abilities.contains(&Ability::BigBullets)
                    && player_abilities.contains(&Ability::MediumBullets)
            }
            Ability::BiggestBullets => {
                !player_abilities.contains(&Ability::BiggestBullets)
                    && player_abilities.contains(&Ability::BigBullets)
            }
            Ability::Crossbow => !player_abilities.contains(&Ability::Crossbow),
            Ability::DoubleBarrel => !player_abilities.contains(&Ability::DoubleBarrel),
            Ability::TripleBarrel => {
                !player_abilities.contains(&Ability::TripleBarrel)
                    && player_abilities.contains(&Ability::DoubleBarrel)
            }
            Ability::FlamingBullets => !player_abilities.contains(&Ability::FlamingBullets),
            Ability::Shells => !player_abilities.contains(&Ability::Shells),
            Ability::Sniper => !player_abilities.contains(&Ability::Sniper),
            Ability::Shotgun => {
                !player_abilities.contains(&Ability::Shotgun)
                    && player_abilities.contains(&Ability::TripleBarrel)
            }
            Ability::MegaShotgun => {
                !player_abilities.contains(&Ability::MegaShotgun)
                    && player_abilities.contains(&Ability::Shotgun)
            }
            Ability::BulletsGalore => true,
            Ability::Faster => true,
            Ability::HotterFire => player_abilities.contains(&Ability::FlamingBullets),
            Ability::MediumBullets => !player_abilities.contains(&Ability::MediumBullets),
            Ability::Reload => true,
            Ability::ShootingSpeed => true,
            Ability::Sixfold => !player_abilities.contains(&Ability::Sixfold),
            Ability::Thorns => !player_abilities.contains(&Ability::Thorns),
            Ability::MaxHp => true,
            Ability::Potion => true,
        }
    }

    pub fn damage_mult(&self) -> f32 {
        match self {
            Ability::MediumBullets => 2.0,
            Ability::BigBullets => 2.0,
            Ability::BiggestBullets => 2.0,
            Ability::Shells => 1.5,
            _ => 1.,
        }
    }

    pub fn knockback_mult(&self) -> f32 {
        match self {
            Ability::MediumBullets => 1.5,
            Ability::BigBullets => 1.5,
            Ability::BiggestBullets => 1.5,
            _ => 1.,
        }
    }

    pub fn reload_mult(&self) -> f32 {
        match self {
            Ability::MediumBullets => 0.8,
            Ability::BigBullets => 0.8,
            Ability::BiggestBullets => 0.8,
            Ability::Reload => 1.75,
            _ => 1.,
        }
    }

    pub fn shoot_speed_mult(&self) -> f32 {
        match self {
            Ability::DoubleBarrel => 0.7,
            Ability::TripleBarrel => 0.9,
            Ability::Shotgun => 0.9,
            Ability::MediumBullets => 0.8,
            Ability::BigBullets => 0.8,
            Ability::BiggestBullets => 0.8,
            Ability::ShootingSpeed => 1.4,
            _ => 1.,
        }
    }
}

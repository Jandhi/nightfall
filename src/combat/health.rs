use bevy::prelude::*;

pub type HealthType = u32;

#[derive(Component, Clone)]
pub struct Health {
    pub value: HealthType,
    pub max: HealthType,
    pub is_invincible: bool,
}

#[derive(Event)]
pub struct TookDamageEvent {
    pub entity: Entity,
    pub amount: HealthType,
}

impl Health {
    pub fn new(max: HealthType) -> Health {
        Health {
            value: max,
            max,
            is_invincible: false,
        }
    }

    pub fn take_damage(
        &mut self,
        my_entity: Entity,
        took_damage_ev: &mut EventWriter<TookDamageEvent>,
        dmg: HealthType,
    ) {
        if self.is_invincible {
            return;
        }

        if dmg > self.value {
            self.value = 0;
        } else {
            self.value -= dmg;
        }

        took_damage_ev.send(TookDamageEvent {
            entity: my_entity,
            amount: dmg,
        });
    }

    pub fn is_alive(&self) -> bool {
        self.value > 0
    }
}

#[derive(Component)]
pub struct Dead;

#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
}

pub fn check_death(
    q_health: Query<(Entity, &Health), Without<Dead>>,
    mut death_ev: EventWriter<DeathEvent>,
    _commands: Commands,
) {
    for (entity, health) in q_health.iter() {
        if !health.is_alive() {
            death_ev.send(DeathEvent { entity });
        }
    }
}

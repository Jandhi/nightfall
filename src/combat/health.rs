use bevy::prelude::*;

pub type HealthType = u32;

#[derive(Component, Clone)]
pub struct Health {
    pub value: HealthType,
    pub max: HealthType,
}

impl Health {
    pub fn new(max: HealthType) -> Health {
        Health { value: max, max }
    }

    pub fn take_damage(&mut self, dmg: HealthType) {
        if dmg > self.value {
            self.value = 0;
        } else {
            self.value -= dmg;
        }
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

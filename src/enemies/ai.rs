use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    movement::{pause::ActionPauseState, velocity::Velocity},
    player::Player,
    util::radians::Radian,
};



#[derive(Component)]
pub struct FollowPlayerAI {
    // The desired speed
    pub speed: f32,

    // How quick the velocity corrects
    pub corrective_force: f32,
}

pub fn follow_player(
    mut q_enemies: Query<(&Transform, &FollowPlayerAI, &mut Velocity)>,
    q_player: Query<&Transform, (With<Player>, Without<FollowPlayerAI>)>,
    pause: Res<ActionPauseState>,
) {
    if pause.is_paused {
        return;
    }

    let player_transform = q_player.single();

    for (transform, ai, mut velocity) in q_enemies.iter_mut() {
        let direction = player_transform.translation.truncate() - transform.translation.truncate();
        // obtain angle to target with respect to x-axis.
        let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
        let direction_vec = angle_to_target.unit_vector();

        let desired_velocity = direction_vec * ai.speed;
        let diff = desired_velocity - velocity.vec;

        if diff.length() < ai.corrective_force {
            velocity.vec = desired_velocity;
        } else {
            velocity.vec += diff.normalize() * ai.corrective_force;
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum MoveAndShootAIState {
    Move,
    Slow,
    Charge,
}

#[derive(Component)]
pub struct MoveAndShootAI {
    state: MoveAndShootAIState,

    // The desired speed
    pub speed: f32,

    // How quick the velocity corrects
    pub corrective_force: f32,

    // The desired shoot distance
    pub shoot_distance: f32,

    // Shoot refresh
    pub charge_timer: Timer,
    pub refresh_timer: Timer,
}

impl MoveAndShootAI {
    pub fn new(
        speed: f32,
        corrective_force: f32,
        shoot_distance: f32,
        charge_time: f32,
        refresh_time: f32,
    ) -> MoveAndShootAI {
        MoveAndShootAI {
            state: MoveAndShootAIState::Move,
            speed,
            corrective_force,
            shoot_distance,
            charge_timer: Timer::from_seconds(charge_time, TimerMode::Once),
            refresh_timer: Timer::from_seconds(refresh_time, TimerMode::Once),
        }
    }
}

#[derive(Event)]
pub struct ChargeShootEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct ShootEvent {
    pub entity: Entity,
    pub target: Entity,
}

pub fn move_and_shoot_ai(
    mut q_enemies: Query<(Entity, &Transform, &mut MoveAndShootAI, &mut Velocity)>,
    q_player: Query<(Entity, &Transform), (With<Player>, Without<FollowPlayerAI>)>,
    mut charge_ev: EventWriter<ChargeShootEvent>,
    mut shoot_ev: EventWriter<ShootEvent>,
    pause: Res<ActionPauseState>,
    time: Res<Time>,
) {
    if pause.is_paused {
        return;
    }

    let (player_entity, player_transform) = q_player.single();

    for (entity, transform, mut ai, mut velocity) in q_enemies.iter_mut() {
        ai.charge_timer.tick(time.delta());
        ai.refresh_timer.tick(time.delta());

        if ai.state == MoveAndShootAIState::Move
            && (player_transform.translation.distance(transform.translation) <= ai.shoot_distance)
            && ai.refresh_timer.finished()
        {
            ai.state = MoveAndShootAIState::Slow;
        }

        if ai.state == MoveAndShootAIState::Slow && velocity.vec.length() < 0.05 {
            ai.state = MoveAndShootAIState::Charge;
            charge_ev.send(ChargeShootEvent { entity });
            ai.charge_timer.reset();
        }

        if ai.state == MoveAndShootAIState::Charge && ai.charge_timer.just_finished() {
            ai.state = MoveAndShootAIState::Move;
            ai.refresh_timer.reset();
            shoot_ev.send(ShootEvent {
                entity: entity,
                target: player_entity,
            })
        }

        if ai.state != MoveAndShootAIState::Slow && ai.state != MoveAndShootAIState::Move {
            continue;
        }

        let direction = player_transform.translation.truncate() - transform.translation.truncate();
        // obtain angle to target with respect to x-axis.
        let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
        let direction_vec = angle_to_target.unit_vector();

        let desired_velocity = match ai.state {
            MoveAndShootAIState::Move => match direction.length() > ai.shoot_distance {
                true => direction_vec * ai.speed,
                false => Vec2::ZERO,
            },
            _ => Vec2::ZERO,
        };
        let diff = desired_velocity - velocity.vec;

        if diff.length() < ai.corrective_force {
            velocity.vec = desired_velocity;
        } else {
            velocity.vec += diff.normalize() * ai.corrective_force;
        }
    }
}

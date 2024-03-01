use crate::movement::character_controller::{AnimationState, CharacterAnimations};
use bevy::{animation::AnimationPlayer, prelude::*};
use bevy_gltf_blueprints::{AnimationPlayerLink, Animations};
use bevy_mod_sysfail::prelude::*;
use bevy_tnua::{
    builtins::TnuaBuiltinWalk, controller::TnuaController, TnuaAnimatingState,
    TnuaAnimatingStateDirective,
};
use std::time::Duration;

#[sysfail(Log<anyhow::Error, Error>)]
pub(crate) fn play_animations(
    mut query: Query<(
        &mut TnuaAnimatingState<AnimationState>,
        &TnuaController,
        &CharacterAnimations,
        &AnimationPlayerLink,
        &Animations,
    )>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("play_animations").entered();
    for (mut animating_state, controller, animation_names, link, animations) in query.iter_mut() {
        let mut animation_player = animation_players.get_mut(link.0)?;
        match animating_state.update_by_discriminant({
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                continue;
            };
            let speed = basis_state.running_velocity.length();
            if controller.is_airborne()? {
                AnimationState::Airborne
            } else if speed > 10.0 {
                AnimationState::Running(speed)
            } else if speed > 0.01 {
                AnimationState::Walking(speed)
            } else {
                AnimationState::Standing
            }
        }) {
            TnuaAnimatingStateDirective::Maintain { state } => {
                if let AnimationState::Running(speed) = state {
                    let anim_speed = (speed / 7.0).max(1.0);
                    animation_player.set_speed(anim_speed);
                }
            }
            TnuaAnimatingStateDirective::Alter {
                // We don't need the old state here, but it's available for transition
                // animations.
                old_state: _,
                state,
            } => match state {
                AnimationState::Airborne | AnimationState::Running(..) => {
                    animation_player
                        .play_with_transition(
                            animations
                                .named_animations
                                .get(&animation_names.aerial)
                                .unwrap()
                                .clone_weak(),
                            Duration::from_secs_f32(0.2),
                        )
                        .repeat();
                }
                AnimationState::Standing => {
                    animation_player
                        .play_with_transition(
                            animations
                                .named_animations
                                .get(&animation_names.idle)
                                .unwrap()
                                .clone_weak(),
                            Duration::from_secs_f32(0.2),
                        )
                        .repeat();
                }
                AnimationState::Walking(_speed) => {
                    animation_player
                        .play_with_transition(
                            animations
                                .named_animations
                                .get(&animation_names.walk)
                                .unwrap()
                                .clone_weak(),
                            Duration::from_secs_f32(0.1),
                        )
                        .repeat();
                }
            },
        }
    }
}

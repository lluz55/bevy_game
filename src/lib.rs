#![feature(let_chains)]
// These two generate a lot of false positives for Bevy systems
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

//! Foxtrot is split into many plugins with their own set of responsibilities.
//! This is an organizational measure and not meant to be imply that you can turn them on or off at will,
//! since the plugins are interdependent.  
//! Instead, decide for yourself which features you like and which one's you don't and simply trim the code accordingly.
//! Feel free to [file an issue](https://github.com/janhohenheim/foxtrot/issues/new) if you need help!
//! The docs are organized such that you can click through the plugins to explore the systems at play.

#[cfg(feature = "dev")]
use crate::dev::dev_plugin;
use crate::{
    bevy_config::bevy_config_plugin, file_system_interaction::file_system_interaction_plugin,
    ingame_menu::ingame_menu_plugin, level_instantiation::level_instantiation_plugin,
    menu::menu_plugin, movement::movement_plugin, particles::particle_plugin,
    player_control::player_control_plugin, shader::shader_plugin,
    world_interaction::world_interaction_plugin,
};
use bevy::prelude::*;

pub(crate) mod bevy_config;
#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod file_system_interaction;
pub(crate) mod ingame_menu;
pub(crate) mod level_instantiation;
pub(crate) mod menu;
pub(crate) mod movement;
pub(crate) mod particles;
pub(crate) mod player_control;
pub(crate) mod shader;
pub(crate) mod util;
pub(crate) mod world_interaction;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the loading_plugin will load our assets
    #[default]
    Loading,
    /// During this State the actual game logic is executed
    Playing,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
}

/// Main entrypoint for Foxtrot.
///
/// The top-level plugins are:
/// - [`bevy_config_plugin`]: Sets up the bevy configuration.
/// - [`menu_plugin`]: Handles the menu.
/// - [`movement_plugin`]: Handles the movement of entities.
/// - [`player_control_plugin`]: Handles the player's control.
/// - [`world_interaction_plugin`]: Handles the interaction of entities with the world.
/// - [`level_instantiation_plugin`]: Handles the creation of levels and objects.
/// - [`file_system_interaction_plugin`]: Handles the loading and saving of games.
/// - [`shader_plugin`]: Handles the shaders.
/// - [`dev_plugin`]: Handles the dev tools.
/// - [`ingame_menu_plugin`]: Handles the ingame menu accessed via ESC.
/// - [`particle_plugin`]: Handles the particle system.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            bevy_config_plugin,
            menu_plugin,
            movement_plugin,
            player_control_plugin,
            world_interaction_plugin,
            level_instantiation_plugin,
            file_system_interaction_plugin,
            shader_plugin,
            ingame_menu_plugin,
            particle_plugin,
            #[cfg(feature = "dev")]
            dev_plugin,
        ));
    }
}

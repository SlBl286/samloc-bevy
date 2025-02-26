use bevy::prelude::*;
use components::*;
use samloc::despawn_screen;
use states::MenuState;
use systems::*;

use crate::states::AppStates;

mod systems;
pub mod components;
pub mod states;
pub mod enums;

pub struct MenuScreenPlugin;

impl Plugin for MenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app
        // At start, the menu is not enabled. This will be changed in `menu_setup` when
        // entering the `GameState::Menu` state.
        // Current screen in the menu is handled by an independent state from `GameState`
        .init_state::<MenuState>()
        .add_systems(OnEnter(AppStates::MainMenu), menu_setup)
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), (main_menu_setup,spawn_audio))
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        // Systems to handle the settings menu screen
        // .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        // Systems to handle the display settings screen
        // .add_systems(
        //     OnEnter(MenuState::SettingsDisplay),
        //     display_settings_menu_setup,
        // )
        // .add_systems(
        //     Update,
        //     (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
        // )
        .add_systems(
            OnExit(MenuState::SettingsDisplay),
            despawn_screen::<OnDisplaySettingsMenuScreen>,
        )
        // Systems to handle the sound settings screen
        // .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
        // .add_systems(
        //     Update,
        //     setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
        // )
        .add_systems(
            OnExit(MenuState::SettingsSound),
            despawn_screen::<OnSoundSettingsMenuScreen>,
        )
        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(AppStates::MainMenu)),
        );
    }
}


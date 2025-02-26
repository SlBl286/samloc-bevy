use bevy::ecs::component::Component;

#[derive(Component)]
pub struct OnMainMenuScreen;
// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
pub struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
pub struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
pub struct OnSoundSettingsMenuScreen;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub struct  ClickedAudio;
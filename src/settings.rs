use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
struct SettingsUi;

pub struct SettingsPlugin;

pub struct Settings {
    pub sfx_volume: f64,
    pub music_volume: f64,
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings { sfx_volume: 1.0, music_volume: 1.0 })
			.add_system_set(SystemSet::on_enter(GameState::Settings).with_system(load_ui));
    }
}

fn load_ui(mut commands: Commands) {
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				flex_direction: FlexDirection::ColumnReverse,
				..Default::default()
			},
			color: UiColor(Color::BLACK),
			..Default::default()
		})
		.insert(SettingsUi)
		.insert(Name::new("Ui"));
}
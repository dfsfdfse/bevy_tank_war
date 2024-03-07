use bevy::app::App;
use bevy_tank::plugins::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
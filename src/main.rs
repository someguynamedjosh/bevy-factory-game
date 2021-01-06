use bevy::prelude::*;

pub mod iso_coord;

fn hello_world() {
    println!("Hello world!");
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(hello_world.system())
        .run();
}

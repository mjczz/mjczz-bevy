use std::thread::sleep;
use std::time;
use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// add people to our World using a "startup system".
// startup systems are just like normal systems, but they run exactly once, before all other systems, right when our app starts.
// use Commands to spawn some entities into our World:
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// iterate over every Name component for entities that also have a Person component
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
    println!("");
}

// make query mutable, and use a mutable reference (&mut) to the components we want to change
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "New Name Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

fn hello_world() {
    println!("hello world! ..........");
    sleep(time::Duration::from_secs(1))
}

fn main() {
    App::new()
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .add_systems(Startup, add_people)
        .run();
}


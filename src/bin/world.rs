use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// add people to our World using a "startup system".
// startup systems are just like normal systems, but they run exactly once, before all other systems, right when our app starts.
// use Commands to spawn some entities into our World:
fn add_people(mut commands: Commands) {
    println!("adding people");
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// make query mutable, and use a mutable reference (&mut) to the components we want to change
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    println!("update people");
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "New Name Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

// customize some Resource
#[derive(Resource)]
struct CalledTimes(i32);

fn greet_people(
    mut time: ResMut<Time>,
    mut timer: ResMut<GreetTimer>,
    mut demo: ResMut<CalledTimes>,
    query: Query<&Name, With<Person>>,
) {
    // Update the timer with the elapsed time since the last frame
    time.advance_to(Duration::from_secs(3));
    loop {
        println!("{:?}", time.delta());
        if timer.0.tick(time.delta()).just_finished() {
            demo.0 += 1;
            for name in query.iter() {
                println!("hello {}! called {} times", name.0, demo.0);
            }
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum SimulationSet {
    Spawn,
    Age,
}

fn main() {
    let mut world = World::new();
    world.insert_resource(CalledTimes { 0: 0 });
    world.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    world.init_resource::<Time>();

    let mut schedule = Schedule::default();
    schedule.add_systems((
        add_people.in_set(SimulationSet::Spawn),
        (update_people, greet_people)
            .chain()
            .after(SimulationSet::Spawn),
    ));
    schedule.run(&mut world);
}

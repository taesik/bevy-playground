use bevy::app::{App, Plugin, Startup, Update};
use bevy::DefaultPlugins;
use bevy::ecs::component::Component;
use bevy::prelude::{Commands, IntoSystemConfigs, Query, Res, ResMut, Resource, With};
use bevy::time::{Time, Timer, TimerMode};

fn main() {
  App::new()
    .add_plugins((DefaultPlugins,HelloPlugin))
    .run();
}



#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
  commands
    .spawn((Person,Name("Brook_kidler".to_owned())));
  commands
    .spawn((Person,Name("Jen".to_owned())));
  commands
    .spawn((Person,Name("Sage".to_owned())));
}

fn greet_people(
  time: Res<Time>,
  mut timer: ResMut<GreetTimer>,
  query: Query<&Name, With<Person>>
) {
  if timer.0.tick(time.delta()).just_finished() {
    for name in &query {
      println!("hello {}!", name.0);
    }
  }
}

fn update_people(mut query: Query<&mut Name,With<Person>>) {
  for mut name in &mut query {
    if name.0.contains("_") {
      name.0 = name.0.replace("_"," _ ");
      break;
    }
  }
}


struct HelloPlugin;

impl Plugin for HelloPlugin{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GreetTimer(Timer::from_seconds(2.0,TimerMode::Once)))
      .add_systems(Startup,add_people)
      .add_systems(Update,(greet_people,update_people).chain())
    ;
  }
}

# [derive(Resource)]
struct GreetTimer(Timer);
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use specs::prelude::*;

use components::{SDLColor, Pos, PlayerControlled};

mod resources;
mod systems;
mod components;

fn main() {
    // SDL2 setup
    let sdl_context = sdl2::init().expect("Failed to create SDL2 Context");
    let video_subsystem = sdl_context.video().expect("Failed to create SDL2 Video subsystem");

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string()).expect("Failed to create window");

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("Failed to turn window into canvas");
    
    // specs setup
    let (mut game_state, mut dispatcher) = default_state_and_dispatcher();


    
    canvas.set_draw_color(Color::RED);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump.");

    // game loop
    'sdl_loop: loop {

        // forward all sdl events to specs resource
        if let Some(sdl_event_resource) = game_state.world.get_mut::<resources::SDLEventResource>() {
            sdl_event_resource.events = event_pump.poll_iter().collect();
        }

        dispatcher.dispatch(&game_state.world);
        
        // check exit resource if game should exit
        if let Some (exit_resource) = game_state.world.get_mut::<resources::ExitResource>() {
            if exit_resource.exit {
                break 'sdl_loop;
            }
        }
        
        // render code here
        canvas.set_draw_color(Color::RED); // reset draw color to clear color
        canvas.clear();

        let pos_storage = game_state.world.read_storage::<Pos>();
        let color_storage = game_state.world.read_storage::<SDLColor>();

        for (pos, color_component) in (&pos_storage, &color_storage).join() {
            let rect = Rect::new(pos.x as i32, pos.y as i32, 50, 50);

            canvas.set_draw_color(color_component.color);

            match canvas.draw_rect(rect) {
                Ok(()) => (()),
                Err(e) => println!("Failed to draw rect: {}", e),
            }
        }

        canvas.present();

        // replace this with more sophisticated FPS limiting
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // 60ish FPS
    }
    println!("Exiting game.")
}

            
pub struct GameState {
    pub world: World,
}

pub fn default_state_and_dispatcher<'a> () -> (GameState, Dispatcher<'a, 'a>) {
    let mut gs = GameState {
        world: World::new(),
    };

    // register components
    gs.world.register::<SDLColor>();
    gs.world.register::<Pos>();
    gs.world.register::<PlayerControlled>();
    
    // insert resources
    gs.world.insert(resources::ExitResource::default());

    // Player entity
    gs.world.create_entity()
        .with(components::Pos::new(200.0,200.0))
        .with(components::PlayerControlled{})
        .with(components::SDLColor::rgba(0,0,0,1))
        .build();

    // Build dispatcher, specify systems
    let mut dispatcher =
        DispatcherBuilder::new()
        .with(
            systems::HandleSDLEventSystem::new(&mut gs.world),
            "HandleSDLEventSystem",
            &[]
        )
        .build();

    dispatcher.setup(&mut gs.world);
    (gs, dispatcher)
}
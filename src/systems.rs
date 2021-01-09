use specs::prelude::*;

use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use crate::resources::{SDLEventResource, ExitResource};
use crate::components::{Pos, PlayerControlled};

const STEP: f64 = 5.0;

pub struct HandleSDLEventSystem {
}

impl HandleSDLEventSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        Self { }
    }
}
impl <'a> System<'a> for HandleSDLEventSystem {
    type SystemData = (
        Read<'a, SDLEventResource>,
        Write<'a, ExitResource>,
        WriteStorage<'a, Pos>,
        ReadStorage<'a, PlayerControlled>,
    );

    fn run(&mut self, (event_resource, mut exit_resource, mut position, player_controlled): Self::SystemData) {
        for event in event_resource.events.iter() {
            match event {
                Event::Quit{..} => {
                    exit_resource.exit = true;
                }
                Event::KeyDown{keycode, ..} => {
                    if let Some(actual_keycode) = keycode {
                        match actual_keycode {
                            Keycode::Escape => {
                                exit_resource.exit = true;
                            }
                            Keycode::Left => {
                                for(pos, _) in (&mut position, &player_controlled).join() {
                                    pos.x -= STEP;
                                }
                            }
                            Keycode::Right => {
                                for(pos, _) in (&mut position, &player_controlled).join() {
                                    pos.x += STEP;
                                }
                            }
                            Keycode::Up => {
                                for(pos, _) in (&mut position, &player_controlled).join() {
                                    pos.y -= STEP;
                                }
                            }
                            Keycode::Down => {
                                for(pos, _) in (&mut position, &player_controlled).join() {
                                    pos.y += STEP;
                                }
                            }
                            _ => { // no actions bound for this keycode, ignore
                            },
                        }
                    }
                }
                _ => {},
            }
        }
    }
}
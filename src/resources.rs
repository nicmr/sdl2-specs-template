use sdl2::event::Event;

#[derive(Debug, Clone, Default)]
pub struct ExitResource {
    pub exit: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SDLEventResource {
    pub events: Vec<Event>,
}
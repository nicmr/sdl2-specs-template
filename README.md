# sdl2-specs-template

## sdl2 + specs = ❤ ?

#### This template / code example shows how to use the [Rust SDL2 bindings](https://crates.io/crates/sdl2) with the Rust ECS [specs](https://crates.io/crates/specs) for game development.

SDL2 objects can not be used as a specs `Resource` (as of sdl2 0.34.2), because they are marked as not safe to send between threads (aka [`!Send`](https://doc.rust-lang.org/std/marker/trait.Send.html)), which is a requirement for specs resources.  
As a consequence,calls to SDL2 can not be handled by a specs system and have to be executed after the system dispatch instead.
SDL2 events such as user input have to be inserted before each ECS system dispatch.

## How to run
`cargo r`


## A quick overview of the source files (`/src/`)

- `main.rs`: SDL setup, specs setup, game loop with ecs dispatching
- `components.rs`, `resources.rs`: Various components and resources necessary to illustrate event forwarding and rendering a basic game
- `systems.rs`: Basic systems to show the interaction: Process SDL input and update positions
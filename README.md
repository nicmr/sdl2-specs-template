# sdl2 + specs = ❤ (?)

#### This template / code example shows how to use the [Rust SDL2 bindings](https://crates.io/crates/sdl2) with the ECS [specs](https://crates.io/crates/specs).

SDL2 objects can not be used as a specs `Resource` (as of 01/2021), because they are not safe to send between threads (aka [!Send](https://doc.rust-lang.org/std/marker/trait.Send.html)).  
As a consequence, events have to be inserted before each ECS system dispatch and calls to SDL2 have to be executed after the dispatch.

## How to run



## A quick overview of the source files (`/src/`)

- `main.rs`: SDL setup, specs setup, game loop with ecs dispatching
- `components.rs`, `resources.rs`: Various components and resources necessary to illustrate event forwarding and rendering a basic game
- `systems.rs`: Basic systems to show the interaction: Process SDL input and update positions
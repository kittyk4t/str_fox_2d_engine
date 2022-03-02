use engine::*;
use engine::animation::*;
use engine::entity::*;
use engine::types::*;
use engine::engine_safe::*;
use std::path::Path;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};


const WIDTH: usize = 240;
const HEIGHT: usize = 240;

struct Assets {
    image_paths: Vec<std::path::Path>,
    sheet_data: SheetData,
    obj_size: Vec2i,
    entity_size: Vec2i,
}

struct World{
    player: Entity,
    enemy: Vec<Entity>,
    projectiles: Vec<Entity>,
    score: u8,
    time: f32,
    interval: f64,
    frame: usize,
    level: usize,
}
impl World{
    fn new() -> World {
        World{
             player: Entity::new(),
    enemy: Vec::<Entity>::new(),
    projectiles: Vec::<Entity>::new(),
    score: 0,
    time: 0.0, //might have a timer instance of something
    interval: 0.0,
    frame: 0,
    level: 0,

        }
    }
}

//NOTES TO SELF, might change from having entity type, to just
//creating wrapper structs for entities in game
fn init_enemies(&mut World) -> ()
{

}

struct Game {}

fn main() {
    engine::go::<Game>();
}

impl engine::Game for Game {
    type Assets = Assets;
    type GameState = World;
    type DrawState = animation::DrawState;
    fn new() -> (State, Assets, DrawState) {
        let assets = Assets {
            
        };
        let state = World {
           
        };
        let draw_state = DrawState::new()//add params
        (state, assets, draw_state)
    }

    fn update(state: &mut World, assets: &mut Assets, input: &engine::Input) {
        use winit::event::VirtualKeyCode;
        
        //process and react to inputs
        //probably look at collisions
        //trigger animations
    }

    fn render(state: &mut World, draw: &mut DrawState, assets: &mut Assets, fb2d: &mut Image) {
        //could add if in mode, and have render cutscene/drawstate?
        draw.load_buffer(state.entities, fb2d);
    }
}


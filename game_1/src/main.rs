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

pub enum GameMode{
    Startscene,
    Game,
    Endscene
}

struct Graphics {
    draw_state: DrawState,
    cutscenes: Vec<Cutscene>
}

struct World{
    mode: GameMode,
    player: Entity,
    enemy: Vec<Entity>,
    projectiles: Vec<Entity>,
    score: u8,
    level: usize,
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
    type Assets = Graphics;
    type GameState = World;
    fn new() -> (World, Graphics) {
        let assets = Graphics {
            
        };
        let state = World {
           
        };
        let draw_state = DrawState::new()//add params
        (state, assets)
    }

    fn update(state: &mut World, assets: &mut Graphics, input: &engine::Input) {
        use winit::event::VirtualKeyCode;
        
        //process and react to inputs
        //probably look at collisions
        //trigger animations
    }

    fn render(state: &mut World, assets: &mut Graphics, fb2d: &mut Image) {
        //could add if in mode, and have render cutscene/drawstate?
        draw.load_buffer(state.entities, fb2d);
    }
}


use engine::*;
use engine::animation::*;
use engine::entity::*;
use engine::types::*;
use engine::engine_safe::*;
use std::path::Path;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use rand::{thread_rng, Rng};


const WIDTH: usize = 240;
const HEIGHT: usize = 240;
const WORLD_SIZE: (i32, i32) = (240, 240);
const DT: f32 = 1.0/30.0;

pub enum GameMode{
    Title,
    Startscene,
    Game,
    Endscene
}

fn scenes() -> Vec<Cutscene>{
    let mut scenes = Vec::new();
    let timing1: vec![15, 15];
    let timing2: Vec<usize>;
    let timing3 = vec![10,5,5,5,5,30,
    15,10,10,10,10,10,
    10,15,10,10,10,10,
    15,15,15,10,15,10,
    10,15,10,10,10,15,
    15,15,15,5,10];
    let mut data: SceneData;

    data = SceneData::new(2, Vec2i::from_tuple(WORLD_SIZE), timing1, true);
    scenes.push(Cutscene::new_data(std::path::Path::new("src/start_scene.png"), data));

    data = SceneData::new(35, Vec2i::from_tuple(WORLD_SIZE), timing3, false);
    scenes.push(Cutscene::new_data(std::path::Path::new("src/cutscene-pt2.png"), data));

    scenes
    
}
fn sheet_info() -> SheetData{
    let anim_num = vec![1, 1, 1, 3];
    let length = vec![vec![6], vec![6], vec![6],vec![2, 5, 5]];
    let timing = vec![vec![1,2,3,4,5, 5], vec![1,2,3,4,5, 5], vec![1,2,3,4,5, 5], 
    vec![1, 2], vec![1,2,3,4,5], vec![1,2,3,4,5]];
    let cycles = vec![vec![false], vec![false], vec![false], vec![false, false, false]];
    let retriggers = vec![vec![false], vec![false], vec![false], vec![true, true, true]];
    let prior = vec![vec![1], vec![6], vec![6],vec![2, 5, 5]];

    SheetData::new(anim_num, length, timing, cycles, retriggers, prior)
}

struct Graphics {
    draw_state: DrawState,
    cutscenes: Vec<Cutscene>
}

impl Graphics{
    pub fn new(world: &World) -> Graphics{
        Graphics{
            draw_state: DrawState::new(
        std::path::Path::new("src/game_sheet.png"),
        sheet_info(),
        Vec2i::new(48, 48),
        std::path::Path::new("src/background.png"),
        world.entities.as_ref(),
        Vec2i::from_tuple(WORLD_SIZE),
    ),
    cutscenes: scenes()

        }
        

    }

}

struct World{
    mode: GameMode,
    entities: Vec<Entity>,
    score: u8,
    level: usize,
}
impl World{
    fn new() -> World{
        let mut world = World{
            mode: GameMode::Title,
            entities: Vec::new(),
            score: 0,
            level: 0
        };
        let mut player = Entity{
             id: 0,
    ent_type: EntityType::Player,
    pos: Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32/ 2.0),
    vel: Vec2::new(0.0, 0.0),
    acc: Vec2::new(0.0, 0.0),
    size: Vec2i::new(48,48),
    hurt_box: HurtBox::new(Vec2i::new(24, 24)),
    texture: Texture{
        index: 2,
        is_visible: true}
    };
    player.update_hurtbox();
    world.entities.push(player);
    world.gen_enemies(4);
        world
    }

    fn gen_enemies(&mut self, num: usize) -> (){
        let mut num = num;
        let mut temp: Entity;
        let mut rng = thread_rng();
        let x_pos: f32;
        let y_pos = (WORLD_SIZE.1 - 48) as f32;

        if num * 48  > WORLD_SIZE.0 as usize{
            num = WORLD_SIZE.0 as usize / 48;
        }

        x_pos = (WORLD_SIZE.0 as usize - (num * 48)) as f32 / 2.0;
        
        for i in 0..num{
            temp = Entity{
                id: self.entities.len(),
    ent_type: EntityType::Enemy,
    pos: Vec2::new(x_pos + (i * 48) as f32, y_pos),
    vel: Vec2::new(0.0, 0.0),
    acc: Vec2::new(0.0, 0.0),
    size: Vec2i::new(48,48),
    hurt_box: HurtBox::new(Vec2i::new(24, 24)),
    texture: Texture{
        index: rng.gen_range(0..2),
        is_visible: true}
    };
    temp.update_hurtbox();
    self.entities.push(temp);


            }
        }

    fn update_pos(&mut self) -> ()
    {
        for entity in self.entities.iter_mut()
        {
            compute_distance(entity, DT, Vec2i::from_tuple(WORLD_SIZE));
        }
    }
}

fn change_motion(entity: &mut Entity, stop: bool, move_right: bool, move_down: bool) {
    let mut xd = 1.0;
    let mut yd = 1.0;
    if !move_right{
        xd = -1.0;
    }
    if !move_down{
        yd = -1.0;
    }

    match entity.ent_type{
        EntityType::Player => {
            if stop{
                entity.vel.x = 0.0;
            }
            else{

            if entity.vel.x <= 0.000001{
                entity.vel.x = xd;
            }
            else{
                entity.vel.x *= xd;
            }
            }
            
        },
        _ => {
            if stop{
                entity.acc = Vec2::new(0.0, 0.0);
            }
            else{
                if entity.acc.x <= 0.000001{
                    entity.acc.x = xd;
                }
                else{
                    entity.acc.x *= xd;
                }
                if entity.acc.y <= 0.000001{
                    entity.acc.y = yd;
                }
                else{
                    entity.acc.y *= yd;
                }
                
            }
        }

    }
}
fn compute_distance(entity: &mut Entity, time_constant: f32, world_sz: Vec2i) -> (){
    match entity.ent_type{
        EntityType::Player =>{
            entity.pos.x += entity.vel.x;

            if (entity.pos.x + entity.size.x as f32) as i32 > world_sz.x {
            entity.pos.x = (world_sz.x as f32) - entity.size.x as f32 - 1.0;
        } else if entity.pos.x as usize <= 0 {
            entity.pos.x = 0.0;
        }

        },
        _ => {
            entity.vel.x += entity.acc.x * time_constant;
            entity.pos.x += entity.vel.x *time_constant;
            entity.vel.y += entity.acc.y * time_constant;
            entity.pos.y += entity.vel.y *time_constant;

            if (entity.pos.x + entity.size.x as f32) as i32 > world_sz.x {
            entity.pos.x = (world_sz.x as f32) - entity.size.x as f32 - 1.0;
        } else if entity.pos.x as usize <= 0 {
            entity.pos.x = 0.0;
        }

        if (entity.pos.y + entity.size.y as f32) as i32 > world_sz.y {
            entity.pos.y = (world_sz.y as f32) - entity.size.y as f32 - 1.0;
        } else if entity.pos.y as usize <= 0 {
            entity.pos.y = 0.0;
        }

        },
        
    }
}

struct Game {}

fn main() {
    engine::go::<Game>();
}

impl engine::Game for Game {
    type Assets = Graphics;
    type GameState = World;
    fn new() -> (World, Graphics) {
        let state = World::new();
        let assets = Graphics::new(&state);
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
        
    }
}


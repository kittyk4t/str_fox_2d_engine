use engine::*;
use engine::animation::*;
use engine::entity::*;
use engine::types::*;
use engine::engine_safe::*;
use std::path::Path;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use rand::{thread_rng, Rng};


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
    let timing1 =  vec![15, 15];
    let timing2 = vec![1,2,3,4,5,6,7,
    5,10,10,10,30,10,10,
    5,10,10,10,30,5,10,
    5,10,10,10,10,30,10,
    10,10,30,10,10,10,30,
    10,10,10,30,10,10,10,
    30,10,5,10];

    let timing3 = vec![10,5,5,5,5,30,
    15,10,10,10,10,10,
    10,15,10,10,10,10,
    15,15,15,10,15,10,
    10,15,10,10,10,15,
    15,15,15,5,10];
    let mut data: SceneData;

    data = SceneData::new(2, Vec2i::from_tuple(WORLD_SIZE), timing1, true);
    scenes.push(Cutscene::new_data(std::path::Path::new("src/title_scene.png"), data));
    scenes[0].trigger();

    data = SceneData::new(46, Vec2i::from_tuple(WORLD_SIZE), timing2, false);
    scenes.push(Cutscene::new_data(std::path::Path::new("src/cutscene-pt1.png"), data));

    data = SceneData::new(35, Vec2i::from_tuple(WORLD_SIZE), timing3, false);
    scenes.push(Cutscene::new_data(std::path::Path::new("src/cutscene-pt2.png"), data));

    scenes
    
}
fn sheet_info() -> SheetData{
    let anim_num = vec![1, 1, 1, 3];
    let length = vec![vec![6], vec![6], vec![6],vec![2, 5, 5]];
    let timing = vec![vec![1,2,3,4,5, 5], vec![1,2,3,4,5, 5], vec![1,2,3,4,5, 5], 
    vec![2, 2], vec![1,2,3,4,5], vec![1,2,3,4,5]];
    let cycles = vec![vec![false], vec![false], vec![false], vec![false, false, false]];
    let retriggers = vec![vec![false], vec![false], vec![false], vec![true, true, true]];
    let prior = vec![vec![1], vec![6], vec![6],vec![0, 5, 5]];

    SheetData::new(anim_num, length, timing, cycles, retriggers, prior)
}

struct EntityGrid{
    grid: Vec<Vec<usize>>,
    row_sz: usize,
    mid: usize,
}
impl EntityGrid{
    fn new(row_sz: usize) -> EntityGrid{
        EntityGrid{
            grid: Vec::new(),
            row_sz,
            mid: row_sz/2
        }
    }
    fn add_row(&mut self, index_offset: usize, entities: &Vec<Entity>) -> (){
        assert!(self.row_sz <= entities.len());
        let diff = self.row_sz - entities.len();
        let mut new = Vec::new();
        let mut added = 0;

        for i in 0..self.row_sz{
            if i >= diff && added < entities.len(){
                let index = (i - diff) + index_offset;
                new.push(index);
                added+=1;
            }
            else{
                new.push(0);
            }
        }
        self.grid.push(new);
    }
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
    enem_grid: EntityGrid,
    score: u8,
    level: usize,
}
impl World{
    fn new() -> World{
        let mut world = World{
            mode: GameMode::Title,
            entities: Vec::new(),
            enem_grid: EntityGrid::new(4),
            score: 0,
            level: 0
        };
        let mut player = Entity{
             id: 0,
    ent_type: EntityType::Player,
    pos: Vec2::new(WIDTH as f32 / 2.0, 0.0),
    vel: Vec2::new(0.0, 0.0),
    acc: Vec2::new(0.0, 0.0),
    size: Vec2i::new(48,48),
    hurt_box: HurtBox::new(Vec2i::new(24, 24)),
    texture: Texture{
        index: 3,
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
        index: rng.gen_range(0..3),
        is_visible: true}
    };
    temp.update_hurtbox();
    //self.enem_grid.add_row(self.entities.len(), &temp);
    self.entities.push(temp);


            }
        }

    fn update_pos(&mut self) -> ()
    {
        for entity in self.entities.iter_mut()
        {
            entity.compute_distance(DT, Vec2i::from_tuple(WORLD_SIZE));
        }
    }

    fn move_enemies(&mut self) -> (){
        let mut first = true;
                let mut stop = false;
                let mut total_enemies = 0;
                let mut last = 0;
                for (i, entity) in self.entities.iter_mut().enumerate(){
                    match entity.ent_type{
                        EntityType::Enemy =>{
                            total_enemies+=1;
                            last = i;
                            if first{
                                first = false;
                                stop = entity.pos.y as i32 <= 92;
                            }
                            entity.change_motion(stop, Vec2b::new(false, false), Vec2b::new(true, false));
                        },
                        _ => {}
                    }
                }
                if last != 0 && self.entities[last].pos.y as i32 <= (WORLD_SIZE.1 - 96){
                    let mut rng = thread_rng();
                    self.gen_enemies(rng.gen_range(1..5));
                }

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

        match state.mode {
            GameMode::Title => 
            {
                if input.is_key_down(VirtualKeyCode::Space){
                    assets.cutscenes[1].trigger();
                    state.mode = GameMode::Game;
                }
            },
            GameMode::Startscene=> {
                if !assets.cutscenes[1].is_active(){
                    let index = assets.cutscenes[1].last_plate();
                    assets.cutscenes[1].set_plate(index);

                    state.mode = GameMode::Game;
                }
            },
            GameMode::Game =>{
                //moving player
                if input.is_key_down(VirtualKeyCode::Right){
                    state.entities[0].change_motion(false, Vec2b::new(true, true), Vec2b::new(false, false));
                } else if input.is_key_down(VirtualKeyCode::Left){
                    state.entities[0].change_motion(false, Vec2b::new(true, false), Vec2b::new(false, false));
                }  else{
                     state.entities[0].change_motion(true, Vec2b::new(true, false), Vec2b::new(false, false));
                }
                state.move_enemies();
                
                //shooting at enemies
                if input.is_key_pressed(VirtualKeyCode::A){
                    assets.draw_state.trigger_animation(&state.entities[0], 0);
                }
                

                state.update_pos();
            }
            GameMode::Endscene =>{
                if !assets.cutscenes[2].is_active(){
                    let index = assets.cutscenes[2].last_plate();
                    assets.cutscenes[2].set_plate(index);
                }
            }
        }
        
        //process and react to inputs
        //probably look at collisions
        //trigger animations
    }

    fn render(state: &mut World, assets: &mut Graphics, fb2d: &mut Image) {
        //could add if in mode, and have render cutscene/drawstate?
        match state.mode{
            GameMode::Title =>{ 
                assets.cutscenes[0].load_buffer(fb2d);
            },
            GameMode::Startscene =>{
                assets.cutscenes[1].load_buffer(fb2d);
            },
            GameMode::Game => {
                assets.draw_state.load_buffer(&state.entities, fb2d);
            },
            GameMode::Endscene => {
                assets.cutscenes[2].load_buffer(fb2d);
            },
        }
        
    }
}


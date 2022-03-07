use engine::*;
//use std::path::Path;
use std::collections::HashMap;
use winit::event::VirtualKeyCode;
//use winit::event_loop::{ControlFlow, EventLoop};
use rand::{thread_rng, Rng};

const WORLD_SIZE: (i32, i32) = (240, 240);
const DT: f32 = 1.0 / 30.0;

#[derive(Clone, Copy)]
pub enum GameMode {
    Title,
    Startscene,
    Game,
    Endscene,
}

fn scenes() -> Vec<Cutscene> {
    let mut scenes = Vec::new();
    let timing1 = vec![10, 10];
    let timing2 = vec![
        1, 2, 3, 4, 5, 6, 7, 5, 10, 10, 10, 30, 10, 10, 5, 10, 10, 10, 30, 5, 10, 5, 10, 10, 10,
        10, 30, 10, 10, 10, 30, 10, 10, 10, 30, 10, 10, 10, 30, 10, 10, 10, 30, 10, 5, 10,
    ];

    let timing3 = vec![
        30, 10, 5, 5, 5, 30, 15, 10, 10, 10, 10, 10, 10, 15, 10, 10, 10, 10, 15, 15, 15, 10, 15,
        10, 10, 15, 10, 10, 10, 15, 15, 15, 15, 5, 10,
    ];
    let mut data: SceneData;

    data = SceneData::new(2, Vec2i::from_tuple(WORLD_SIZE), timing1, true);
    scenes.push(Cutscene::new_data(
        std::path::Path::new("src/title_scene.png"),
        data,
    ));
    scenes[0].trigger();

    data = SceneData::new(46, Vec2i::from_tuple(WORLD_SIZE), timing2, false);
    scenes.push(Cutscene::new_data(
        std::path::Path::new("src/cutscene-pt1.png"),
        data,
    ));

    data = SceneData::new(35, Vec2i::from_tuple(WORLD_SIZE), timing3, false);
    scenes.push(Cutscene::new_data(
        std::path::Path::new("src/cutscene-pt2.png"),
        data,
    ));

    scenes
}
fn sheet_info() -> SheetData {
    let anim_num = vec![1, 1, 1, 3, 1];
    let length = vec![vec![6], vec![6], vec![6], vec![2, 5, 5], vec![2]];
    let timing = vec![
        vec![1, 2, 3, 4, 5, 10],
        vec![1, 2, 3, 4, 5, 10],
        vec![1, 2, 3, 4, 5, 10],
        vec![2, 2],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2],
    ];
    let cycles = vec![
        vec![false],
        vec![false],
        vec![false],
        vec![false, false, false],
        vec![true],
    ];
    let retriggers = vec![
        vec![false],
        vec![false],
        vec![false],
        vec![true, true, true],
        vec![false],
    ];
    let prior = vec![vec![1], vec![6], vec![6], vec![0, 5, 5], vec![2]];

    SheetData::new(anim_num, length, timing, cycles, retriggers, prior)
}

#[derive(Clone, Copy)]
pub enum RowOff {
    Center,
    Left,
    Right,
}
struct EntityGrid {
    grid: Vec<Vec<usize>>,
    row_sz: usize,
    even_right: bool,
    even_rows: RowOff,
    odd_rows: RowOff,
}
impl EntityGrid {
    fn new(row_sz: usize) -> EntityGrid {
        EntityGrid {
            grid: Vec::new(),
            row_sz,
            even_right: true,
            even_rows: RowOff::Center,
            odd_rows: RowOff::Center,
        }
    }
    fn add_row(&mut self, id: Vec<usize>) -> () {
        let diff = self.row_sz - id.len();
        let mut new = Vec::new();
        let mut added = 0;
        let mut index = 0;

        for i in 0..self.row_sz {
            if i >= diff && added < id.len() {
                new.push(id[index]);
                index += 1;
                added += 1;
            } else {
                new.push(0);
            }
        }
        self.grid.push(new);
    }
    fn first(&self) -> usize {
        for row in self.grid.iter() {
            for id in row.iter() {
                if *id > 0 {
                    return *id;
                }
            }
        }
        0
    }
    fn last(&self) -> usize {
        let mut last = 0;
        for row in self.grid.iter() {
            for id in row.iter() {
                if *id > 0 {
                    last = *id;
                }
            }
        }
        last
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for row in self.grid.iter() {
            for id in row.iter() {
                if *id > 0 {
                    count += 1;
                }
            }
        }
        count
    }

    fn remove_index(&mut self, rid: usize) -> () {
        for (i, row) in self.grid.clone().iter().enumerate() {
            for (j, id) in row.clone().iter().enumerate() {
                if *id == rid {
                    self.grid[i][j] = 0;
                }
            }
        }
    }

    fn offset(&mut self) -> () {
        match (self.even_rows, self.odd_rows) {
            (RowOff::Center, RowOff::Center) => {
                if self.even_right {
                    self.even_rows = RowOff::Right;
                    self.odd_rows = RowOff::Left;
                } else {
                    self.even_rows = RowOff::Left;
                    self.odd_rows = RowOff::Right;
                }
            }
            (RowOff::Right, RowOff::Left) => {
                if self.even_right {
                    self.even_rows = RowOff::Center;
                    self.odd_rows = RowOff::Center;
                    self.even_right = !self.even_right;
                }
            }
            (RowOff::Left, RowOff::Right) => {
                if !self.even_right {
                    self.even_rows = RowOff::Center;
                    self.odd_rows = RowOff::Center;
                    self.even_right = !self.even_right;
                }
            }
            _ => {}
        }
    }
}

struct Graphics {
    draw_state: DrawState,
    cutscenes: Vec<Cutscene>,
}

impl Graphics {
    pub fn new(world: &World) -> Graphics {
        Graphics {
            draw_state: DrawState::new(
                std::path::Path::new("src/sheet_final.png"),
                sheet_info(),
                Vec2i::new(48, 48),
                std::path::Path::new("src/background.png"),
                Vec::from_iter(world.entities.values()),
                Vec2i::from_tuple(WORLD_SIZE),
            ),
            cutscenes: scenes(),
        }
    }
}

struct World {
    mode: GameMode,
    entities: HashMap<usize, engine::entity::Entity>,
    enem_grid: EntityGrid,
    num_enem: usize,
    score: u8,
}
impl World {
    fn new() -> World {
        let mut world = World {
            mode: GameMode::Title,
            entities: HashMap::new(),
            enem_grid: EntityGrid::new(4),
            num_enem: 1,
            score: 0,
        };
        let mut player = engine::entity::Entity {
            id: 0,
            ent_type: engine::entity::EntityType::Player,
            pos: Vec2::new(WIDTH as f32 / 2.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
            size: Vec2i::new(48, 48),
            hurt_box: engine::entity::HurtBox::new(Vec2i::new(24, 24)),
            texture: engine::entity::Texture {
                index: 3,
                is_visible: true,
            },
        };
        player.update_hurtbox();
        world.entities.insert(player.id, player);
        world.gen_enemies(4);
        world
    }

    fn gen_enemies(&mut self, num: usize) -> () {
        let mut num = num;
        let mut temp: engine::entity::Entity;
        let mut rng = thread_rng();
        let x_pos: f32;
        let y_pos = (WORLD_SIZE.1 - 48) as f32;

        if num * 48 > WORLD_SIZE.0 as usize {
            num = WORLD_SIZE.0 as usize / 48;
        }

        x_pos = (WORLD_SIZE.0 as usize - (num * 48)) as f32 / 2.0;

        let mut ids = Vec::new();
        for i in 0..num {
            temp = engine::entity::Entity {
                id: self.num_enem + i,
                ent_type: engine::entity::EntityType::Enemy,
                pos: Vec2::new(x_pos + (i * 48) as f32, y_pos),
                vel: Vec2::new(0.0, 0.0),
                acc: Vec2::new(0.0, 0.0),
                size: Vec2i::new(48, 48),
                hurt_box: engine::entity::HurtBox::new(Vec2i::new(36, 36)),
                texture: engine::entity::Texture {
                    index: rng.gen_range(0..3),
                    is_visible: true,
                },
            };
            temp.update_hurtbox();
            ids.push(temp.id);
            //self.enem_grid.add_row(self.entities.len(), &temp);
            self.entities.insert(temp.id, temp);
        }
        self.num_enem += num;
        self.enem_grid.add_row(ids);
    }

    fn create_projectile(&mut self, from: &engine::entity::Entity) -> usize {
        let mut temp = engine::entity::Entity {
            id: self.entities.len() + 100,
            ent_type: engine::entity::EntityType::Projectile,
            pos: Vec2::new(
                from.pos.x + from.size.x as f32,
                from.pos.y + from.size.y as f32,
            ),
            vel: Vec2::new(0.0, 1.0),
            acc: Vec2::new(0.0, 5.0),
            size: Vec2i::new(16, 16),
            hurt_box: engine::entity::HurtBox::new(Vec2i::new(16, 16)),
            texture: engine::entity::Texture {
                index: 4,
                is_visible: true,
            },
        };
        let ret = temp.id;
        temp.update_hurtbox();
        self.entities.insert(temp.id, temp);
        ret
    }
    fn update_pos(&mut self) -> () {
        for entity in self.entities.values_mut() {
            entity.compute_distance(DT, Vec2i::from_tuple(WORLD_SIZE));
            entity.update_hurtbox();
        }
    }

    fn move_enemies(&mut self, cur_frame: usize) -> () {
        if self.enem_grid.count() > 0 {
            match (self.enem_grid.even_rows, self.enem_grid.odd_rows) {
                (RowOff::Center, RowOff::Center) => {
                    if self.entities.get(&self.enem_grid.last()).unwrap().pos.y
                        < (WORLD_SIZE.1 - 48) as f32
                    {
                        let mut rng = thread_rng();
                        self.gen_enemies(rng.gen_range(1..self.enem_grid.row_sz));
                    }
                }
                _ => {}
            }
        }
        if cur_frame % 45 == 0 {
            if self.entities.get(&self.enem_grid.first()).unwrap().pos.y > 24.0 {
                for row in self.enem_grid.grid.iter() {
                    for id in row.iter() {
                        if *id > 0 {
                            match self.entities.get_mut(id) {
                                None => {}
                                Some(enemy) => {
                                    enemy.pos.y -= 24.0;
                                }
                            }
                        }
                    }
                }
            }
        }
        if cur_frame % 60 == 0 {
            self.enem_grid.offset();
            for (i, row) in self.enem_grid.grid.iter().enumerate() {
                for id in row.iter() {
                    if *id > 0 {
                        match self.entities.get_mut(id) {
                            None => {}
                            Some(enemy) => {
                                if self.enem_grid.even_right {
                                    if i % 2 == 0 {
                                        enemy.pos.x += 24.0;
                                    } else {
                                        enemy.pos.x -= 24.0;
                                    }
                                } else {
                                    if i % 2 == 0 {
                                        enemy.pos.x -= 24.0;
                                    } else {
                                        enemy.pos.x += 24.0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
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
        match state.mode {
            GameMode::Title => {
                if input.is_key_down(VirtualKeyCode::Space) {
                    assets.cutscenes[1].trigger();
                    state.mode = GameMode::Startscene;
                }
            }
            GameMode::Startscene => {
                if input.is_key_pressed(VirtualKeyCode::Space) {
                    state.mode = GameMode::Game;
                    let index = assets.cutscenes[1].last_plate();
                    assets.cutscenes[1].set_plate(index);
                }
                if !assets.cutscenes[1].is_active() {
                    let index = assets.cutscenes[1].last_plate();
                    assets.cutscenes[1].set_plate(index);

                    state.mode = GameMode::Game;
                }
            }
            GameMode::Game => {
                //moving player
                if input.is_key_down(VirtualKeyCode::Right) {
                    state.entities.get_mut(&0).unwrap().change_motion(
                        false,
                        Vec2b::new(true, true),
                        Vec2b::new(false, false),
                    );
                } else if input.is_key_down(VirtualKeyCode::Left) {
                    state.entities.get_mut(&0).unwrap().change_motion(
                        false,
                        Vec2b::new(true, false),
                        Vec2b::new(false, false),
                    );
                } else {
                    state.entities.get_mut(&0).unwrap().change_motion(
                        true,
                        Vec2b::new(true, false),
                        Vec2b::new(false, false),
                    );
                }

                state.move_enemies(assets.draw_state.cur_frame);

                //shooting at enemies
                if input.is_key_pressed(VirtualKeyCode::A) {
                    assets
                        .draw_state
                        .trigger_animation(&state.entities.get(&0).unwrap(), 0);
                    let p_id = state.create_projectile(&state.entities.get(&0).unwrap().clone());
                    assets
                        .draw_state
                        .trigger_animation(&state.entities.get(&p_id).unwrap(), 0);
                }

                state.update_pos();
                let contacts = engine::collision::contacts(Vec::from_iter(state.entities.values()));

                for contact in contacts.iter() {
                    match contact.contact_type {
                        (engine::entity::EntityType::Player, engine::entity::EntityType::Enemy) => {
                            assets
                                .draw_state
                                .trigger_animation(&state.entities.get(&0).unwrap(), 1);
                            assets.draw_state.anim_entities.remove(&contact.collider2);
                            state.enem_grid.remove_index(contact.collider2);
                            state.entities.remove(&contact.collider2);
                        }

                        (engine::entity::EntityType::Enemy, engine::entity::EntityType::Player) => {
                            assets
                                .draw_state
                                .trigger_animation(&state.entities.get(&0).unwrap(), 2);
                            assets.draw_state.anim_entities.remove(&contact.collider1);
                            state.enem_grid.remove_index(contact.collider1);
                            state.entities.remove(&contact.collider1);
                        }
                        (
                            engine::entity::EntityType::Projectile,
                            engine::entity::EntityType::Enemy,
                        ) => {
                            match state.entities.get(&contact.collider2) {
                                None => {}
                                Some(enemy) => {
                                    assets.draw_state.trigger_animation(enemy, 0);
                                    assets.draw_state.anim_entities.remove(&contact.collider1);
                                }
                            }
                            state.enem_grid.remove_index(contact.collider2);
                            state.entities.remove(&contact.collider1);
                            state.entities.remove(&contact.collider2);
                            state.score += 10;
                        }

                        (
                            engine::entity::EntityType::Enemy,
                            engine::entity::EntityType::Projectile,
                        ) => {
                            match state.entities.get(&contact.collider1) {
                                None => {}
                                Some(enemy) => {
                                    assets.draw_state.trigger_animation(enemy, 0);
                                    assets.draw_state.anim_entities.remove(&contact.collider2);
                                }
                            }
                            state.enem_grid.remove_index(contact.collider1);
                            state.entities.remove(&contact.collider1);
                            state.entities.remove(&contact.collider2);
                            state.score += 10;
                        }
                        (_, _) => {}
                    }

                    print!("score: ");
                    println!("{}", state.score);
                    assets
                        .draw_state
                        .tidy(Vec::from_iter(state.entities.keys()));

                    if state.score >= 100 {
                        assets.cutscenes[2].trigger();
                        state.mode = GameMode::Endscene;
                    }
                }
            }
            GameMode::Endscene => {
                if !assets.cutscenes[2].is_active() {
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
        match state.mode {
            GameMode::Title => {
                assets.cutscenes[0].load_buffer(fb2d);
            }
            GameMode::Startscene => {
                assets.cutscenes[1].load_buffer(fb2d);
            }
            GameMode::Game => {
                assets
                    .draw_state
                    .load_buffer(Vec::from_iter(state.entities.values()), fb2d);
            }
            GameMode::Endscene => {
                assets.cutscenes[2].load_buffer(fb2d);
            }
        }
    }
}

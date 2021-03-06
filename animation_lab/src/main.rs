use crate::entity::*;
use crate::types::*;
use engine::animation::*;
use engine::engine_core::*;
use engine::image::*;
use engine::*;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use std::time::Instant;

//ADD IMPORTS FROM ENGINE

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

pub enum SurfaceType {
    Normal,
    Slick,
    Muddy,
}

#[derive(Clone)]
pub struct Figure {
    pos: Vec2,
    size: Vec2,
    vel: Vec2,
    acc: Vec2,
    sprite: Sprite,
    cur_anim: Vec2, //first is the index current animation, second is the current pose in animation
}
impl Figure //LOOK HERE
{
    pub fn new(pos: Vec2, size: Vec2, vel: Vec2, acc: Vec2, sprite: Sprite) -> Figure {
        Figure {
            pos,
            size,
            vel,
            acc,
            sprite,
            cur_anim: Vec2::new(0.0, 0.0),
        }
    }

    fn to_entity(&self) -> Entity {
        Entity {
            id: 0,
            pos: self.pos,
            vel: self.vel,
            size: self.size.to_Vec2i(),
            texture: Texture {
                index: 0,
                is_visible: true,
                animation_layer: 1,
            },
        }
    }

    pub fn compute_distance(self: &mut Self) -> () {
        self.vel.x += self.acc.x;
        self.pos.x += self.vel.x;

        /*if (self.pos.x + self.size.x) as usize > WIDTH {
            self.pos.x = (WIDTH as f32) - self.size.x - 1.0;
            self.acc.x *= -0.5; //mimics energy lost from hitting something
            self.vel.x *= -1.0;
        } else if self.pos.x as usize == 0 {
            self.acc.x *= -0.5; //mimics energy lost from hitting something
            self.vel.x *= -1.0;
        }*/

        self.vel.y += self.acc.y * self.pos.y;
        self.pos.y += self.vel.y * self.pos.y;

        if (self.pos.y + self.size.y) as usize > HEIGHT {
            self.pos.y = (HEIGHT as f32) - self.size.y - 1.0;
            self.acc.y *= -0.5;
            self.vel.y *= -1.0;
        } else if self.pos.y as usize == 0 {
            self.acc.y *= -0.5;
            self.vel.y *= -1.0;
        }
    }

    pub fn apply_friction(self: &mut Self) -> () {
        self.acc.x *= 0.25;
        self.acc.y *= 0.25;

        if self.acc.x.abs() < 0.1 {
            self.acc.x = 0.0;

            if self.vel.x > 0.1 {
                self.vel.x *= 0.5;
            } else {
                self.vel.x = 0.0;
            }
        }
        if self.acc.y.abs() < 0.1 {
            self.acc.y = 0.0;

            if self.vel.y > 0.1 {
                self.vel.y *= 0.5;
            } else {
                self.vel.y = 0.0;
            }
        }
    }

    /*
    axis: true = x
    direction:
    */
    //LOOK HERE
    pub fn set_acc(self: &mut Self, surface: SurfaceType, axis: bool, direction: f32) -> () {
        match surface {
            SurfaceType::Normal => {
                if axis {
                    self.acc.x = direction * 2.0;
                } else {
                    self.acc.y = direction * 2.0;
                }
            }
            SurfaceType::Slick => {
                if axis {
                    self.acc.x = direction * 5.0;
                } else {
                    self.acc.y = direction * 5.0;
                }
            }
            SurfaceType::Muddy => {
                if axis {
                    self.acc.x = direction * 0.5;
                } else {
                    self.acc.y = direction * 0.5;
                }
            }
        }
    }
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

fn main() {
    let mut sprite_sheet =
        SpriteSheet::new(Image::from_file(std::path::Path::new("src/loki_test.png")));
    /*let mut sprites = Vec::new();
    sprites.push(1);

    let mut per_pose = Vec::new();
    per_pose.push(1);
    per_pose.push(2);
    let mut timing = Vec::new();
    timing.push(per_pose);

    let data = SheetData::new(sprites.clone(), vec![vec![2]], timing.clone(), 
    vec![vec![false]], vec![vec![true]], vec![vec![2]]);
    

    sprite_sheet.load_sprites(data.clone(), Vec2i::new(48, 48));

    let color = engine::types::Color::new(0, 0, 0, 255);
    let fig = Figure::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(48.0, 48.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        sprite_sheet.sprites[0].clone(),
    );*/

    let mut entities = Vec::new();
    //entities.push(fig.to_entity());
    entities.push(Entity{
        id: 0,
        pos: Vec2::new(0.0, 0.0),
        vel:  Vec2::new(0.0, 0.0),
        size: Vec2i::new(48, 48),
        texture: Texture{
        index: 3,
        is_visible: true,
        animation_layer: 1,
    },
    });

    let mut draw_state = DrawState::new(
        std::path::Path::new("src/test_sheet.png"),
        sheet_info(),
        Vec2i::new(48, 48),
        std::path::Path::new("src/test_back.png"),
        entities.as_ref(),
        Vec2i::new(WIDTH as i32, HEIGHT as i32),
    );
    let test = draw_state.anim_entities.get(&entities[0].id).unwrap().sprite.animations[0].pose.len();
    println!("sa{}", test);

    let c_time = vec![10,5,5,5,5,30,
    15,10,10,10,10,10,
    10,15,10,10,10,10,
    15,15,15,10,15,10,
    10,15,10,10,10,15,
    15,15,15,5,10];
    let mut cutscene = Cutscene::new(std::path::Path::new("src/test.png"), 35, Vec2i::new(240, 240), c_time, false);

    let mut now_keys = [false; 255];
    let mut prev_keys = now_keys.clone();
    let mut mouse_press = [false; 3]; //LOOK HERE
                                      //let mut now_pos = (0.0, 0.0);

    let event_loop = EventLoop::new();

    let (mut vulkan_config, mut vulkan_state) =
        engine_core::vulkan_init(&event_loop, WIDTH, HEIGHT);
    
    let mut acc = 0.0_f32;
    let mut prev_t = Instant::now();
    // Let's clock the game at 60 simulation steps per second
   const SIM_DT : f32 = 1.0/30.0;

    event_loop.run(move |event, _, control_flow| {
        match event {
            // Nested match patterns are pretty useful---see if you can figure out what's going on in this match.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                vulkan_state.recreate_swapchain = true;
            }
            // NewEvents: Let's start processing events.
            Event::NewEvents(_) => {}
            // WindowEvent->KeyboardInput: Keyboard input!
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    },
                ..
            } => match state {
                winit::event::ElementState::Pressed => {
                    now_keys[keycode as usize] = true;
                }
                winit::event::ElementState::Released => {
                    now_keys[keycode as usize] = false;
                }
            },
            Event::MainEventsCleared => {
                let elapsed = prev_t.elapsed().as_secs_f32();
                //println!("{}", elapsed);
                acc += elapsed;
                prev_t = Instant::now();
                while acc >= SIM_DT {
                    if now_keys[VirtualKeyCode::Right as usize]{
                    let old = entities[0].pos;
                    entities[0].pos = Vec2::new(old.x +1.0, old.y);
                }
                if now_keys[VirtualKeyCode::Left as usize]{
                   
                }
                if now_keys[VirtualKeyCode::Down as usize]{
                    //draw_state.trigger_animation(&entities[0], 0);
                    cutscene.trigger();
                }
                // now_keys are officially "old" now, after update
                prev_keys.copy_from_slice(&now_keys);
                //cutscene.incr_frame();
                cutscene.load_buffer(&mut vulkan_config);  
                                
                acc -= SIM_DT;
                
            }
                
               // let rect = Rect::new(Vec2i::new(0,0), Vec2i::new(96, 48));
                //vulkan_config.fb2d.bitblt(&draw_state.sprite_sheet.sheet, rect , Vec2i::new(0,0));
                //draw_state.load_buffer(entities.as_ref(), &mut vulkan_config.fb2d);
                
                engine_core::render3d(&mut vulkan_config, &mut vulkan_state);
            }
            _ => (),
        }
    });
}

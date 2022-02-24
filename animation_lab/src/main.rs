use crate::entity::*;
use crate::types::*;
use engine::animation::*;
use engine::engine_core::*;
use engine::image::*;
use engine::*;
use serde;
use serde::Deserialize;
use serde_json;
use std::fs;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

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

fn main() {
    let mut sprite_sheet =
        SpriteSheet::new(Image::from_file(std::path::Path::new("src/loki_test.png")));
    let mut sprites = Vec::new();
    sprites.push(1);

    sprite_sheet.load_sprites(sprites.clone(), Vec2i::new(48, 48));

    let color = engine::types::Color::new(0, 0, 0, 255);
    let fig = Figure::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(48.0, 48.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        sprite_sheet.sprites[0].clone(),
    );

    let mut entities = Vec::new();
    entities.push(fig.to_entity());

    let mut draw_state = DrawState::new(
        std::path::Path::new("src/loki_test.png"),
        sprites.clone(),
        Vec2i::new(48, 48),
        entities.as_ref(),
        Vec2i::new(WIDTH as i32, HEIGHT as i32),
    );
    //let test = draw_state.anim_entities.get().unwrap().sprite.animations.len();
    println!("{}", draw_state.anim_entities.len());

    let mut now_keys = [false; 255];
    let mut prev_keys = now_keys.clone();
    let mut mouse_press = [false; 3]; //LOOK HERE
                                      //let mut now_pos = (0.0, 0.0);

    let event_loop = EventLoop::new();

    let (mut vulkan_config, mut vulkan_state) =
        engine_core::vulkan_init(&event_loop, WIDTH, HEIGHT);

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
                if now_keys[VirtualKeyCode::Right as usize]{
                    draw_state.trigger_animation(entities[0].clone(), 0);
                }
                // now_keys are officially "old" now, after update
                prev_keys.copy_from_slice(&now_keys);

            
                //let rect = Rect::new(Vec2i::new(0,0), Vec2i::new(96, 48));
                //vulkan_config.fb2d.bitblt(&draw_state.sprite_sheet.sheet, rect , Vec2i::new(0,0));
                draw_state.load_buffer(entities.as_ref(), &mut vulkan_config);
                engine_core::render3d(&mut vulkan_config, &mut vulkan_state);
            }
            _ => (),
        }
    });
}

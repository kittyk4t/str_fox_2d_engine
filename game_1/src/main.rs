use engine::*;
use engine::animation::*;
use engine::entity::*;

use serde;
use serde::Deserialize;
use serde_json;
use std::fs;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};


const WIDTH: usize = 240;
const HEIGHT: usize = 240;

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



fn main() {
    //Game State Stuff
    let mut world = World::new();
    let mut setup = Setup::create_setUp(WIDTH as u32, HEIGHT as u32);
    let event_loop = EventLoop::new();
    
    //Animation Stuff
    let background_image = Image::new(Vec2i::new(WIDTH as i32, HEIGHT as i32));
    let sprite_sheet = SpriteSheet::new(Image::from_file("src/loki_test.png".to_string()));
    let sprites = Vec::new();
    sprites.push(1);
    sprite_sheet.load_sprites(sprites, Vec2i::new(48, 48));

    let color = Color::new(0, 0, 0, 255);

    // Here's our (2D drawing) framebuffer.
    let mut fb2d = Image::new(Vec2i::new(WIDTH as i32, HEIGHT as i32));

     // Game entities 
    // create Player entity and assign to correct sprite
   //  create certain number of beginner enemies -> assign to sprites
   //somehow set behavior?

    //Game Loop
    let mut now_keys = [false; 255];
    let mut prev_keys = now_keys.clone();

    event_loop.run(move |event, _, control_flow| {
        match event {
            // NewEvents: Let's start processing events.
            Event::NewEvents(_) => {
                // Leave now_keys alone, but copy over all changed keys
                prev_keys.copy_from_slice(&now_keys);
            }
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
                setup.recreate_swapchain = true;
            }
            // WindowEvent->KeyboardInput: Keyboard input!
            Event::WindowEvent {
                // Note this deeply nested pattern match
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                // Which serves to filter out only events we actually want
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                // It also binds these handy variable names!
                match state {
                    winit::event::ElementState::Pressed => {
                        // VirtualKeycode is an enum with a defined representation
                        now_keys[keycode as usize] = true;
                    }
                    winit::event::ElementState::Released => {
                        now_keys[keycode as usize] = false;
                    }
                }
            }
            Event::MainEventsCleared => {
                {
                    //println!("DT: {:?}", last_frame.elapsed());
                    // We need to synchronize here to send new data to the GPU.
                    // We can't send the new framebuffer until the previous frame is done being drawn.
                    // Dropping the future will block until it's done.
                    if let Some(mut fut) = setup.previous_frame_end.take() {
                        fut.cleanup_finished();
                    }

                    //input of keys
                    if now_keys[VirtualKeyCode::Right as usize] {

                        //fig.set_acc(SurfaceType::Slick, true, 1.0);
                    } else if now_keys[VirtualKeyCode::Left as usize] {

                        //fig.set_acc(SurfaceType::Slick, true, -1.0);
                    }
                    //have enemies update behavior
                    //check for collisions
                    //sync animation with positions/updates
               
                }
                fb2d.clear();
                //update/tick animations
                //work on rendering 

                setup.run(&fb2d);
                
            }
            _ => (),
        }
    });
}



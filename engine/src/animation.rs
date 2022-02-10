/*
animation:
define a spritesheet, then ahve the game share one spritesheet
struct spritesheet
image:
data: slices for each indivial sprite
know the positon and size of each sprite and then you load
get all the sprites 

animation layers -> if things have a priorirty can sort the list based on priorty,or like using a priorty_queue 
sort by enum????

use entity type 
match on entty.type
 player
enemey
etc 

same entity type, just which ever gets drawn first
-> projectle, team jsut give one team priority 


draw_sprite(id) -> data

struct
basically handles the loading of the image 
handles getting specific sprites

externally only ever call render, interally given list of entities
to render 
main functionally
puts on screen
*/
use png;
use std::sync::Arc;
use serde;
use serde::Deserialize;
use serde_json;
use std::fs;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::descriptor_set::PersistentDescriptorSet;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::format::Format;
use vulkano::image::ImageCreateFlags;
use vulkano::image::{
    view::ImageView, ImageAccess, ImageDimensions, ImageUsage, StorageImage, SwapchainImage,
};
use vulkano::instance::Instance;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::render_pass::{Framebuffer, RenderPass, Subpass};
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::swapchain::{self, AcquireError, Swapchain, SwapchainCreationError};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano::Version;
use vulkano_win::VkSurfaceBuild;

struct Color [u8, u8, u8, u8]; 

struct Image{
    pixels:Box<[Color]>,
    width:usize,
    height:usize,
}
struct Animation{
    pose: Vec<Image>, //images that make up the animation
    is_active: bool, //if animation has been triggered
    priority: usize, //priority of animation
    timing: Vec<usize>, //how many frames each pose is held 
    frame_triggered: usize, //frame from plate when triggered
    cur_pose: usize, //index of poses
    cycle: bool, //animation is looping or non-looping (aka a cycle or not)

}

struct Spritesheet{
    sheet: Image, //main image, all sprites and animations
    sprites: Vec<Sprite>, //indiviual sprites in sheet
}

struct Sprite{
    id: usize,
    animations: Vec<Animation>,
    default_animation: usize,
    animation_layer: usize,
    active_animation: Vec<usize>, 
}

/*
gets rendered */
struct Plate{
    plate: Image;
    sprite_sheet: SpriteSheet, //sprite sheet
    entities: Vec<Entity>, //list of entities, gives positions and can trigger animations
    cur_frame: usize, //current frame
    dimensions: 
    pipleline: 
    swapchain: 
}

impl Plate{
    
    pub fn new(sheet: File, data: File, entites: Vec<Entity)
    {
        Plate{load_sheet(sheet, data), entities, cur_frame = 0}
    }
    /*
    loads sprite sheet and data about how sheet is divided into sprites
    */
    fn load_sheet(sheet: File, data: File) -> SpriteSheet
    {

    }
}

struct Render{

}
/*
need to figure out what can be shared and what needs to be seperate
need ot set up pipeline
goal: create animation and it can jsut draw based on the entities
*/
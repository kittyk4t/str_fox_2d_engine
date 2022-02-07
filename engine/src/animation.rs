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

struct Animation{
    
}
struct AnimationState
{

}
struct Spritesheet{
    sheet: Vec,
    sprites: Vec<Sprite>,
}

struct Sprite{
    id: usize,
    dimensions: ImageDimensions::Dim2d, 
    animations: Vec<Animation>,
    state: AnimationState,
}

/*
gets rendered */
struct Plate{
    sprite_sheet: SpriteSheet,
    entities: Vec<Entity>,
}

impl Plate{
    
    pub fn new(sheet: File, data: File, entites: Vec<Entity)
    {
        Plate{load_sheet(sheet, data), entities}
    }
    /*
    loads sprite sheet and data about how sheet is divided into sprites
    */
    fn load_sheet(sheet: File, data: File) -> SpriteSheet
    {

    }

    

}

/*
need to figure out what can be shared and what needs to be seperate
need ot set up pipeline
goal: create animation and it can jsut draw based on the entities
*/
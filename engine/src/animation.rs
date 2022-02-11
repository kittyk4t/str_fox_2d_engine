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

impl Color

struct Image{
    pixels:Box<[Color]>,
    width:usize,
    height:usize,
}
impl Image{
    fn new (png: File) -> Image{
        let png_bytes = include_bytes!("../47.png").to_vec();
        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1,
        };
        let mut image_data = Vec::new();
        image_data.resize((info.width * info.height * 4) as u8, 0);
        reader.next_frame(&mut image_data).unwrap();

        Image{image_data.boxed(), info.width, info.height}
    }

    fn sub_image (&self, pos: Vec2, size: Vec2) -> Image{
        let mut temp = Box[Color; 4*pos.x as usize*pos.y as usize];

        //needs to go through and copy only relavent portion of image
    }
}
struct Animation{
    pose: Vec<Image>, //images that make up the animation
    priority: usize, //priority of animation
    timing: Vec<usize>, //how many frames each pose is held 
    cycle: bool, //animation is looping or non-looping (aka a cycle or not)

}

impl Animation{
    fn new() -> Animation{ Animation{Vec<Image>::new(), 0, Vec<usize>::new(), false}}
}

struct AnimationState{
    animation_played: usize,
    is_active: bool, //if animation has been triggered
    is_visiable: bool,
    frame_triggered: usize, //frame from plate when triggered
    cur_pose: usize, //index of pose
}

struct Sprite{
    animations: Vec<Animation>,
    default_animation: usize,
}

struct Spritesheet{
    sheet: Image, //main image, all sprites and animations
    sprites: Vec<Sprite>, //indiviual sprites in sheet
}
impl SpriteSheet{
    fn new(sheet: Image) -> SpriteSheet
    {
        SpriteSheet{sheet, Vec<Sprite>::new()}
    }

   /*
    animation_number: number of animations per sprite, 
            the lenght of this gives the number of distinct sprites
    pose_size: size of the images held by Animation, assumed constant for spritesheet
   */
    fn load_sprites(&mut self, animation_number: Vec<usize>, pose_size: Vec2)
    {
        //number of poses in a animation
        let animation_length = self.sheet.x / pose_size.x as usize;
        let mut temp = Vec<Animation>::new();

        //number of distinct sprites in sprite_sheet
        for i in 0..animation_number.iter()
        {
            temp.add(Animation::new(self.sheet.sub_image()))
        }
    }
}

struct AnimationEntity{
    sprite: Sprite,
    state: AnimationState,
    position: Vec2,
    size: Vec2,
    animation_layer: usize,
}

impl AnimationEntity{
    
}
/*
gets rendered */
struct Plate{
    plate: Image;
    sprite_sheet: SpriteSheet, //sprite sheet
    entities: Vec<Entity>, //list of entities, gives positions and can trigger animations
    cur_frame: usize, //current frame
    anim_entities: Vec<AnimationEntity>,
    dimensions: 
    pipleline: 
    swapchain: 
}

impl Plate{
    
    pub fn new(sheet: File, entites: Vec<Entity)
    {
        Plate{load_sheet(sheet), entities, cur_frame = 0, Vec<AnimationEntity>::new()}
        pair_entity();
    }
    /*
    loads sprite sheet and data about how sheet is divided into sprites
    */
    fn load_sheet(sheet: File) -> SpriteSheet
    {
        let image = Image::new(sheet);

        let sheet = SpriteSheet::new(image);
        sheet.load_sprites()
    }

    fn pair_entity(&mut self: Self) -> (){
        
        for i in entities.iter()
        {
            let entity = entities.get(i);

            self.anim_entities.add(AnimationEntity::new(
                self.sprite_sheet.sprites.get(entity.texture.index),
                entity.texture.pos,
                entity.texture.size,
                entity.texture.animation_layer
            )
        }
    }
    
}

struct Render{

}
/*
need to figure out what can be shared and what needs to be seperate
need ot set up pipeline
goal: create animation and it can jsut draw based on the entities
*/
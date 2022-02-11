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

struct Color{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color{
    fn new(r: u8, g:u8, b:u8, a:u8)-> Color{
        Color{r,g,b,a}
    }

    fn default() -> Color{
        Color{0,0,0,255}
    }

    fn from_array(pixel: [u8; 4])-> Color{
       Color{r: pixel[0],
        g: pixel[1],
        b: pixel[2],
        a: pixel[3],} 
    }
}

struct Image{
    pixels:Vec<Color>,
    width:usize,
    height:usize,
}
impl Image{
    fn new (png: String) -> Image{
        let file = File::open(png).unwrap(); //may or may not be right -> run compiler
        let png_bytes = include_bytes!(file).to_vec();
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
        image_data.resize((info.width * info.height * 4) as usize, 0);
        reader.next_frame(&mut image_data).unwrap();

        let mut color_image = Vec::new();

        for color in image_data.chunks_mut(4)
        {
            color_image.push(Color::from_array(color))
        }

        Image{pixels:color_image, width:info.width, height:info.height}
    }

    fn background (size:Vec2)
    {
        let mut color_image = Vec::new();
       
        for i in 0..((size.y*size.x) as usize) {
            color_image.push(Color::default());
       }
       color_image
    }

    fn sub_image (&self, pos: Vec2, size: Vec2) -> Image{
       let x1 = (pos.x + size.x) as usize; //end of horizontal
       let y1 = (pos.x + size.x) as usize;//end of vertical

       let sub = Vec<Color>::new();
       //goes by height
       for i in (pos.y)..(y1) {
            sub.append(self.pixels[y * self.width + pos.x..(y * self.width + x1)]);//horizontal
       }
       Image{pixels:sub, width:size.x as usize, height:size.y as usize}
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
struct DrawState{
    tb_render: Image;
    sprite_sheet: SpriteSheet, //sprite sheet
    entities: Vec<Entity>, //list of entities, gives positions and can trigger animations
    cur_frame: usize, //current frame
    anim_entities: Vec<AnimationEntity>,
}

impl DrawState{
    
    pub fn new(sheet: File, entites: Vec<Entity>, size: Vec2)-> DrawState {
        DrawState{
        tb_render: Image::backgroun(size),
        sprite_sheet: load_sheet(sheet),
        entities: entities,
        cur_frame: 0,
        anim_entities: Vec::new()}
    }
  
    /*
    loads sprite sheet and data about how sheet is divided into sprites
    */
    fn load_sheet(sheet: File) -> SpriteSheet
    {
        let sheet = SpriteSheet::new(Image::new(sheet));
        sheet.load_sprites();
        sheet
    }

    fn pair_entity(&mut self: Self) -> (){
        
        for i in entities.iter()
        {
            let entity = entities[i];

            self.anim_entities.add(AnimationEntity::new(
                self.sprite_sheet.sprites.get(entity.texture.index),
                entity.texture.pos,
                entity.texture.size,
                entity.texture.animation_layer
            )
        }
    }
    
}

/*
need to figure out what can be shared and what needs to be seperate
need ot set up pipeline
goal: create animation and it can jsut draw based on the entities
*/
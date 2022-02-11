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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

    /**
     * creates an image of the given size which is just the default color
     */
    fn background (size:Vec2)-> Image
    {
        let mut color_image = Vec::new();
       
        for i in 0..((size.y*size.x) as usize) {
            color_image.push(Color::default());
       }
       color_image
    }

    /**
     * creates an image of the given size which is just the default color
     */
    fn background_color (size:Vec2, color: Color)-> Image
    {
        let mut color_image = Vec::new();
       
        for i in 0..((size.y*size.x) as usize) {
            color_image.push(color);
       }
       color_image
    }

    /**
     * returns a cropped portion of the image
     */
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

/*
This is a struct that is used as a reference point for instances of Animation entites,
it holds the difference images that go together to form 1 animated action, as well as the 
data that timings this animation
*/
#[derive(Clone, Copy)]
struct Animation{
    pose: Vec<Image>, //images that make up the animation
    priority: usize, //priority of animation
    timing: Vec<usize>, //how many frames each pose is held 
    cycle: bool, //animation is looping or non-looping (aka a cycle or not)

}

impl Animation{
    fn new() -> Animation{ Animation{poses:Vec<Image>::new(), priority:0, timing:Vec<usize>::new(), cycle:false}}

    fn new_poses(poses: Vec<Image>) -> Animation{Animation{poses, priorirty:0, timing: Vec<usize>::new(), cycle: false}}
}
#[derive(Clone, Copy)]
struct AnimationState{
    animation_played: usize,
    is_active: bool, //if animation has been triggered
    is_visiable: bool,
    frame_triggered: usize, //frame from plate when triggered
    cur_pose: usize, //index of pose
}

/* 
This holds the information for one set of animations on a sprite sheet, 
it tells you all the possible animations for a specific "character" on the sprite sheet
ex on the cat sheet, a sprite would be a Grey Cat which has running and scared animations
 */
#[derive(Clone, Copy)]
struct Sprite{
    animations: Vec<Animation>,
    default_animation: usize,
}
impl Sprite{
    fn new(animations: Vec<Animation>)-> Sprite{Sprite{animations, default_animation: 0}}
}
/*
This holds the sprite sheet image and knows the sprites on the sheet
It represents all the possible things that can be drawn with
 */
#[derive(Clone, Copy)]
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
    ths is a basic load sprite, based on having a consistent pose size
    later would want to move this to reading more data from a file
   */
    fn load_sprites(&mut self, animation_number: Vec<usize>, pose_size: Vec2)
    {
        //number of poses in a animation
        let animation_length = self.sheet.x / pose_size.x as usize;
        let mut temp = Vec<Animation>::new();
        let mut temp_poses = Vec<Image>::new();
        let mut pos = (0.0, 0.0);

        //number of distinct sprites in sprite_sheet
        for i in 0..animation_number.iter()
        {
            //go by number of animations for that sprite
            for j in 0..animation_number[i]{
                //number of poses in an animation
                for k in 0..animation_length{
                    temp_poses.push(self.sheet.sub_image(pos, pose_size)); 
                    pos.x += pose_size.x;
                }
                pos.x = 0.0;
                pos.y += pose_size.y;
                temp.push(Animation::new_poses(temp_poses));
                temp_poses.clear();
            }
            self.sprites.push(Sprite::new(temp));
            temp.clear();
            //need to go by animation and then get each pose
            //end of loop, create and push sprite to sprites list
        }
    }
}
/**
 * this is one instance of an animated object, it is connected with a sprite which dictates
 * what animations is can perform
 * the positioon and animation layer is based on the game entity it is connected to
 */
#[derive(Clone, Copy)]
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
This is what is parellel to the game state and handles the changes to
make the images that are displayed match what has occured in the game */
#[derive(Clone, Copy)]
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
        tb_render: Image::background(size),
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
        
        //think this syntax works?
        for entity in entities.iter()
        {
            self.anim_entities.add(AnimationEntity::new(
                self.sprite_sheet.sprites.get(entity.texture.index),
                entity.texture.pos,
                entity.texture.size,
                entity.texture.animation_layer
            )
        }
    }

    fn sync_entity(&mut self: Self)-> (){
        
    }
    
}

/*
need to figure out what can be shared and what needs to be seperate
need ot set up pipeline
goal: create animation and it can jsut draw based on the entities
*/
use std::sync::Arc;
use serde;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

use super::types::*; 
use super::image::*;
use super::entity::*;
use super::engine_core::*;
/*
This is a struct that is used as a reference point for instances of Animation entites,
it holds the difference images that go together to form 1 animated action, as well as the 
data that timings this animation
*/
#[derive(Clone)]
pub struct Animation{
    id: usize,
    pub pose: Vec<Image>, //images that make up the animation
    priority: usize, //priority of animation
    timing: Vec<usize>, //how many frames each pose is held 
    cycle: bool, //animation is looping or non-looping (aka a cycle or not)
    retrigger: bool, //animation can be retrigged
    pause: bool //can be paused

}

impl Animation{
    fn new() -> Animation{ Animation{id: 0, pose:Vec::<Image>::new(), 
        priority:0, timing:Vec::<usize>::new(), cycle:false, retrigger: false, pause: false}}

    fn new_poses(id: usize, pose: Vec<Image>) -> Animation{Animation{id, pose, priority:0, 
        timing: Vec::<usize>::new(), cycle: false, retrigger: false, pause: false}}
}
#[derive(Clone)]
pub struct AnimationState{
    animation: Animation, //index for sprite animations
    is_visible: bool,
    frame_triggered: usize, //frame from plate when triggered
    cur_pose: usize, //index of pose
    is_finished: bool
}

impl AnimationState{
    fn new(animation: Animation, is_visible: bool, frame_triggered: usize) -> AnimationState {AnimationState{animation, is_visible, frame_triggered, cur_pose:0, is_finished: false}}
   
    fn tick(&mut self, cur_frame: usize) -> (){
        if self.frame_triggered + self.animation.timing[self.cur_pose] == cur_frame{
            self.cur_pose += 1;

            if self.cur_pose >= self.animation.pose.len(){
                self.cur_pose = 0;
                self.is_finished = !self.animation.cycle; 
            }
        }

    }
    fn current_frame(&self) -> usize{
        self.cur_pose
    }

    fn is_finished(&self)-> bool {
        self.is_finished

    }
}

#[derive(Clone)]
pub struct AnimQueue {
    queue:Vec<(f32,AnimationState,bool)>
}
impl AnimQueue {
    fn new() -> AnimQueue{AnimQueue{queue: Vec::<(f32,AnimationState,bool)>::new()}}
    fn push(&mut self, p:f32, anim:AnimationState, pause:bool, retrigger:bool) {
        // If this is a retrigger, replace the old animation (if any)
        // otherwise, leave the old animation alone!
        let to_insert = if let Some(found_pos) = self.queue.iter().position(|(qp, qanim, _)| qanim.animation.id == anim.animation.id) {
            let (_qp, qanim, _qpause) = self.queue.remove(found_pos);
            if retrigger {
                //HAVE SOMEONE LOOK AT THIS
                (p, anim.clone(), pause)
            } else {
                (p, qanim, pause)
            }
        } else {
            //HAVE SOMEONE LOOK AT THIS
            (p, anim.clone(), pause)
        };
        // put highest priority thing at end
        let pos = self.queue.iter().rposition(|(qp, _, _)| qp < &p).map(|n| n+1).unwrap_or(0);
        self.queue.insert(pos, (p, anim, pause));
    }
    fn tick(&mut self, cur_frame: usize) {
        let qlen = self.queue.len();
        // tick possibly-paused non-current animations
        if qlen > 1 {
            for (_p, anim, pause) in self.queue.iter_mut().take(qlen-1) {
                if ! *pause { anim.tick(cur_frame); }
            }
        }
        // ignore pause for topmost anim if any and tick it
        if let Some((_,active,_)) = self.queue.last_mut() {
            active.tick(cur_frame);

        }
        // Throw away finished animations
        self.queue.retain(|(_p, anim, _)| !anim.is_finished());
    }
    // Got to return option here---nothing to return if we have no animations in the queue!
    fn current_frame(&self) -> Option<usize> {
        self.queue.last().map(|(_,anim,_)| anim.current_frame())
    }

    fn current_animation(&self) -> Option<usize> {
        self.queue.last().map(|(_,anim,_)| anim.animation.id)
    }
}
 //self.queue.last().unwrap().1.tick(cur_frame);

/* 
This holds the information for one set of animations on a sprite sheet, 
it tells you all the possible animations for a specific "character" on the sprite sheet
ex on the cat sheet, a sprite would be a Grey Cat which has running and scared animations
 */
#[derive(Clone)]
pub struct Sprite{
    pub animations: Vec<Animation>,
    pub default_animation: usize,
}
impl Sprite{
    fn new(animations: Vec<Animation>)-> Sprite{Sprite{animations, default_animation: 0}}
}
/*
This holds the sprite sheet image and knows the sprites on the sheet
It represents all the possible things that can be drawn with
 */
#[derive(Clone)]
pub struct SpriteSheet{
    pub sheet: Image, //main image, all sprites and animations
    pub sprites: Vec<Sprite>, //indiviual sprites in sheet
}
impl SpriteSheet{
   pub  fn new(sheet: Image) -> SpriteSheet
    {
        SpriteSheet{sheet, sprites: Vec::<Sprite>::new()}
    }

   /*
    animation_number: number of animations per sprite, 
            the lenght of this gives the number of distinct sprites
    pose_size: size of the images held by Animation, assumed constant for spritesheet
    ths is a basic load sprite, based on having a consistent pose size
    later would want to move this to reading more data from a file
   */
     pub fn load_sprites(&mut self, animation_number: Vec<usize>, pose_size: Vec2i) {
        //number of poses in a animation
        let animation_length = self.sheet.sz.x / pose_size.x;
        
        let mut temp = Vec::<Animation>::new();
        let mut temp_poses = Vec::<Image>::new();
        let mut pos = Vec2i{x:0, y:0};

        //number of distinct sprites in sprite_sheet
        for i in animation_number.iter() {
            //go by number of animations for that sprite
            for _j in 0..*i {
                //number of poses in an animation
                for _k in 0..animation_length {
                   
                    temp_poses.push(self.sheet.sub_image(pos, pose_size));
                    pos.x += pose_size.x;
                }
                
                pos.x = 0;
                pos.y += pose_size.y;
                temp.push(Animation::new_poses(*i, temp_poses.clone()));
                temp_poses.clear();
            }
            self.sprites.push(Sprite::new(temp.clone()));
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
#[derive(Clone)]
pub struct AnimationEntity{
    sprite: Sprite,
    states: AnimQueue,
    pos: Vec2,
    size: Vec2i,
    animation_layer: usize,
}

impl AnimationEntity{

    pub fn new( id: usize,sprite: Sprite, states: AnimQueue, pos: Vec2, size: Vec2i, animation_layer: usize,) -> 
    AnimationEntity{AnimationEntity{id, sprite, states, pos, size, animation_layer} }

    pub fn to_Rect(&self) -> Rect {
        let image = self.pose();
        
        Rect::new(self.pos.to_Vec2i(), image.sz)
        
    }
    //returns current pose
    pub fn pose(&self )-> Image{
        //queue.current_frame() retunrs the current frame of animatino; need to figure out the animation being played
        self.sprite.animations[self.states.current_animation().unwrap()].pose[self.states.current_frame().unwrap()].clone()
    }

    pub fn trigger_animation(&mut self, animation: usize, is_visible: bool, frame: usize) -> (){
        let animation = &self.sprite.animations[animation];
        let mut state = AnimationState::new(animation.clone(),is_visible, frame);
        self.states.push(animation.priority as f32, state, animation.retrigger, animation.pause);
    }

    pub fn tick(&mut self, frame: usize){
        self.states.tick(frame);
    }
    
}


/*
This is what is parellel to the game state and handles the changes to
make the images that are displayed match what has occured in the game */
#[derive(Clone)]
pub struct DrawState{
    tb_render: Image,
    sprite_sheet: SpriteSheet, //sprite sheet
    cur_frame: usize, //current frame
    anim_entities: HashMap<Entity, AnimationEntity>,
}

impl DrawState{
    
    pub fn new(sheet: &std::path::Path, anim_num: Vec<usize>, pose_sz: Vec2i, entities: Vec<Entity>, size: Vec2i)-> DrawState {
        DrawState{
        tb_render: Image::new(size),
        sprite_sheet: DrawState::load_sheet(sheet, anim_num, pose_sz),
        cur_frame: 0,
        anim_entities: DrawState::init_anim_enitities(entities)}
    }
  
    /*
    loads sprite sheet and data about how sheet is divided into sprites
    */
    fn load_sheet(sheet: &std::path::Path, anim_num: Vec<usize>, pose_sz: Vec2i) -> SpriteSheet
    {
        let mut sheet = SpriteSheet::new(Image::from_file(sheet));
        sheet.load_sprites(anim_num, pose_sz);
        sheet
    }

    fn init_anim_enitities(&mut self, entities: Vec<Entity>) -> HashMap<Entity, AnimationEntity>{
        let mut map = HashMap::new();
        for entity in self.entities.iter()
        {
            map.insert(entity,AnimationEntity::new(
                self.sprite_sheet.sprites[entity.texture.index].clone(),
                AnimQueue::new(),
                entity.pos,
                entity.size,
                entity.texture.animation_layer
            ));
        }
    }

    fn sync_entity(&mut self, entities: Vec<Entity>)-> (){
        //remove anim_entites whose entities are gone?
        for entity in self.entities.iter(){
            match self.anim_entities.get_mut(entity){
                None => {
                    self.anim_entities.insert(entity, AnimationEntity::new(
                self.sprite_sheet.sprites[entity.texture.index].clone(),
                AnimQueue::new(),
                entity.pos,
                entity.size,
                entity.texture.animation_layer
            ));
                }, 
                Some(key, anim_entity) => anim_entity.pos = key.pos;
            }
            
        }
    }

    pub fn trigger_animation(&mut self, entity: Entity, anim_id: usize)-> (){
        match self.anim_entities.get_mut(entity){
            None => {
                let mut new = AnimationEntity::new(
                self.sprite_sheet.sprites[entity.texture.index].clone(),
                AnimQueue::new(),
                entity.pos,
                entity.size,
                entity.texture.animation_layer
            );
            new.trigger_animation(anim_id, entity.texture.is_visible, self.cur_frame);
            self.anim_entities.insert(entity, new);
                
            },
            Some(key, anim_enity) =>{
                anim_entity.trigger_animation(anim_id, entity.texture.is_visible, self.cur_frame);
            }
        }
    }
    fn remove_entity()
    
    //returns a clone of the draw state
    fn incr_frame(&mut self, entities: Vec<Entity>) -> (){
        self.sync_entity(entites);

        //will need to check syntax
        for (entity, anim_entity) in &self.anim_entities.iter().unwrap(){
            self.tb_render.bitblt(&anim_entity.pose(), anim_entity.to_Rect(), anim_entity.pos.to_Vec2i());
            anim_entity.tick(self.cur_frame);
        }
        self.cur_frame += 1;
    }

    pub fn load_buffer(&mut self, vulkan_config:  &mut VulkanConfig) -> ()
    {
        self.incr_frame();
        let rect = Rect{pos:Vec2i::new(0,0), sz: self.tb_render.sz};
        vulkan_config.fb2d.bitblt(&self.tb_render, rect, Vec2i::new(0,0));
    }
}

/*
need to figure out what can be shared and what needs to be seperate
need ot set up pipeline
goal: create animation and it can jsut draw based on the entities
*/
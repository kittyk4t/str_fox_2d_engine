/*
entity main structure

deserialize -> json

entities could have an enum
on_collsion inside hurtbox, matches on that enum and determines behavior

score -> display entity, null fields

enum team:
good
bad

entity TYPES:
player
enemy
projectile<good, bad>
display (preview, display lives, text_box ->defines the space)
background

position -> vec2
hurt box -> rectangle
--> relative position to entity x,y
--> collision layer
--> on_collison
texture ->
uv to reference, all that info whoo

animation:
define a spritesheet, then ahve the game share one spritesheet
struct spritesheet
image:
data: slices for each indivial sprite
know the positon and size of each sprite and then you load
get all the sprites

draw_sprite(id) -> data

struct texture
index/id: -> usize abstract until have access to sprite sheet
postion: vec2
size: vec2
is_visible: bool


struct hurt_box impl Colliable
position: vec2 <- constant b/c relative to entity
size: vec2
collision_layer: usize
check_collison: bool

impl Colliable for hurt_box/specific thing

struct entity has trait update -> defines default behavior
type: ENUM
positon: vec2
vel: vec2
hurt box: hurt_box type
texture: texture struct

impl
on_colision (correct typing yay) -> return whoo
{};

game
impl same thing
on_collision () ->
{actual code}

abstract on collision method
*/

use super::types::*;
use std::hash::{Hash};

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Team {
    Good,
    Bad,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum DisplayType {
    Preview,
    Lives,
    TextBox,
}
#[derive(Clone, Debug, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum EntityType {
    Player,
    Enemy,
    Projectile,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Texture {
    pub index: usize,
    pub is_visible: bool,
}

pub trait Colliable {
    fn on_collison(&self);
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HurtBox {
    pub pos: Vec2,
    pub size: Vec2i,
}
impl HurtBox{
    pub fn new(size: Vec2i) -> HurtBox{
        HurtBox{pos: Vec2::new(0.0, 0.0),
        size: size,
    }}
    pub fn touching(&self, other: HurtBox)  -> bool{
        let this_rect = Rect::new(self.pos.to_vec2i(), self.size);
        let other_rect = Rect::new(other.pos.to_vec2i(), other.size);
        
       super::collision::rect_touching(this_rect, other_rect)
    }
}


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Entity {
    pub id: usize,
    pub ent_type: EntityType,
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub size: Vec2i,
    pub hurt_box: HurtBox,
    pub texture: Texture,
}

impl Entity{
    pub fn update_hurtbox(&mut self) -> (){
       let off_x = (self.size.x - self.hurt_box.size.x).abs() /2;
       let off_y = (self.size.y - self.hurt_box.size.y).abs() /2;

       self.hurt_box.pos = Vec2::new(self.pos.x + off_x as f32, self.pos.y + off_y as f32);
        
    }

    pub fn change_motion( &mut self, stop: bool, move_x: Vec2b, move_y: Vec2b) {
    let mut xd = 1.0;
    let mut yd = 1.0;
    if !move_x.move_positive{
        xd = -1.0;
    }
    if !move_y.move_positive{
        yd = -1.0;
    }

    match self.ent_type{
        EntityType::Player => {
            if stop{
                self.vel.x = 0.0;
            }
            else{

            if move_x.move_axis{
                if self.vel.x <= 0.000001{
                self.vel.x = xd;
            }
            else{
                self.vel.x *= xd;
            }

            }
            
            }
            
        },
        _ => {
            if stop{
                self.acc = Vec2::new(0.0, 0.0);
            }
            else{
                if move_x.move_axis{
                    if self.acc.x <= 0.000001{
                    self.acc.x = 2.0 * xd;
                }
                else{
                    self.acc.x *= xd;
                }

                }
                
                if move_y.move_axis{
                    if self.acc.y <= 0.000001{
                    self.acc.y = 2.0 * yd;
                }
                else{
                    self.acc.y *= yd;
                }

                }
                
                
            }
        }

    }
}
pub fn compute_distance(&mut self, time_constant: f32, world_sz: Vec2i) -> (){
    match self.ent_type{
        EntityType::Player =>{
            self.pos.x += self.vel.x;

            if (self.pos.x + self.size.x as f32) as i32 > world_sz.x {
            self.pos.x = (world_sz.x as f32) - self.size.x as f32 - 1.0;
        } else if self.pos.x as usize <= 0 {
            self.pos.x = 0.0;
        }

        },
        _ => {
            self.vel.x += self.acc.x * time_constant;
            self.pos.x += self.vel.x *time_constant;
            self.vel.y += self.acc.y * time_constant;
            self.pos.y += self.vel.y *time_constant;

            if (self.pos.x + self.size.x as f32) as i32 > world_sz.x {
            self.pos.x = (world_sz.x as f32) - self.size.x as f32 - 1.0;
        } else if self.pos.x as usize <= 0 {
            self.pos.x = 0.0;
        }

        if (self.pos.y + self.size.y as f32) as i32 > world_sz.y {
            self.pos.y = (world_sz.y as f32) - self.size.y as f32 - 1.0;
        } else if self.pos.y as usize <= 0 {
            self.pos.y = 0.0;
        }

        },
        
    }
}

pub fn collided(&self, other: &Entity) -> bool{
    self.hurt_box.touching(other.hurt_box)
}

}










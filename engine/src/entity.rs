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
use serde;
use serde::Deserialize;
use serde_json;
use super::types::*;
use std::hash::{Hash, Hasher};

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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum EntityType {
    Player,
    Enemy,
    Projectile(Team),
    Display,
    Background,
}

#[derive(Clone, Hash)]
pub struct Texture {
    pub index: usize,
    pub is_visible: bool,
    pub animation_layer: usize,
}

pub trait Colliable {
    fn on_collison(&self);
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct HurtBox {
    pos: Vec2,
    size: Vec2,
    collision_layer: usize,
    check_collison: bool,
}

#[derive(Clone, Hash)]
pub struct Entity {
    pub id: usize,
    pub ent_type: EntityType,
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2i,
    pub hurt_box: HurtBox,
    pub texture: Texture,
}




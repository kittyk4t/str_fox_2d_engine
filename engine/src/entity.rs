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

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Team
{
    Good,
    Bad,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum DisplayType
{
    Preview,
    Lives,
    TextBox,
}
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum EntityType {
Player,
Enemy,
Projectile(Team),
Display,
Background,
}


#[derive(Clone, Copy, Debug, Deserialize)]
struct Texture{
    index: usize,
    postion: Vec2,
    size: Vec2,
    is_visible: bool,
}

pub trait Colliable
{
   fn on_collison(&self) ; 
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct HurtBox{
    position: Vec2,
    size: Vec2,
    collision_layer: usize,
    check_collison: bool,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct Entity{
    ent_type: EntityType,
    positon: Vec2,
    vel: Vec2,
    hurt_box: HurtBox,
    texture: Texture,
}



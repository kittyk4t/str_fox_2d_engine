use std::hash::{Hash, Hasher};


#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn to_Vec2i(self) -> Vec2i {Vec2i{x: self.x as i32, y:self.y as i32}}
}

//this is bad if hash were to just be used on Vec2...
//BUT Vec2 should only ever be hashed in the entity
//ask how to make this suck less
impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let vec = self.to_Vec2i();
        vec.hash(state);
    }
}

impl PartialEq for Vec2{
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() > 0.0001 && (self.y - other.y).abs() > 0.0001 
    }
}

impl Eq for Vec2{}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}
impl Vec2i{
    pub fn new(x: i32, y:i32) -> Vec2i{
        Vec2i{x,y}
    }
}
impl std::ops::Add<Vec2i> for Vec2i {
    type Output = Self;

    fn add(self, other: Vec2i) -> <Self as std::ops::Add<Vec2i>>::Output {
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    pub pos: Vec2i,
    pub sz: Vec2i,
}

impl Rect {
    pub fn new(pos: Vec2i, sz: Vec2i) -> Rect{Rect{pos,sz}}
    pub fn contains(&self, other: Rect) -> bool {
        let br = self.pos + self.sz;
        let obr = other.pos + other.sz;
        self.pos.x <= other.pos.x && self.pos.y <= other.pos.y && obr.x <= br.x && obr.y <= br.y
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
   pub  r: u8,
    pub g: u8,
   pub  b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn default() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn from_array(pixel: &[u8]) -> Color {
        assert!(pixel.len() == 4);
        Color {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
            a: pixel[3],
        }
    }
}

unsafe impl vulkano::format::Pixel for Color {
    fn ensure_accepts(
        format: vulkano::format::Format,
    ) -> std::result::Result<(), vulkano::format::IncompatiblePixelsType> {
        // TODO: Be more strict: accept only if the format has a matching AcceptsPixels impl.
        if format.size().map_or(false, |x| {
            x % std::mem::size_of::<Self>() as vulkano::DeviceSize == 0
        }) {
            Ok(())
        } else {
            Err(vulkano::format::IncompatiblePixelsType)
        }
    }
    fn rate(format: vulkano::format::Format) -> u32 {
        (format.size().expect("this format cannot accept pixels")
            / std::mem::size_of::<Self>() as vulkano::DeviceSize) as u32
    }
}

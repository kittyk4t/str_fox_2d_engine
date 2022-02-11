pub struct Animation {
    // Do this for the exercise today!
    // You'll want to know the frames involved and the timing for each frame
    frames: Vec<Image>,
    timing: Vec<usize>,
    cycle: bool,
}

pub struct AnimationState {
    // Here you'll need to track how far along in the animation you are.
    // You can also choose to have an Rc<Animation> to refer to the animation in use.
    // But you could also decide to just pass around the animation and state together
    // where needed.
    progress: usize,
}

impl Animation {
    // Should hold some data...
    // Be used to decide what frame to use...
    // Could have a query function like current_frame(&self, start_time:usize, now:usize, speedup_factor:usize)
    // Or could be ticked in-place with a function like tick(&self)
}

use std::rc::Rc;

pub struct Sprite {
    image: Rc<Image>,
    // For example, but this is just one way to do it:
    animations:Vec<Animation>,
    animation_state:AnimationState,
}

impl Sprite {
    // maybe some play_animation() function to start a new animation!
    // maybe some draw() function to draw the sprite!
    // and a tick_animation() function to advance the animation state
}
pub trait DrawSpriteExt {
    fn draw_sprite(&mut self, s: &Sprite, pos:Vec2i);
}

use crate::image::Image;
impl DrawSpriteExt for Image {
    fn draw_sprite(&mut self, s: &Sprite, pos:Vec2i) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of Sprite are visible inside this module
        self.bitblt(&s.image, s.animation_state.current_frame(), pos);
    }
}

struct AnimQueue {
    queue:Vec<(f32,AnimationState,bool)>
}
impl AnimQueue {
    fn push(&mut self, p:f32, anim:AnimationState, pause:bool, retrigger:bool) {
        // If this is a retrigger, replace the old animation (if any)
        // otherwise, leave the old animation alone!
        let to_insert = if let Some(found_pos) = queue.iter().position(|(qp, qanim, _)| qanim.animation == anim) {
            let (_qp, qanim, _qpause) = queue.remove(found_pos);
            if retrigger {
                (p, anim, pause)
            } else {
                (p, qanim, pause)
            }
        } else {
            (p, anim, pause)
        };
        // put highest priority thing at end
        let pos = queue.iter().rposition(|(qp, _, _)| qp < p).unwrap_or(0);
        queue.insert(pos, (p, anim, pause));
    }
    fn tick(&mut self) {
        let qlen = self.queue.len();
        // tick possibly-paused non-current animations
        if qlen > 1 {
            for (_p, anim, pause) in self.queue.iter_mut().take(qlen-2) {
                if !pause { anim.tick(); }
            }
        }
        // ignore pause for topmost anim if any and tick it
        if let Some((_,active,_)) = self.queue.last() {
            active.tick();
        }
        // Throw away finished animations
        self.queue.retain(|(_p, anim, _)| !anim.is_finished());
    }
    // Got to return option here---nothing to return if we have no animations in the queue!
    fn current_frame(&self) -> Option<Rect> {
        self.queue.last().map(|(_,anim,_)| anim.current_frame())
    }
}
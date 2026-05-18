use raylib::prelude::Rectangle;

#[derive(Debug, Clone)]
pub struct AnimFrame {
    pub rec: Rectangle,
    pub duration: f32,
}

#[derive(Debug, Clone)]
pub struct Anim {
    pub frames: Vec<AnimFrame>,
    pub looping: bool,
}

#[derive(Debug, Clone)]
pub struct AnimState {
    pub current_frame: usize,
    pub timer: f32,
    pub anim: Anim,
}

impl AnimState {
    pub fn new(anim: Anim) -> Self {
        Self {
            current_frame: 0,
            timer: 0.0,
            anim,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.anim.frames.is_empty() {
            return;
        }

        self.timer += delta;

        let frame_duration = self.anim.frames[self.current_frame].duration;
        if self.timer >= frame_duration {
            self.timer -= frame_duration;

            if self.current_frame + 1 < self.anim.frames.len() {
                self.current_frame += 1;
            } else if self.anim.looping {
                self.current_frame = 0;
            } else {
                self.current_frame = self.anim.frames.len() - 1;
                self.timer = frame_duration;
            }
        }
    }

    pub fn current_rect(&self) -> &Rectangle {
        &self.anim.frames[self.current_frame].rec
    }

    pub fn switch_to(&mut self, new_anim: Anim, reset: bool) {
        self.anim = new_anim;
        if reset {
            self.current_frame = 0;
            self.timer = 0.0;
        }
    }
}

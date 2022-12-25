use sdl2::sys::Uint32;
pub struct FrameTimer {
    last_time: u32,
    frame_time: u32,
    delta: f32,
    start: u32,
    stop: u32,
}

pub const MS_TO_SECONDS: f32 = 1.0 / 1000.0;
impl FrameTimer {
    pub fn new() -> FrameTimer {
        FrameTimer {
            last_time: 0,
            frame_time: 0,
            delta: 0.0,
            start: 0,
            stop: 0,
        }
    }

    fn time_now(&self) -> Uint32 {
        #[allow(unsafe_code)]
        unsafe {
            sdl2::sys::SDL_GetTicks()
        }
    }

    pub fn time_start(&mut self) {
        self.frame_time = self.stop - self.start;
        self.delta = self.frame_time as f32 * MS_TO_SECONDS;
        self.start = self.time_now();
    }
    pub fn time_stop(&mut self) {
        self.stop = self.time_now();
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }
}

use crate::millis;

pub struct Stopwatch {
    time: u32,
    previous: u32,
    // last: u32,
    running: bool,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            time: 0,
            previous: 0,
            // last: 0,
            running: false,
        }
    }

    pub fn start(&mut self) {
        // self.time = self.last;
        self.time = 0;
        self.running = true;
        self.previous = millis();
    }

    #[inline]
    fn update(&mut self) {
        // self.time = self.last + (millis() - self.previous);
        self.time = millis() - self.previous;
    }

    pub fn get_time(&mut self) -> u32 {
        if self.running {
            self.update();
        }
        self.time
    }
    pub fn stop(&mut self) {
        self.update();
        self.running = false;
        // self.last = self.time;
    }

    pub fn reset(&mut self) {
        self.time = 0;
        self.previous = 0;
        self.running = false;
    }
}

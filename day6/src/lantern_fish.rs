
#[derive(Debug)]
pub struct LanternFish {
    timer: u32,
    gestation_time: u32,
}

impl LanternFish {
    pub fn new(init_timer: u32, gestation_time: u32) -> LanternFish {
        LanternFish {
            timer: init_timer,
            gestation_time,
        }
    }

    /// Aging the fish of one day. Returns true if this fish gave birth to a new one.
    pub fn age_one_day(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = self.gestation_time - 1;
            return true;
        }
        else {
            self.timer -= 1;
            return false;
        }
    }
}
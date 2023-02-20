use super::Device;

#[derive(Clone)]
pub struct Tv {
    on: bool,
    volume: u8,
    channel: u16,
}

impl Default for Tv {
    fn default() -> Self {
        Self {
            on: false,
            volume: 0,
            channel: 0
        }
    }
}

impl Device for Tv {
    fn is_enabled(&self) -> bool {
        self.on
    }
    fn enable(&mut self) {
        self.on = true
    }
    fn disable(&mut self){
        self.on = false
    }
    fn volume(&self) -> u8 {
        self.volume
    }
    fn set_volume(&mut self, percent: u8) {
        self.volume = percent
    }
    fn channel(&self) -> u16 {
        self.channel
    }
    fn set_channel(&mut self, channel: u16) {
        self.channel = channel
    }
    fn print_status(&self) {
        println!("------------------------------------");
        println!("| I'm a tv.");
        println!("| I'm {}", if self.on { "enabled" } else { "disabled" });
        println!("| Current volume is {}%", self.volume);
        println!("| Current channel is {}", self.channel);
        println!("------------------------------------\n");
    }
}


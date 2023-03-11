/// A music track.
pub struct Track {
    pub title: String,
    pub duration: u32,
    cursor: u32,
}


impl Track {
    pub fn new(title: &'static str, duration: u32) -> Self {
        Self { 
            title: title.to_string(), 
            duration: duration, 
            cursor: 0 
        }
    }
}

/// A music player holds a playlist and it can do basic operations over it.
pub struct Player {
    playlist: Vec<Track>,
    current_track: usize,
    _volume: u8,
}


impl Default for Player {
    fn default(){}    
}

impl Player {
    pub fn next_track(&mut self) {
    }

    pub fn prev_track(&mut self) {
    }

    pub fn play(&mut self) {
    }

    pub fn pause(&mut self) {
    }

    pub fn rewind(&mut self) {
    }

    pub fn track(&self) -> &Track {
    }

    fn track_mut(&mut self) -> &mut Track {
    }
}
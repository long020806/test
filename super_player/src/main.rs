mod media;
use std::str::FromStr;

use media::Playable;


struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("paly now {}",self.0);
    }
} 
impl Playable for Video {
    fn play(&self) {
        println!("paly now {}",self.0);
    }
} 

impl Default for Audio {
    fn default() -> Self {
        let from_str = String::from_str("default audio").unwrap();
        let audio = Self(from_str);
        audio
    }
}
fn main() {
    println!("Super player!");
    let audio = Audio("ambient_music.mp3".to_string()); 
    let video = Video("big_buck_bunny.mkv".to_string()); 
    audio.play(); 
    video.play(); 
}

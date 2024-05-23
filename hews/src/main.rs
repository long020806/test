mod app; 
mod hackernews; 
use app::App; 
fn main() { 
    let (app, rx) = App::new(); 
    app.launch(rx); 
}
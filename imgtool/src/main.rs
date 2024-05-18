use std::{env, path::Path};



fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);
    let image = image::open(path).unwrap();
    let rotate_image = image.rotate90();
    rotate_image.save(path).unwrap();
}

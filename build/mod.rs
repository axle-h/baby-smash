use crate::sound::build_sound_modules;
use crate::sprite_sheet::build_sprite_sheets;

mod opaque;
mod geometry;
mod sprite;
mod triangulate;
mod sprite_sheet;
mod sound;

const ROOT_DIR: &str = "./src/assets/";

fn main() {
    build_info_build::build_script();

    embed_resource::compile("icon.rc", embed_resource::NONE);

    println!("0");
    build_sound_modules(ROOT_DIR).unwrap();
    build_sprite_sheets(ROOT_DIR).unwrap()
}




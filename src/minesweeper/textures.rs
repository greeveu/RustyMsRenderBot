pub fn load_textures() -> Vec<u8> {
    let skin_full: Vec<u8> = include_bytes!("../../resources/skin_full.png").to_vec();
    // let skin_gif: Vec<u8> = include_bytes!("../resources/skin_20.png").to_vec();

    skin_full
    // if metadata.x_size >= 32 || metadata.y_size >= 32 {
    //     skin_full
    // } else {
    //     skin_gif
    // }
}

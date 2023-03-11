// #[macro_use]
// extern crate rocket;
//
// use rocket::http::ContentType;
// use rocket::{Build, Rocket};
//
// use crate::parser::{
//     parse_flag_data, parse_meta_data, parse_mine_data, parse_open_data, ApiData, ParsedData,
// };
// use crate::renderer::Renderer;
// use crate::textures::load_textures;
//
// mod base62;
// mod error;
// mod minesweeper_logic;
// mod parser;
// mod renderer;
// mod textures;
//
// #[get("/render/<game_id>")]
// async fn render(game_id: usize) -> (ContentType, Vec<u8>) {
//     let data = fetch_data(game_id);
//     let option = data.split_once('=').expect("Unable to get Version");
//
//     if option.0.eq("1") {
//         let split: Vec<&str> = option.1.split('+').collect();
//         let data = parse_v1(
//             split[0].trim(),
//             split[1].trim(),
//             split[2].trim(),
//             split[3].trim(),
//         );
//
//         let sprite = load_textures();
//
//         let mut renderer = Renderer::new(
//             data.metadata,
//             data.game_board,
//             data.open_data,
//             data.flag_data,
//             sprite.as_slice(),
//         );
//         return (ContentType::JPEG, renderer.render_jpeg().expect("err"));
//     } else {
//         println!("Unknown / Unsupported version");
//     }
//
//     (ContentType::JPEG, vec![1_u8])
// }
//
// #[launch]
// fn rocket() -> Rocket<Build> {
//     rocket::build().mount("/", routes![render])
// }
//
// fn fetch_data(gameid: usize) -> String {
//     let request_data =
//         ureq::get(format!("https://api.greev.eu/v2/stats/minesweeper/game/{gameid}").as_ref())
//             .call()
//             .expect("Unable to fetch Data")
//             .into_string()
//             .expect("Unable to parse Data");
//
//     let v: ApiData = serde_json::from_str(request_data.as_ref()).expect("Unable to parse Data");
//     v.game_data
// }
//
// fn parse_v1(
//     raw_meta: &str,
//     raw_mine_data: &str,
//     raw_open_data: &str,
//     raw_flag_data: &str,
// ) -> ParsedData {
//     let metadata = parse_meta_data(raw_meta);
//
//     ParsedData {
//         game_board: parse_mine_data(raw_mine_data, &metadata),
//         open_data: parse_open_data(raw_open_data),
//         flag_data: parse_flag_data(raw_flag_data),
//         metadata,
//     }
// }

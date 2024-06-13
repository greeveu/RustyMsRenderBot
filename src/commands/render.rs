use std::borrow::Cow;

use chrono::{NaiveDateTime, Timelike};
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::MessageFlags;
use serenity::model::channel::AttachmentType::Bytes;
use serenity::utils::Color;

use crate::commands::error::CommandError;
use crate::minesweeper::parsers;
use crate::minesweeper::parsers::parser::{Iparser, ParsedData};
use crate::minesweeper::provider::greev::greev_provider::GreevProvider;
use crate::minesweeper::provider::mcplayhd::mcplay_provider::McPlayHdProvider;
use crate::minesweeper::provider::provider::{ApiData, PlayerData, Provider};
use crate::minesweeper::renderer::Renderer;

const DEFAULT_PROVIDER: &str = "mcplayhd";

pub(crate) async fn run(command: &ApplicationCommandInteraction, ctx: &Context) {
    let game_id = command.data.options.iter().find(|x| x.name.eq("game_id"));
    let use_gif = command.data.options.iter().find(|x| x.name.eq("gif"));
    let option_provider = command.data.options.iter().find(|x| x.name.eq("provider"));

    if game_id.is_none() || game_id.unwrap().value.as_ref().is_none() {
        error_response(command, ctx, "Please provide a game id").await;
        return;
    }

    let game_id = game_id
        .expect("GameID is empty")
        .value
        .as_ref()
        .expect("Unable to get value")
        .as_str()
        .expect("Unable to get the GameID as str");

    let gif = use_gif
        .map(|x| x.value.as_ref().unwrap().as_bool().unwrap_or(false))
        .unwrap_or(false);

    let possible_providers: Vec<&dyn Provider> = vec![&GreevProvider, &McPlayHdProvider];

    let provider = option_provider
        .map(|x| x.value.as_ref().unwrap().as_str().unwrap().to_lowercase())
        .unwrap_or(DEFAULT_PROVIDER.to_string());

    let optional_provider = possible_providers
        .iter()
        .find(|x| x.id() == provider.as_str());

    if optional_provider.is_none() {
        error_response(command, ctx, "Unknown Provider").await;
        return;
    }

    let provider = optional_provider.unwrap();

    let result_api_data = provider.fetch_data(game_id);

    if result_api_data.is_err() {
        error_response(command, ctx, "Unable to fetch game data").await;
        return;
    }

    let api_data = result_api_data.unwrap();

    let image_data_result = get_image_data(&api_data, &gif).await;

    if let Err(error) = image_data_result {
        error_response(command, ctx, error.to_string().as_str()).await;
        return;
    }

    let timestamp = NaiveDateTime::from_timestamp_millis(api_data.time as i64)
        .expect("Unable to get Timestamp from time");

    let player_data = provider
        .fetch_name(api_data.uuid.as_str())
        .unwrap_or_else(|_| PlayerData {
            name: "%".to_string(),
        });

    let result = match provider.id() {
        "greev" => {
            command
                .create_followup_message(&ctx.http, |message| {
                    let msg = message.embed(|e| {
                        e.title(format!("[Greev] Minesweeper Game {}", game_id))
                            .field("Username", player_data.name, true)
                            .field(
                                "Time",
                                format!(
                                    "{:02}:{:02}:{:02}.{:03}",
                                    timestamp.hour(),
                                    timestamp.minute(),
                                    timestamp.second(),
                                    timestamp.nanosecond() / 1_000_000
                                ),
                                true,
                            )
                            .field("", "", false)
                            .field(
                                "Difficulty",
                                api_data.tiepe.expect("type is required for greev"),
                                true,
                            )
                            .field(
                                "Generator",
                                api_data.generator.expect("generator is required for greev"),
                                true,
                            )
                            .field("", "", false)
                            .field(
                                "Correct Flags",
                                api_data
                                    .correct_flags
                                    .expect("correct_flags is required for greev"),
                                true,
                            )
                            .field(
                                "Incorrect Flags",
                                api_data
                                    .incorrect_flags
                                    .expect("incorrect_flags is required for greev"),
                                true,
                            )
                            .field("Won", if api_data.won { "Yes" } else { "No" }, false)
                            .color(if api_data.won {
                                Color::from_rgb(102, 187, 106)
                            } else {
                                Color::from_rgb(255, 138, 101)
                            })
                    });

                    if let Some(data) = image_data_result.unwrap() {
                        return msg.add_file(Bytes {
                            data: Cow::from(data),
                            filename: "game".to_string() + if gif { ".gif" } else { ".webp" },
                        });
                    }

                    msg
                })
                .await
        }
        "mcplayhd" => {
            command
                .create_followup_message(&ctx.http, |message| {
                    let msg = message.embed(|e| {
                        e.title(format!("Minesweeper Game {}", game_id))
                            .field("Username", player_data.name, true)
                            .field(
                                "Time",
                                format!(
                                    "{:02}:{:02}:{:02}.{:03}",
                                    timestamp.hour(),
                                    timestamp.minute(),
                                    timestamp.second(),
                                    timestamp.nanosecond() / 1_000_000
                                ),
                                true,
                            )
                            .field("", "", false)
                            .field(
                                "Correct Flags",
                                api_data
                                    .correct_flags
                                    .expect("correct_flags is required for mcplayhd"),
                                true,
                            )
                            .field(
                                "Incorrect Flags",
                                api_data
                                    .incorrect_flags
                                    .expect("incorrect_flags is required for mcplayhd"),
                                true,
                            )
                            .field("Won", if api_data.won { "Yes" } else { "No" }, false)
                            .color(if api_data.won {
                                Color::from_rgb(102, 187, 106)
                            } else {
                                Color::from_rgb(255, 138, 101)
                            })
                    });

                    if let Some(data) = image_data_result.unwrap() {
                        return msg.add_file(Bytes {
                            data: Cow::from(data),
                            filename: "game".to_string() + if gif { ".gif" } else { ".webp" },
                        });
                    }

                    msg
                })
                .await
        }
        _ => {
            command
                .create_followup_message(&ctx.http, |message| {
                    let msg = message.embed(|e| {
                        e.title(format!("Minesweeper Game {}", game_id))
                            .field("Username", player_data.name, true)
                            .field(
                                "Time",
                                format!(
                                    "{:02}:{:02}:{:02}.{:03}",
                                    timestamp.hour(),
                                    timestamp.minute(),
                                    timestamp.second(),
                                    timestamp.nanosecond() / 1_000_000
                                ),
                                true,
                            )
                            .field("Won", if api_data.won { "Yes" } else { "No" }, false)
                            .color(if api_data.won {
                                Color::from_rgb(102, 187, 106)
                            } else {
                                Color::from_rgb(255, 138, 101)
                            })
                    });

                    if let Some(data) = image_data_result.unwrap() {
                        return msg.add_file(Bytes {
                            data: Cow::from(data),
                            filename: "game".to_string() + if gif { ".gif" } else { ".webp" },
                        });
                    }

                    msg
                })
                .await
        }
    };

    if let Err(error) = result {
        println!("Was unable to respond to command! {:?}", error)
    }
}

async fn get_image_data(
    api_data: &ApiData,
    mut gif: &bool,
) -> Result<Option<Vec<u8>>, CommandError> {
    if let Some(game_data) = &api_data.game_data {
        let option = game_data.split_once('=').expect("Unable to get Version");

        let possible_parsers: Vec<&dyn Iparser> = vec![
            &parsers::v1::parser::ParserV1,
            &parsers::v2::parser::ParserV2,
        ];

        let option_found_parser = possible_parsers
            .iter()
            .find(|p| p.supported_versions().contains(&option.0));

        if option_found_parser.is_none() {
            return Err(CommandError::UnsupportedVersion);
        }

        let parser = option_found_parser.unwrap();

        let split: Vec<&str> = option.1.split('+').collect();

        let metadata = parser.parse_meta_data(split[0].trim());

        let game_data = ParsedData {
            game_board: parser.parse_mine_data(split[1].trim(), &metadata),
            open_data: parser.parse_open_data(split[2].trim()),
            flag_data: parser.parse_flag_data(split[3].trim()),
            metadata,
        };

        //If the field is too large overwrite the gif value to not render a gif
        if game_data.metadata.x_size > 32 || game_data.metadata.y_size > 32 {
            gif = &false
        }

        let mut renderer = Renderer::new(
            game_data.metadata,
            game_data.game_board,
            game_data.open_data,
            game_data.flag_data,
            gif,
        );

        Ok(Some(if *gif {
            renderer
                .render_gif()
                .map_err(|_| CommandError::ImageRender)?
        } else {
            renderer
                .render_jpeg()
                .map_err(|_| CommandError::ImageRender)?
        }))
    } else {
        Ok(None)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ms")
        .description("Shows details about a Minesweeper Game")
        .create_option(|option| {
            option
                .name("game_id")
                .description("The GameID of the Minesweeper round")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("gif")
                .description("Render the game as a gif (Only up to 32x32 fields)")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("provider")
                .description(format!(
                    "Where the game was played (Default: {DEFAULT_PROVIDER})"
                ))
                .kind(CommandOptionType::String)
                .add_string_choice("Greev", "greev")
                .add_string_choice("McPlayHD", "mcplayhd")
                .required(false)
        })
}

async fn error_response(command: &ApplicationCommandInteraction, ctx: &Context, error_text: &str) {
    command
        .create_followup_message(&ctx.http, |message| {
            message
                .embed(|e| {
                    e.description(error_text)
                        .color(Color::from_rgb(255, 50, 50))
                })
                .flags(MessageFlags::EPHEMERAL)
        })
        .await
        .unwrap();
}

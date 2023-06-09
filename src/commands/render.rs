use std::borrow::Cow;

use chrono::{NaiveDateTime, Timelike};
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::{InteractionResponseType, MessageFlags};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::channel::AttachmentType::Bytes;
use serenity::utils::Color;

use crate::commands::error::CommandError;
use crate::minesweeper;
use crate::minesweeper::fetcher::{ApiData, fetch_name, PlayerData};
use crate::minesweeper::parser::parse_v1;
use crate::minesweeper::renderer::Renderer;

pub(crate) async fn run(command: &ApplicationCommandInteraction, ctx: &Context) {
    let game_id = command.data.options.iter().find(|x| x.name.eq("game_id"));

    if game_id.is_none() || game_id.unwrap().value.as_ref().is_none() {
        error_response(command, ctx, "Please provide a game id").await;
        return;
    }

    let game_id = game_id
        .expect("GameID is empty")
        .value
        .as_ref()
        .expect("Unable to get value")
        .as_u64()
        .expect("Unable to get the GameID as u64");

    let result_api_data = minesweeper::fetcher::fetch_data(game_id);

    if result_api_data.is_err() {
        error_response(command, ctx, "Unable to fetch game data").await;
        return;
    }

    let api_data = result_api_data.unwrap();

    let image_data_result = get_image_data(&api_data).await;

    if let Err(error) = image_data_result {
        error_response(command, ctx, error.to_string().as_str()).await;
        return;
    }

    let timestamp = NaiveDateTime::from_timestamp_millis(api_data.time as i64)
        .expect("Unable to get Timestamp from time");

    let player_data = fetch_name(api_data.uuid.as_str()).unwrap_or_else(|_| PlayerData {
        name: "%".to_string(),
    });

    let result = command
        .create_followup_message(&ctx.http, |message| {
            let msg = message.embed(|e| {
                e.title(format!("Minesweeper Game {}", game_id))
                    .field("Username", player_data.name, true)
                    .field(
                        "Time",
                        format!(
                            "{:02}:{:02}:{:02}.{:2}",
                            timestamp.hour(),
                            timestamp.minute(),
                            timestamp.second(),
                            timestamp.nanosecond() / 1_000_000
                        ),
                        true,
                    )
                    .field("", "", false)
                    .field("Difficulty", api_data.tiepe, true)
                    .field("Generator", api_data.generator, true)
                    .field("", "", false)
                    .field("Correct Flags", api_data.correct_flags, true)
                    .field("Incorrect Flags", api_data.incorrect_flags, true)
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
                    filename: "game.webp".to_string(),
                });
            }

            msg
        })
        .await;

    if let Err(error) = result {
        println!("Was unable to respond to command! {:?}", error)
    }
}

async fn get_image_data(api_data: &ApiData) -> Result<Option<Vec<u8>>, CommandError> {
    if let Some(game_data) = &api_data.game_data {
        let option = game_data.split_once('=').expect("Unable to get Version");

        if !(option.0.eq("1")) {
            return Err(CommandError::UnsupportedVersion);
        }

        let game_data = parse_v1(option.1).map_err(|_| CommandError::DataParse)?;

        let mut renderer = Renderer::new(
            game_data.metadata,
            game_data.game_board,
            game_data.open_data,
            game_data.flag_data,
        );

        Ok(Some(
            renderer
                .render_jpeg()
                .map_err(|_| CommandError::ImageRender)?,
        ))
    } else {
        Ok(None)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ms")
        .description("Show info about a Minesweeper Game played on greev.eu")
        .create_option(|option| {
            option
                .name("game_id")
                .description("The GameID of the Minesweeper round")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .required(true)
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
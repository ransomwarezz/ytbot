mod download;

use std::env;
use log::info;
use pretty_env_logger::{init as log_init};
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::InputFile;
use dotenv::dotenv;
use crate::download::{download_audio, download_video};

#[tokio::main]
async fn main() {
    dotenv().ok();
    log_init();
    info!("Starting command bot...");
    let bot = Bot::from_env();
    env::var("RUSTBOT_DIR").expect("RUSTBOT_DIR not set");
    Command::repl(bot, handler).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "This is the youtube downloader bot. \
Supported commands are listed below:")]
enum Command {
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Download a video from youtube in best quality.")]
    DownloadVideo(String),

    #[command(description = "Download an audio from youtube in best quality.")]
    DownloadAudio(String),

}

async fn handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let dir = env::var("RUSTBOT_DIR").unwrap_or_else(|_| "/tmp".to_string());

    match cmd {
        Command::Start => {
            info!("Starting bot...");
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }

        Command::DownloadVideo(url) => {
            info!("Downloading video from url: {}", url);
            bot.send_message(msg.chat.id, format!("Downloading video from url: {url}")).await?;
            match download_video(dir, &url).await {
                Ok(video) => {
                    info!("Sending video: {:?}", video);
                    bot.send_video(msg.chat.id, InputFile::file(video)).await?;
                }
                Err(e) => {
                    info!("Failed to download video: {:?}", e);
                    bot.send_message(msg.chat.id, format!("Failed to download video: {:?}", e)).await?;
                }
            };
        }
        Command::DownloadAudio(url) => {
            info!("Downloading audio from url: {}", url);
            bot.send_message(msg.chat.id, format!("Downloading audio from url: {url}")).await?;
            match download_audio(dir, &url).await {
                Ok(audio) => {
                    info!("Sending audio: {:?}", audio);
                    bot.send_audio(msg.chat.id, InputFile::file(audio)).await?;
                }
                Err(e) => {
                    info!("Failed to download audio: {:?}", e);
                    bot.send_message(msg.chat.id, format!("Failed to download audio: {:?}", e)).await?;
                }
            };
        }
    };

    Ok(())
}

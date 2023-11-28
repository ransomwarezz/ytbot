use anyhow::Result;
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use std::path::{PathBuf};


pub(crate) async fn download_video(dir: String, url: &String) -> Result<PathBuf> {
    let opts = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::Video,
        ..Default::default()
    };
    let video = Video::new_with_options(url, opts)?;
    let path = format!("{}/{}", dir, video.get_video_id());

    video.download(path.clone()).await?;
    Ok(PathBuf::from(&path))
}

pub(crate) async fn download_audio(dir: String, url: &String) -> Result<PathBuf> {
    let opts = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };
    let video = Video::new_with_options(url, opts)?;
    let path = format!("{}/{}", dir, video.get_video_id());

    video.download(path.clone()).await?;
    Ok(PathBuf::from(&path))
}
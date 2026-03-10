use std::path::PathBuf;

use yt_dlp::{Downloader, client::Libraries};
pub struct YoutubeVideoInfo{
    pub id: String,
    pub title: String,
    pub duration: f64
}
pub async fn get_youtube_url_info(url: &str) -> Result<Vec<YoutubeVideoInfo>, Box<dyn std::error::Error>> {
    let libraries_dir: PathBuf = PathBuf::from("libs");
    let output_dir = PathBuf::from("temp");

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let downloader = Downloader::builder(libraries, output_dir).build().await?;
    let mut result = Vec::new();
    if url.contains("playlist") | url.contains("list"){
        let play_list = downloader.fetch_playlist_infos(url).await;
        if play_list.is_err() {
            return fallback_playlist_info(url).await;
        }
        let play_list = play_list.unwrap();
        println!("Playlist Title: {}", play_list.title);
        for video in play_list.available_entries() {
            println!("Video Title: {}", video.title);
            println!("Duration: {} seconds", video.duration.unwrap_or(0.0));
            result.push(YoutubeVideoInfo {
                id: video.clone().id.clone(),
                title: video.clone().title,
                duration: video.duration.unwrap_or(0.0)
            });
        }
    }
    else{
        let video = downloader.fetch_video_infos(url).await?;
        println!("Title: {}", video.title);
        println!("Duration: {} seconds", video.duration.unwrap_or(0));
        result.push(YoutubeVideoInfo {
            id: video.id,
            title: video.title,
            duration: video.duration.unwrap_or(0) as f64
        });
    }
    Ok(result)
}
pub async fn audio() -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("temp");

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let downloader = Downloader::builder(libraries, output_dir).build().await?;

    let url = String::from("https://www.youtube.com/watch?v=gXtp6C-3JKo");
    let video = downloader.fetch_video_infos(url).await?;
    downloader
        .download_audio_stream(&video, "audio.mp3")
        .await?;
    Ok(())
}
async fn fallback_playlist_info(url: &str) -> Result<Vec<YoutubeVideoInfo>, Box<dyn std::error::Error>> {
    print!("yt-dlp failed to fetch playlist info, falling back to manual parsing...");
    let mut result = Vec::<YoutubeVideoInfo>::new();
    let response = reqwest::get(url).await?;
    let html_raw = response.text().await?;
    let mut json = html_raw.split("var ytInitialData =").nth(1).and_then(|s| s.split("</script>").next()).unwrap_or("").trim_end();
    json = json.strip_suffix(';').unwrap_or(json);
    let json: serde_json::Value = serde_json::from_str(json)?;
    println!("Successfully parsed JSON data from YouTube page. {}", json.pointer("/contents/twoColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/0/itemSectionRenderer/contents/0/playlistVideoListRenderer/contents").is_some());
    let videos = json.pointer("/contents/twoColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/0/itemSectionRenderer/contents/0/playlistVideoListRenderer/contents").and_then(|v| v.as_array()).unwrap();
    for video_js in videos {
        let video = video_js.get("playlistVideoRenderer").unwrap_or(video_js);
        let id = video.get("videoId").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let title = video["title"]["runs"][0]["text"].as_str().unwrap_or("").to_string();
        let duration = video["lengthSeconds"].as_str().and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
        result.push(YoutubeVideoInfo { id, title, duration });
    }
    Ok(result)
}
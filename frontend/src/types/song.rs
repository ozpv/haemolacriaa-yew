use crate::types::links::StreamingInfo;
use crate::config::{SongId, STREAMING_PLATFORMS};

#[derive(Clone)]
pub struct ConstSongInfo<'a> {
    pub name: &'a str,
    pub author: &'a str,
    pub id: SongId,
    pub spotify_track_id: &'a str,
    pub youtube_video_id: &'a str,
    pub soundcloud_song_id: &'a str,
    pub apple_music_song_id: &'a str,
}

impl ConstSongInfo<'_> {
    pub fn build_songinfo(&self) -> SongInfo {
        SongInfo {
            name: self.name.to_owned(),
            author: self.author.to_owned(),
            id: self.id.clone(),
            spotify_track_id: self.spotify_track_id.to_owned(),
            youtube_video_id: self.youtube_video_id.to_owned(),
            soundcloud_song_id: self.soundcloud_song_id.to_owned(),
            apple_music_song_id: self.apple_music_song_id.to_owned(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct SongInfo {
    name: String,
    author: String,
    id: SongId,
    spotify_track_id: String,
    youtube_video_id: String,
    soundcloud_song_id: String,
    apple_music_song_id: String,
}

impl SongInfo {
    pub fn build_streaminginfo(&self) -> Vec<StreamingInfo> {
        STREAMING_PLATFORMS.into_iter().map(|platform| StreamingInfo {
            platform_ico: platform.ico,
            platform_name: platform.name.to_owned(),
            song_name: self.name.clone(),
            song_url: {
                match platform.name as &str {
                    "Spotify" => { format!("{}{}", platform.base_song_url.to_owned(), self.spotify_track_id.clone()) },
                    "YouTube" => { format!("{}{}", platform.base_song_url.to_owned(), self.youtube_video_id.clone()) },
                    "SoundCloud" => { format!("{}{}", platform.base_song_url.to_owned(), self.soundcloud_song_id.clone()) },
                    "Apple" => { format!("{}{}", platform.base_song_url.to_owned(), self.apple_music_song_id.clone()) },
                    _ => { "/".to_owned() },
                }
            },
        }).collect::<Vec<StreamingInfo>>()
    }
} 

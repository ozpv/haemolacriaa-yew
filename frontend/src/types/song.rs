use crate::config::{SongId, STREAMING_PLATFORMS};
use crate::types::{
    images::{ConstImage, Image},
    links::StreamingInfo,
};

#[derive(Clone)]
pub struct ConstSongInfo<'a> {
    pub name: &'a str,
    pub author: &'a str,
    pub image: ConstImage<'a>,
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
            image: self.image.build_image().clone(),
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
    pub name: String,
    pub author: String,
    pub image: Image,
    pub id: SongId,
    pub spotify_track_id: String,
    pub youtube_video_id: String,
    pub soundcloud_song_id: String,
    pub apple_music_song_id: String,
}

impl SongInfo {
    pub fn build_streaminginfo(&self) -> Vec<StreamingInfo> {
        STREAMING_PLATFORMS
            .into_iter()
            .map(|platform| StreamingInfo {
                platform_ico: platform.ico,
                platform_name: platform.name.to_owned(),
                song_name: self.name.clone(),
                song_url: {
                    match platform.name as &str {
                        "Spotify" => {
                            format!(
                                "{}{}",
                                platform.base_song_url.to_owned(),
                                self.spotify_track_id
                            )
                        }
                        "YouTube" => {
                            format!(
                                "{}{}",
                                platform.base_song_url.to_owned(),
                                self.youtube_video_id
                            )
                        }
                        "SoundCloud" => {
                            format!(
                                "{}{}",
                                platform.base_song_url.to_owned(),
                                self.soundcloud_song_id
                            )
                        }
                        "Apple" => {
                            format!(
                                "{}{}",
                                platform.base_song_url.to_owned(),
                                self.apple_music_song_id
                            )
                        }
                        _ => "/".to_owned(),
                    }
                },
            })
            .collect::<Vec<StreamingInfo>>()
    }
}

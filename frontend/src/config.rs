use const_format::formatcp;
use yew_icons::IconId;

use crate::{types::*, Route};

/* info section */

/* assumes you have only one, synchronized name */
const USERNAME: &str = "haemolacriaa";

/* derived from the final id on each profile URL, make sure to
 * ignore tracking info; this is usually presented after the id
 * with a '?'.
 */
const SPOTIFY_ARTIST_ID: &str = "7xu9uzSauNTYNgHFSAmmi6";

/* two url parameters: set the apple music region and id... */
const APPLE_MUSIC_REGION: &str = "us";
const APPLE_MUSIC_ID: &str = "1549699645";

/* it may also be your channel's '@' tag... */
const YOUTUBE_CHANNEL_ID: &str = "UCQDQqA9iaWtlNkwXiCQogYQ";

/* MAKE SURE TO SORT EACH ARRAY INTO THE
 * SEQUENCE YOU WOULD LIKE THEM TO BE PRINTED
 * ONTO THE PAGE.
 */

/* navbar section */

pub const NAV_ITEMS: [nav::NavItem; 2] = [
    nav::NavItem {
        route: Route::Home,
        text: "Home",
    },
    nav::NavItem { 
        route: Route::Blog, 
        text: "Blog",
    },
    //nav::NavItem {route: Route::Donate, text: "Donate",},
];

/* link list section */

#[derive(PartialEq, Clone)]
pub enum SongId {
    Serene,
    Still,
    Stay,
    HorrorVacui,
    ThusFar,
    Eurydice,
}

const EURYDICE_COVER: images::ConstImage = images::ConstImage {
    filename: "img/eurydice.webp",
    width: 400,
    height: 400,
};

pub const EURYDICE_SONG: song::ConstSongInfo = song::ConstSongInfo {
    name: "eurydice",
    author: USERNAME,
    image: EURYDICE_COVER,
    id: SongId::Eurydice,
    spotify_track_id: "3jVgwiRUrfanloK2E1peWf",
    youtube_video_id: "_qF4fSIdNqs",
    soundcloud_song_id: "eurydice",
    apple_music_song_id: "1707755091",
};

pub const STREAMING_PLATFORMS: [links::StreamingPlatform; 4] = [
    links::StreamingPlatform {
        ico: IconId::SimpleIconsSpotify,
        name: "Spotify",
        base_song_url: "https://open.spotify.com/track/",
    },
    links::StreamingPlatform {
        ico: IconId::SimpleIconsYoutube,
        name: "YouTube",
        base_song_url: "https://www.youtube.com/watch?v=",
    },
    links::StreamingPlatform {
        ico: IconId::SimpleIconsSoundcloud,
        name: "SoundCloud",
        base_song_url: formatcp!("https://soundcloud.com/{}/", USERNAME),
    },
    links::StreamingPlatform {
        ico: IconId::SimpleIconsApple,
        name: "Apple",
        base_song_url: formatcp!("https://music.apple.com/{}/album/", APPLE_MUSIC_REGION),
    },
];

/* footer section */

/* note: for icons, you must also add the icon's name
 * into yew_icons' features inside Cargo.toml.
 */

pub const SOCIALMEDIAINFO_ITEMS: [links::ConstSocialMediaInfo; 5] = [
    links::ConstSocialMediaInfo {
        ico: IconId::SimpleIconsApple,
        url: formatcp!(
            "https://music.apple.com/{}/artist/{}",
            APPLE_MUSIC_REGION,
            APPLE_MUSIC_ID
        ),
    },
    links::ConstSocialMediaInfo {
        ico: IconId::SimpleIconsSoundcloud,
        url: formatcp!("https://soundcloud.com/{}", USERNAME),
    },
    links::ConstSocialMediaInfo {
        ico: IconId::SimpleIconsYoutube,
        url: formatcp!("https://youtube.com/channel/{}", YOUTUBE_CHANNEL_ID),
    },
    links::ConstSocialMediaInfo {
        ico: IconId::SimpleIconsInstagram,
        url: formatcp!("https://instagram.com/{}", USERNAME),
    },
    links::ConstSocialMediaInfo {
        ico: IconId::SimpleIconsSpotify,
        url: formatcp!("https://open.spotify.com/artist/{}", SPOTIFY_ARTIST_ID),
    },
];

use yew_icons::IconId;
use const_format::formatcp;

use crate::Route;
use crate::types::*;

/* info section */

/* assumes you have only one, synchronized name */
const USERNAME: &'static str = "haemolacriaa";

/* derived from the final id on each profile URL, make sure to
 * ignore tracking info; this is usually presented after the id 
 * with a '?'.
 */
const SPOTIFY_ARTIST_ID: &'static str = "7xu9uzSauNTYNgHFSAmmi6";

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

pub const NAV_ITEMS: &'static [&'static nav::NavItem] = &[ 
    &nav::NavItem{route: Route::Home, text: "Home",},
    &nav::NavItem{route: Route::About, text: "About",},
    &nav::NavItem{route: Route::Donate, text: "Donate",},
    //&nav::NavItem{route: Route::Privacy, text: "Privacy",},
];

/* link list section */

pub const STREAMINGINFO_ITEMS: &'static [&'static links::StreamingInfo] = &[
    &links::StreamingInfo{service: links::StreamingService::Spotify, service_name: "Spotify", url: "spotify.com/"}
];

/* footer section */

/* note: for icons, you must also add the icon's name
 * into yew_icons' features inside Cargo.toml.
 */
pub const SOCIALMEDIAINFO_ITEMS: &'static [&'static links::SocialMediaInfo] = &[
    &links::SocialMediaInfo{ico: IconId::SimpleIconsApple, url: formatcp!("https://music.apple.com/{}/artist/{}", APPLE_MUSIC_REGION, APPLE_MUSIC_ID)},
    &links::SocialMediaInfo{ico: IconId::SimpleIconsSoundcloud, url: formatcp!("https://soundcloud.com/{}", USERNAME)},
    &links::SocialMediaInfo{ico: IconId::SimpleIconsYoutube, url: formatcp!("https://youtube.com/channel/{}", YOUTUBE_CHANNEL_ID)},
    &links::SocialMediaInfo{ico: IconId::SimpleIconsInstagram, url: formatcp!("https://instagram.com/{}", USERNAME)},
    &links::SocialMediaInfo{ico: IconId::SimpleIconsSpotify, url: formatcp!("https://open.spotify.com/artist/{}", SPOTIFY_ARTIST_ID)},
];

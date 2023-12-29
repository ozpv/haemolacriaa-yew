use crate::types::*;
use crate::app::Route;

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


pub const SOCIALMEDIAINFO_ITEMS: &'static [&'static links::SocialMediaInfo] = &[
    &links::SocialMediaInfo{service: links::SocialMedia::Instagram, service_name: "Instagram", url: "instagram.com/"},
];

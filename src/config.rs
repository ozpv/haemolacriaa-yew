use crate::types::*;

/* navbar section */

pub const NAV_ITEMS: &'static [&'static nav::NavItem] = &[ 
    &nav::NavItem{text: "Home", href: "/"},
    &nav::NavItem{text: "About", href: "/about"},
    &nav::NavItem{text: "Support", href: "/support"},
    &nav::NavItem{text: "Privacy", href: "/privacy"},
];

/* link list section */

pub const STREAMINGINFO_ITEMS: &'static [&'static links::StreamingInfo] = &[
    &links::StreamingInfo{service: links::StreamingService::Spotify, service_name: "Spotify", url: "spotify.com/"}
];


pub const SOCIALMEDIAINFO_ITEMS: &'static [&'static links::SocialMediaInfo] = &[
    &links::SocialMediaInfo{service: links::SocialMedia::Instagram, service_name: "Instagram", url: "instagram.com/"},
];

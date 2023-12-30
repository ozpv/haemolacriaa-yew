use yew_icons::IconId;

pub enum StreamingService {
    Spotify = 1,
    Soundcloud = 2,
    YouTube = 3,
    YouTubeMusic = 4,
    AppleMusic = 5,
    Bandcamp = 6,
    Amazon = 7,
    ITunes = 8,
}

pub struct StreamingInfo<'a> {
    pub service: StreamingService,
    pub service_name: &'a str,    
    pub url: &'a str,
}

pub struct SocialMediaInfo<'a> {
    pub ico: IconId,
    pub url: &'a str,
}

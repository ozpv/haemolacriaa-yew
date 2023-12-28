/* u8 so max 256 streaming services... */
pub enum StreamingService {
    Spotify,
    Soundcloud,
    YouTube,
    YouTubeMusic,
    AppleMusic,
    Bandcamp,
    Amazon,
    iTunes,
}

pub struct StreamingInfo {
    pub service: u8,
    pub service_name: String,    
    pub url: String,
}

/* u8 so max 256 social media services... */
pub enum SocialMedia {
    Instagram,
}

pub struct SocialMediaInfo {
    pub service: u8,
    pub service_name: String,
    pub url: String,
}

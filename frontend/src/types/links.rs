use yew_icons::IconId;

pub struct StreamingInfo {
    pub song_name: String,
    pub song_url: String,
    pub platform_ico: IconId,
    pub platform_name: String,
}

pub struct StreamingPlatform<'a> {
    pub ico: IconId,
    pub name: &'a str,    
    pub base_song_url: &'a str,
}

pub struct SocialMediaInfo<'a> {
    pub ico: IconId,
    pub url: &'a str,
}

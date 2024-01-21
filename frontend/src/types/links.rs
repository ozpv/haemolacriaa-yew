use yew::Properties;
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

pub struct ConstSocialMediaInfo<'a> {
    pub ico: IconId,
    pub url: &'a str,
}

impl ConstSocialMediaInfo<'_> {
    pub fn build_socialmediainfo(&self) -> SocialMediaInfo {
        SocialMediaInfo {
            ico: self.ico.clone(),
            url: self.url.to_owned(),
        }
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct SocialMediaInfo {
    pub ico: IconId,
    pub url: String,
}

impl Default for SocialMediaInfo {
    fn default() -> Self {
        Self {
            ico: IconId::SimpleIconsApple,
            url: String::new(),
        }
    }
}

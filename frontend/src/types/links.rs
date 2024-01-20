use yew_icons::IconId;

pub struct StreamingInfo<'a> {
    pub ico: IconId,
    pub name: &'a str,    
    pub url: &'a str,
}

pub struct SocialMediaInfo<'a> {
    pub ico: IconId,
    pub url: &'a str,
}

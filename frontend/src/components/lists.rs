use yew::prelude::*;
use yew_icons::Icon;

use crate::types::{
    images::Image,
    song::SongInfo,
    links::SocialMediaInfo,
};
use crate::components::buttons::LinkButton;

#[derive(PartialEq, Properties)]
pub struct AppendixProps {
    socialmedia_info: SocialMediaInfo,
}

#[function_component(Appendix)]
pub fn appendix(props: &AppendixProps) -> Html {
    html! {
        <nav id="social-media-links">
            <a class="flex justify-center hover:text-rose-500 p-[16px]" href={props.socialmedia_info.url.clone()}><Icon icon_id={props.socialmedia_info.ico.clone()} width={"16"} height={"16"} /></a>
        </nav>
    }
}

#[derive(PartialEq, Properties)]
pub struct StreamingLinkListProps {
    pub song_info: SongInfo,
    pub image: Image,
    pub id: String,  
    #[prop_or_default]
    pub appendix: bool,
    #[prop_or_default]
    pub appendix_info: SocialMediaInfo,
}

#[function_component(StreamingLinkList)]
pub fn streaming_link_list(props: &StreamingLinkListProps) -> Html {
    html! {
        <div id={props.id.clone()}>
            <img class="block mx-auto pt-[16px]" src={props.song_info.image.path.clone()} width={props.song_info.image.width.clone()} height={props.song_info.image.height.clone()} alt={props.song_info.name.clone()} />
            <h1 class="block text-center pt-[16px]">{props.song_info.author.clone()}</h1>
            <div class="flex justify-center">
                <nav id="streaming-links">
                    {
                        props.song_info.build_streaminginfo().into_iter().map(|item| {
                            html! {
                                <LinkButton class="text-white p-[20px] w-80" href={item.song_url} id={format!("{}{}", {props.song_info.name.clone()}, "-link-button")}>
                                    <Icon icon_id={item.platform_ico} width={"24"} height={"24"} />
                                    <p class="pl-4">{item.platform_name}</p>
                                </LinkButton>
                            }
                        }).collect::<Html>()
                    }
                    if props.appendix.clone() {
                        <Appendix socialmedia_info={props.appendix_info.clone()} />
                    } 
                </nav>
            </div>
        </div> 
    }
}

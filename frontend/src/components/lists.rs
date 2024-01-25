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
        <div class="flex justify-center">
            <nav id="social-media-links">
                <a class="flex text-white mt-[10px] mb-[10px] p-[8px] rounded-sm transition-all ease-in duration-75 hover:bg-gray-800" href={props.socialmedia_info.url.clone()}><Icon icon_id={props.socialmedia_info.ico.clone()} width={"16"} height={"16"} /></a>
            </nav>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct StreamingLinkListProps {
    pub song_info: SongInfo,
    pub image: Image,
    pub id: String,  
    #[prop_or(false)]
    pub appendix: bool,
    #[prop_or_default]
    pub appendix_info: SocialMediaInfo,
}

#[function_component(StreamingLinkList)]
pub fn streaming_link_list(props: &StreamingLinkListProps) -> Html {
    html! {
        <div id={props.id.clone()}>
            <img class="block mx-auto pt-[16px] shadow-2xl" src={props.song_info.image.path.clone()} width={props.song_info.image.width.clone()} height={props.song_info.image.height.clone()} alt={props.song_info.name.clone()} />
            <h1 class="block text-white text-center text-3xl font-bold pt-[36px]">{props.song_info.name.clone()}</h1>
            <h2 class="block text-white-800 text-center text-lg font-medium pt-[16px] pb-[10px]">{props.song_info.author.clone()}</h2>
            <div class="flex justify-center">
                <nav id="streaming-links">
                    {
                        props.song_info.build_streaminginfo().into_iter().map(|item| {
                            html! {
                                <LinkButton class="text-white pt-[20px] pb-[20px] w-80" href={item.song_url} id={format!("{}{}", {props.song_info.name.clone()}, "-link-button")}>
                                    <Icon icon_id={item.platform_ico} width={"24"} height={"24"} />
                                    <p class="pl-4">{item.platform_name}</p>
                                </LinkButton>
                            }
                        }).collect::<Html>()
                    }
                </nav>
            </div>
            if props.appendix.clone() {
                <Appendix socialmedia_info={props.appendix_info.clone()} />
            }
        </div> 
    }
}

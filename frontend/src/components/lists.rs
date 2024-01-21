use yew::prelude::*;
use yew_icons::Icon;

use crate::types::{
    images::Image,
    song::SongInfo,
};
use crate::components::buttons::LinkButton;

#[derive(PartialEq, Properties)]
pub struct Props {
}

#[derive(PartialEq, Properties)]
pub struct StreamingLinkListProps {
    pub song_info: SongInfo,
    pub image: Image,
    pub id: String,  
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
                                <LinkButton class="flex justify-center text-center mt-[16px] p-[20px] rounded-lg w-80 bg-blue-700 hover:text-rose-500" href={item.song_url} id="link-button">
                                    <Icon icon_id={item.platform_ico} width={"24"} height={"24"} />
                                    <p class="pl-4">{item.platform_name}</p>
                                </LinkButton>
                            }
                        }).collect::<Html>()
                    }
                </nav>
            </div>
        </div> 
    }
}

/*
#[function_component(LinkList)]
pub fn link_list(props: &Props) -> Html {
    match props.list_type.clone() {
        ListType::Streaming => html! { <StreamingLinkList song_info={props.song_info.clone()} id="streaming-link-list" /> },
        _ => html! { <p>{"Error: list not found."}</p> }
    }
}
*/

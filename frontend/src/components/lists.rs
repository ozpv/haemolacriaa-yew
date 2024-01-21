use yew::prelude::*;
use yew_icons::Icon;

use crate::types::song::SongInfo;
use crate::components::buttons::LinkButton;

#[derive(PartialEq, Properties)]
pub struct Props {
}

#[derive(PartialEq, Properties)]
pub struct StreamingLinkListProps {
    pub song_info: SongInfo,
    pub id: String,  
}

#[function_component(StreamingLinkList)]
pub fn streaming_link_list(props: &StreamingLinkListProps) -> Html {
    html! {
        <div class="flex justify-center" id={props.id.clone()}>
            <nav id="streaming-links">
                {
                    {props.song_info.clone()}.build_streaminginfo().into_iter().map(|item| {
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

use yew::prelude::*;

use crate::config::{SOCIALMEDIAINFO_ITEMS, EURYDICE_SONG};
use crate::components::{
    nav::Nav, 
    foot::Foot,
    lists::StreamingLinkList,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Nav />
            <body class="bg-gray-900 min-h-screen">
                <StreamingLinkList song_info={EURYDICE_SONG.build_songinfo()} image={EURYDICE_SONG.image.build_image()} id="streaming-link-list" appendix={true} appendix_info={SOCIALMEDIAINFO_ITEMS[3].build_socialmediainfo()}/>
            </body> 
            <Foot />
        </>
    }
}

use yew::prelude::*;

use crate::config::EURYDICE_SONG;
use crate::components::{
    nav::Nav, 
    foot::Foot,
    lists::StreamingLinkList,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Nav/>
                <body>
                    <StreamingLinkList song_info={EURYDICE_SONG.build_songinfo()} id=""/>
                </body> 
            <Foot/>
        </>
    }
}

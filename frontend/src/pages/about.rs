use yew::prelude::*;

use crate::components::{
    nav::Nav, 
    foot::Foot,
};

#[function_component(About)]
pub fn page_about() -> Html {
    html! {
        <>
            <Nav />
            <body class="bg-gray-900 min-h-screen">
                <p class="text-white">{"Coming soon..."}</p>
            </body> 
            <Foot />
        </>
    }
}

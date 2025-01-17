use yew::prelude::*;

use crate::components::{foot::Foot, nav::Nav};

#[function_component(Privacy)]
pub fn page_privacy() -> Html {
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

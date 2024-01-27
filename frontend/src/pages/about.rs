use yew::prelude::*;

use crate::components::{foot::Foot, nav::Nav};

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

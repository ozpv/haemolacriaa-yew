use yew::prelude::*;

use crate::components::{foot::Foot, nav::Nav};

#[function_component(Donate)]
pub fn page_donate() -> Html {
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

use yew::prelude::*;
use yew_router::prelude;

use crate::components::nav::Nav;

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <>
            <Nav />
            <body class="bg-gray-900 min-h-screen text-white">
                <p>{"blog coming soon..."}</p>
            </body>
        </>
    }
}

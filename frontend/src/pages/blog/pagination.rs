use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;

#[derive(PartialEq, Properties)]
pub struct PageProps {
    pub id: u64,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    html! {
        <>
            <Nav />
            <body class="bg-gray-900 min-h-screen text-white">
                <h1>{format!("Page id: {}", props.id.clone())}</h1>
                <p>{"blog coming soon..."}</p>
            </body>
        </>
    }
}

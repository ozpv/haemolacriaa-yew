use yew::prelude::*;

use crate::app::comp::nav::Nav;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <Nav />
            <p>{"About Page"}</p>
        < />    
    }
}

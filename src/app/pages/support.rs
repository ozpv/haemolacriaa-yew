use yew::prelude::*;

use crate::app::comp::nav::Nav;

#[function_component(Support)]
pub fn support() -> Html {
    html! {
        <>
            <Nav />
            <p>{"Donate here!"}</p>
        < />
    }
} 

use yew::prelude::*;

use crate::app::comp::nav::Nav;

#[function_component(Privacy)]
pub fn privacy() -> Html {
    html! {
        <>
            <Nav />
            <p>{"Privacy Policy"}</p>
        < />    
    }
}

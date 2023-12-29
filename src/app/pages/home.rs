use yew::prelude::*;

use crate::app::comp::nav::Nav;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Nav />
            <p>{"Welcome Home!"}</p>
        < />    
    }
}

use yew::prelude::*;

use crate::app::comp::{
    nav::Nav,
    foot::Foot,
};

pub struct Donate;

impl Component for Donate {
    type Message = ();
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Nav />
                <p>{"Work in progress..."}</p>
                <Foot />
            < />
        }
    }
}

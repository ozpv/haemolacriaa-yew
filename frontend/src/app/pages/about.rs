use yew::prelude::*;

use crate::app::comp::nav::Nav;
use crate::app::comp::foot::Foot;

pub struct About;

impl Component for About {
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

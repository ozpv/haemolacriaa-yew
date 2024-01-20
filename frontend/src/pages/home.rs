use yew::prelude::*;
use yew_icons::Icon;

use crate::components::{
    nav::Nav, 
    foot::Foot,
    home::button::LinkButton,
}; 

use crate::config;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Nav />
            <body> 
                <nav id="streaming-links"> 
                    {
                        config::STREAMINGINFO_ITEMS.into_iter().map(|item| {
                            html! {
                                <LinkButton id="linkbutton">
                                    <a class="flex rounded-r-lg w-80 hover:text-rose-500" href={item.url}>
                                        <Icon icon_id={item.ico} width={"24"} height={"24"} />
                                        <p class="text-center pl-4">{item.name}</p>
                                    </a>
                                </LinkButton>
                            }
                        }).collect::<Html>()  
                    }
                </nav>
            </body> 
            <Foot />
        < />
    }
}

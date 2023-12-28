pub mod comp;
pub mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::pages::home::Home;
use crate::app::pages::support::Support;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/support")]
    Support,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Support => html! { <Support /> },
        Route::NotFound => html! { <p>{"404 Not Found"}</p> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

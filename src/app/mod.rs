pub mod comp;
pub mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::pages::home::Home;
use crate::app::pages::about::About;
use crate::app::pages::donate::Donate;
use crate::app::pages::privacy::Privacy;

#[derive(Routable, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/donate")]
    Donate,
    #[at("/privacy")]
    Privacy,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Donate => html! { <Donate /> },
        Route::Privacy => html! { <Privacy /> },
        Route::NotFound => html! { <p>{"404 Not Found"}</p> },
        _ => todo!(),
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

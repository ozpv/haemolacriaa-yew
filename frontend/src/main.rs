mod components;
mod config;
mod pages;
mod types;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::{about::About, donate::Donate, home::Home, privacy::Privacy, blog::blog::Blog, blog::pagination::Page,};

#[derive(Routable, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    Page { id: u64 },
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

pub enum Msg {
    OnUpdate,
}

struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnUpdate => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <Blog /> },
        Route::Page { id } => html! { <Page id={id} /> },
        Route::About => html! { <About /> },
        Route::Donate => html! { <Donate /> },
        Route::Privacy => html! { <Privacy /> },
        Route::NotFound => html! { <p>{"404 Not Found"}</p> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{PageNotFound, Home, Chat};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/chat")]
    Chat,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Chat => {
            html! { <Chat /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

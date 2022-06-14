pub mod contact_manager;
pub mod page_not_found;

use contact_manager::*;
use page_not_found::*;
use std::num::NonZeroU64;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/:user_id")]
    ContactManagerView { user_id: NonZeroU64 },

    #[at("/")]
    ContactManagerLanding,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::ContactManagerView { user_id } => {
            html! { <ContactManager user_id={*user_id} /> }
        },
        AppRoute::ContactManagerLanding => {
            html! { <ContactManager /> }
        },
        AppRoute::NotFound => {
            html! { <PageNotFound /> }
        },
    }
}

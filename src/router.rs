use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, Projects, About, NotFound, Forked, Images};

#[derive(Routable, Clone, PartialEq, Debug, Copy)]
pub enum AppRoutes {
  #[at("/projects")]
  Projects,
  #[at("/about")]
  About,
  #[at("/images")]
  Images,
  #[at("/forkedytg")]
  Forked,
  #[not_found]
  #[at("/404")]
  NotFound,
  #[at("/")]
  Home,
}

pub fn switch(routes: &AppRoutes) -> Html {
  match routes {
    AppRoutes::Home => html! { <Home /> },
    AppRoutes::Projects => html! { <Projects /> },
    AppRoutes::About => html! { <About /> },
    AppRoutes::Images => html! { <Images /> },
    AppRoutes::Forked => html! { <Forked /> },
    AppRoutes::NotFound => html! { <NotFound /> },
  }
}
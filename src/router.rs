use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum AppRoutes {
  #[at("/projects")]
  Projects,
  #[at("/about")]
  About,
  #[not_found]
  #[at("/404")]
  NotFound,
  #[at("/")]
  Home,
}

pub fn switch(routes: &AppRoutes) -> Html {
  match routes {
    AppRoutes::Home => {
      html! { <div>{"Home"}</div> }
    },
    AppRoutes::Projects => {
      html! { <div>{"Projects"}</div> }
    },
    AppRoutes::About => {
      html! { <div>{"About"}</div> }
    },
    AppRoutes::NotFound => {
      html! { <div>{404}</div> }
    }
  }
}
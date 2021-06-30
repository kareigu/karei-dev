use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::AppRoutes;

pub struct HomeButton {}

impl Component for HomeButton {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let classes = classes!(
      "text-base-lt", "hover:text-base-dk", 
      "hover:bg-base-lt", "transition-colors",
      "rounded-md", "px-5", "py-1", "mr-auto",
      "ml-1", "font-mulish"
    );
    html! {
      <Link<AppRoutes> 
        route=AppRoutes::Home
        classes=classes
      >
        { "mxrr.dev" }
      </Link<AppRoutes>>
    }
  }
}
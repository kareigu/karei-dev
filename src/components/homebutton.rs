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
    html! {
      <Link<AppRoutes> 
        route=AppRoutes::Home
        classes=classes!("flex flex-row text-base-lt hover:text-base-dk hover:bg-base-lt transition-colors rounded-md px-3 py-1 mr-auto ml-0.5 font-mulish".to_string())
      >
        <span class="w-6 h-6 mr-1.5 rounded-full bg-logo bg-contain" />
        { "mxrr.dev" }
      </Link<AppRoutes>>
    }
  }
}
use yew::prelude::*;
use yew_router::prelude::*;

mod router;
use router::{AppRoutes, switch};

mod components;
use components::NavButton;

struct Model {}

impl Component for Model {
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
      <div class="flex flex-col justify-center">
        <div class="flex justify-evenly bg-red-800 text-white">
          <NavButton to=AppRoutes::Home text="Home".to_string() />
          <NavButton to=AppRoutes::Projects text="Projects".to_string() />
          <NavButton to=AppRoutes::About text="About".to_string() />
        </div>
        <div class="flex justify-center">
        <Router<AppRoutes> render= Router::render(switch)/>
        </div>
      </div>        
    }
  }
}

fn main() {
  yew::start_app::<Model>();
}
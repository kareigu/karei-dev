use yew::prelude::*;
use yew_router::prelude::*;

mod router;
use router::{AppRoutes, switch};

mod components;
use components::{NavButton, HomeButton};

mod pages;

mod utils;
use utils::{get_current_page, update_menu_bar};



pub enum Msg {
  UpdateNavbar,
}

pub struct App {
  active_route: AppRoutes,
  link: ComponentLink<App>,
  test_string: String,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let active_route = get_current_page();

    Self {
      active_route,
      link,
      test_string: format!("{:?}", active_route),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::UpdateNavbar => update_menu_bar(self)
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div 
        class="flex flex-col justify-center"
        onclick=self.link.callback(|_| Msg::UpdateNavbar)
      >
        <div class="flex justify-evenly bg-primary-accent-md">
          <HomeButton 
            active=self.active_route.clone()
          />
          <NavButton 
            to=AppRoutes::Projects 
            text="Projects".to_string() 
            active=self.active_route.clone()
            styles="rounded-bl-md"
          />
          <NavButton 
            to=AppRoutes::About
            text="About".to_string() 
            active=self.active_route.clone()
          />
        </div>
        <div class="flex justify-center mt-3">
          <Router<AppRoutes> render= Router::render(switch)/>
        </div>
      </div>        
    }
  }
}

fn main() {
  yew::start_app::<App>();
}
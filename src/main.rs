use yew::prelude::*;
use yew_router::prelude::*;

mod router;
use router::{AppRoutes, switch};

mod components;
use components::{NavButton, HomeButton};

mod utils;
use utils::get_current_page;


pub enum Msg {
  UpdateNavbar,
}

struct App {
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
      Msg::UpdateNavbar => {
        let new = get_current_page();
        if new == self.active_route {
          false
        } else {
          self.active_route = new;
          true
        }
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div class="flex flex-col justify-center">
        <div 
          class="flex justify-evenly bg-primary-accent-md"
          onclick=self.link.callback(|_| Msg::UpdateNavbar)
        >
          <HomeButton 
            active=self.active_route.clone()
          />
          <NavButton 
            to=AppRoutes::Projects 
            text="Projects".to_string() 
            active=self.active_route.clone()
            b_rounding="rounded-bl-md"
          />
          <NavButton 
            to=AppRoutes::About
            text="About".to_string() 
            active=self.active_route.clone()
            b_rounding=""
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
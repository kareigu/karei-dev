use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::current_route;

mod router;
use router::{AppRoutes, switch};

mod components;
use components::{NavButton, HomeButton};

struct Model {
  active_route: AppRoutes,
}

impl Component for Model {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    let active_route = match current_route::<AppRoutes>() {
      Some(route) => route,
      None => AppRoutes::NotFound
    };
    Self {
      active_route,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    let active_route = match AppRoutes::current_route() {
      Some(route) => route,
      None => AppRoutes::NotFound
    };

    if self.active_route == active_route {
      false
    } else {
      self.active_route = active_route;
      true
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false    
  }

  fn view(&self) -> Html {
    html! {
      <div class="flex flex-col justify-center">
        <div class="flex justify-evenly bg-primary-accent-md">
          <HomeButton />
          <NavButton 
            to=AppRoutes::Projects 
            text="Projects".to_string() 
            active=self.active_route.clone() 
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
        {format!("{:?}", self.active_route)}
      </div>        
    }
  }
}

fn main() {
  yew::start_app::<Model>();
}
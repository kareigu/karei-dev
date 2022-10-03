use yew::prelude::*;
use yew_router::prelude::*;

mod router;
use router::{switch, AppRoutes};

mod components;
use components::{HomeButton, NavButton};

mod pages;

mod utils;
use utils::{get_current_page, update_menu_bar};

pub enum Msg {
  UpdateNavbar,
}

pub struct App {
  active_route: AppRoutes,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let active_route = get_current_page();

    Self { active_route }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::UpdateNavbar => update_menu_bar(self),
    }
  }

  //bg-gradient-to-tr from-primary-accent-md to-[#CF0246]

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <BrowserRouter>
      <div
        class="flex flex-col justify-center"
        onclick={ctx.link().callback(|_| Msg::UpdateNavbar)}
      >
        <div class="primary-zigzag filter drop-shadow-md">
          <div class="flex justify-evenly bg-primary-middle-blend">
            <HomeButton
              active={self.active_route.clone()}
            />
            <span id="projects-btn" class="flex flex-col">
            <NavButton
              to={AppRoutes::Projects}
              text="Projects"
              active={self.active_route.clone()}
              styles="rounded-bl-md"
            />
            </span>
            <span id="about-btn" class="flex flex-col">
            <NavButton
              to={AppRoutes::About}
              text="About"
              active={self.active_route.clone()}
            />
            </span>
          </div>
        </div>
        <div class="flex justify-center mt-3 animate-blur-in">
          <Switch<AppRoutes> render={switch} />
        </div>
      </div>
      </BrowserRouter>
    }
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}

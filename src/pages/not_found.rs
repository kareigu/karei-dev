use crate::components::NavButton;
use crate::router::AppRoutes;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct NotFound {}

impl Component for NotFound {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, _cxt: &Context<Self>) -> Html {
    html! {
      <div class="flex flex-col justify-center items-center animation-">
        <h1 class="text-3xl my-3">{"404"}</h1>
        <img data-trunk="true" src="/static/PepegaSit.png" alt="404 - pepega" class="my-3" />
        <NavButton
          to={ AppRoutes::Home }
          text="Go back home"
          active={ AppRoutes::NotFound }
          styles="rounded-b-md bg-base-md mt-6"
        />
      </div>
    }
  }
}

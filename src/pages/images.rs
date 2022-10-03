use crate::components::NavButton;
use crate::router::AppRoutes;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct Images {
  props: Props,
}

impl Component for Images {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self { props }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
  }

  fn view(&self) -> Html {
    html! {
      <div class="flex flex-col justify-center items-center animation-">
        <h1 class="text-3xl my-3">{"Images"}</h1>
        <img data-trunk="true" src="/static/PepegaSit.png" alt="404 - pepega" class="my-3" />
        <NavButton
          to=AppRoutes::Home
          text="Go back home"
          active=AppRoutes::NotFound
          styles="rounded-b-md bg-base-md mt-6"
        />
      </div>
    }
  }
}

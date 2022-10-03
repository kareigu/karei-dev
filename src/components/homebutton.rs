use crate::router::AppRoutes;
use yew::prelude::*;
use yew_router::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub active: AppRoutes,
}

pub struct HomeButton {
  props: Props,
}

impl Component for HomeButton {
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
    let mut classlist = classes!("flex flex-row text-base-lt hover:text-base-dk hover:bg-base-lt transition-colors rounded-r-md px-3 py-1 mr-auto ml-0 font-mulish".to_string());

    if self.props.active == AppRoutes::Home {
      classlist.push("bg-primary-accent-dk bg-opacity-50 border-secondary-accent-md border-r-4");
    } else {
      classlist.push("bg-primary-accent-dk bg-opacity-30 border-base-lt border-r-4");
    }

    html! {
      <Link<AppRoutes>
        route=AppRoutes::Home
        classes=classlist
      >
        <span class="w-6 h-6 mr-1.5 rounded-full bg-logo bg-contain" />
        { "karei.dev" }
      </Link<AppRoutes>>
    }
  }
}

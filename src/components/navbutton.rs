use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::AppRoutes;
use yewtil::NeqAssign;
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub to: AppRoutes,
  pub text: String,
  pub active: AppRoutes,
  pub b_rounding: String,
}

pub struct NavButton {
  props: Props,
}

impl Component for NavButton {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      props
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
  }

  fn view(&self) -> Html {
    let mut classlist = classes!("text-base-lt hover:text-base-dk hover:bg-base-lt transition-colors rounded-t-md px-5 py-1".to_string());

    classlist.push(self.props.b_rounding.clone());

    if self.props.active == self.props.to {
      classlist.push("bg-primary-accent-dk border-b-4 border-tertiary-accent-md");
    }  else {
      classlist.push("border-b-4 border-base-lt");
    }

    html! {
      <Link<AppRoutes> 
        route=self.props.to.clone()
        classes=classlist
      >
        { self.props.text.clone() }
      </Link<AppRoutes>>
    }
  }
}
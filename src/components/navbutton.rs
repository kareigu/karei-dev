use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::AppRoutes;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub to: AppRoutes,
  pub text: String,
  pub active: AppRoutes,
}

pub struct NavButton {
  to: AppRoutes,
  text: String,
  active: AppRoutes,
}

impl Component for NavButton {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      to: props.to,
      text: props.text,
      active: props.active,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if self.to == props.to && self.text == props.text && self.active == props.active {
      false
    } else {
      self.to = props.to;
      self.text = props.text;
      self.active = props.active;
      true
    }
  }

  fn view(&self) -> Html {
    let mut classlist = classes!("text-base-lt hover:text-base-dk hover:bg-base-lt transition-colors rounded-md px-5 py-1".to_string());

    if self.active == self.to {
      classlist.push("bg-tertiary-accent-md");
    }

    html! {
      <Link<AppRoutes> 
        route=self.to.clone()
        classes=classlist
      >
        { self.text.clone() }
      </Link<AppRoutes>>
    }
  }
}
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::AppRoutes;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub to: AppRoutes,
  pub text: String,
}

pub struct NavButton {
  to: AppRoutes,
  text: String,
}

impl Component for NavButton {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      to: props.to,
      text: props.text,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    unimplemented!()
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if self.to == props.to && self.text == props.text {
      false
    } else {
      self.to = props.to;
      self.text = props.text;
      true
    }
  }

  fn view(&self) -> Html {
    html! {
      <Link<AppRoutes> 
        route=self.to.clone()
        classes=classes!("text-base-lt hover:text-base-dk hover:bg-base-lt transition-colors rounded-md px-5 py-1".to_string())
      >
        { self.text.clone() }
      </Link<AppRoutes>>
    }
  }
}
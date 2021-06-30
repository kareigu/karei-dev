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
    let classes = classes!(
      "text-white", "hover:text-black", 
      "hover:bg-white", "transition-colors",
      "rounded-md", "px-5", "py-1"
    );
    html! {
      <Link<AppRoutes> 
        route=self.to.clone()
        classes=classes
      >
        { self.text.clone() }
      </Link<AppRoutes>>
    }
  }
}
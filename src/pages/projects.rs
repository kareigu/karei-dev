use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Projects {
  props: Props,
}

impl Component for Projects {
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
    html! {
      <div class="flex flex-col justify-center items-center">
        {"Projects"}
      </div>
    }
  }
}
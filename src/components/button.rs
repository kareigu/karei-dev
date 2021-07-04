use yew::prelude::*;
use yewtil::NeqAssign;


#[derive(Clone, PartialEq)]
pub enum Colour {
  Primary,
  Secondary,
  Tertiary,
  Custom(String),
}


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub text: String,
  pub colour: Colour,
  pub styles: Option<String>,
  pub icon: Option<String>,
}

pub struct Button {
  props: Props,
}

impl Component for Button {
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
    let colour = match self.props.colour.clone() {
      Colour::Primary => "bg-primary-accent-md".to_string(),
      Colour::Secondary => "bg-secondary-accent-md".to_string(),
      Colour::Tertiary => "bg-tertiary-accent-md".to_string(),
      Colour::Custom(s) => s,
    };

    let mut classlist = classes!("text-base-lt hover:text-base-dk hover:bg-base-lt transition-colors rounded-md px-5 py-1 m-2".to_string());

    classlist.push(colour);
    classlist.push(self.props.styles.clone());

    let icon = if let Some(url) = &self.props.icon {
      html! { <img src={url.clone()} alt="button icon" width="28px" height="28px" class="mr-2" />}
    } else {
      html!{}
    };

    html! {
      <button class={classlist}>
        <span class="flex flex-row justify-center items-center">
          { icon }
          { self.props.text.clone() }
        </span>
      </button>
    }
  }
}
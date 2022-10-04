use yew::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
pub enum Colour {
  Primary,
  Secondary,
  Tertiary,
  Custom(String),
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
  pub text: String,
  pub colour: Colour,
  pub styles: Option<String>,
  pub icon: Option<String>,
}

pub struct Button {}

impl Component for Button {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let colour = match ctx.props().colour.clone() {
      Colour::Primary => "bg-primary-accent-md".to_string(),
      Colour::Secondary => "bg-secondary-accent-md".to_string(),
      Colour::Tertiary => "bg-tertiary-accent-md".to_string(),
      Colour::Custom(s) => s,
    };

    let mut classlist = classes!(
      "text-base-lt hover:text-base-dk hover:bg-base-lt transition-colors rounded-md px-5 py-1 m-2"
        .to_string()
    );

    classlist.push(colour);
    classlist.push(ctx.props().styles.clone());

    let icon = if let Some(url) = ctx.props().icon.clone() {
      html! { <img src={ url.clone() } alt="button icon" width="28px" height="28px" class="mr-2" />}
    } else {
      html! {}
    };

    html! {
      <button class={classlist}>
        <span class="flex flex-row justify-center items-center">
          { icon }
          { ctx.props().text.clone() }
        </span>
      </button>
    }
  }
}

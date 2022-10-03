use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub icon: String,
  pub disable_invert: Option<bool>,
}

pub struct Icon {}

impl Component for Icon {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let icon = match ctx.props().icon.as_str() {
      "gh" => "/static/GitHub-Mark-32px.png",
      "image" => "/static/image.png",
      "compass" => "/static/compass.png",
      "linked-in" => "/static/linked-in.png",
      "steam" => "/static/steam.png",
      "twitter" => "/static/twitter.png",
      _ => "",
    };

    let class = if let Some(b) = ctx.props().disable_invert {
      if !b {
        classes!("filter invert m-2".to_string())
      } else {
        classes!("m-2")
      }
    } else {
      classes!("m-2")
    };

    html! {
      <img class={ class } src={ icon } alt={ ctx.props().icon.clone() } width="24px" height="24px" />
    }
  }
}

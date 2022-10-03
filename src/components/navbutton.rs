use crate::router::AppRoutes;
use yew::prelude::*;
use yew_router::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub to: AppRoutes,
  pub text: String,
  pub active: AppRoutes,
  pub styles: Option<String>,
}

pub struct NavButton {}

impl Component for NavButton {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let mut classlist = classes!(
      "text-base-lt hover:text-base-dk hover:bg-base-lt rounded-l-md transition-all px-5 py-1"
        .to_string()
    );

    classlist.push(ctx.props().styles.clone());

    if ctx.props().active == ctx.props().to {
      classlist.push("bg-primary-accent-dk bg-opacity-50 border-l-4 border-secondary-accent-md");
    } else {
      classlist.push("bg-primary-accent-dk bg-opacity-30 border-l-4 border-base-lt");
    }

    html! {
      <Link<AppRoutes>
        to={ctx.props().to}
        classes={classlist}
      >
        { ctx.props().text.clone() }
      </Link<AppRoutes>>
    }
  }
}

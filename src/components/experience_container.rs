#![allow(clippy::derive_partial_eq_without_eq)]
use yew::{html::ChildrenRenderer, prelude::*};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub img: String,
  pub title: String,
  pub location: String,
  pub company: String,
  pub timespan: String,
  pub extra_info: Option<String>,
  pub children: ChildrenRenderer<Html>,
}

pub struct ExperienceContainer {}

impl Component for ExperienceContainer {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let title_styles = if ctx.props().title.chars().count() > 28 {
      classes!("font-mulish text-l md:text-xl".to_string())
    } else if ctx.props().title.chars().count() > 22 {
      classes!("font-mulish text-2xl".to_string())
    } else {
      classes!("font-mulish text-3xl".to_string())
    };

    let company_styles = if ctx.props().company.chars().count() > 34 {
      classes!("text-xs", "md:text-base")
    } else {
      classes!("text-base")
    };

    let extra_info = match &ctx.props().extra_info {
      None => html! {},
      Some(info) => {
        html! { <p class="py-1 px-2 my-2 text-center rounded-md bg-black bg-opacity-30">{ info.clone() }</p>}
      }
    };

    html! {
      <div class="flex flex-col justify-center animate-slide-up desktop:animate-blur-in bg-black bg-opacity-30 rounded-md w-[calc(100vw-0.75rem)] md:w-[36rem] my-6 mx-2 md:mx-8">
        <div class="flex flex-row justify-center items-center primary-accent-wavy rounded-t-md">
          <img class="ml-1 mr-auto rounded-full bg-base-lt border-2 border-tertiary-accent-lt" src={ ctx.props().img.clone() } alt="company logo" width="48px" height="48px" />
          <div class="flex flex-col justify-center items-center mx-auto">
            <h1 class={ title_styles }>{ ctx.props().title.clone() }</h1>
            <h2 class={ company_styles }>{ ctx.props().company.clone() }</h2>
            <h2>{ format!("📍{}", ctx.props().location) }</h2>
            { extra_info }
          </div>
          <div class="mr-1 ml-auto md:w-[48px] md:h-[48px]" />
        </div>

        <div class="bg-white bg-opacity-5 rounded-sm">
          <ul>
            { ctx.props().children.clone() }
          </ul>
        </div>
        <div class="flex justify-center items-center py-2 bg-tertiary-accent-dk bg-opacity-70 tertiary-zigzag rounded-b-md">
          <h2 class="bg-black bg-opacity-50 px-1 rounded-md">{ format!("⏳{}", ctx.props().timespan) }</h2>
        </div>
      </div>
    }
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct LProps {
  pub text: String,
}

pub struct ListItem {}

impl Component for ListItem {
  type Message = ();
  type Properties = LProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <li class="odd:bg-white odd:bg-opacity-10 px-4 py-1 last:pb-4">{ ctx.props().text.clone() }</li>
    }
  }
}

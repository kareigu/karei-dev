use crate::components::Icon;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub project: Project,
}

pub struct ProjectBlock {}

impl Component for ProjectBlock {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="flex flex-col animate-slide-up desktop:animate-blur-in bg-black bg-opacity-30 w-11/12 md:w-[38rem] h-auto md:h-[34rem] my-6 mx-16 md:mx-8 rounded-t-md rounded-b-lg">
        <div class="flex justify-center rounded-t-md circle-bg bg-opacity-70 bg-secondary-accent-lt">
          <h1 class="text-2xl md:text-3xl font-mulish select-none">{ ctx.props().project.name.clone() }</h1>
        </div>
        <img class="max-h-[405px]" src={format!("/files/{}", ctx.props().project.img)} alt={format!("{} banner", ctx.props().project.name)} />
        <p class="overflow-hidden px-5 py-3 md:text-base text-center text-xs self-center">{ ctx.props().project.description.clone() }</p>

        <div class="flex justify-end mt-auto primary-accent-wavy rounded-b-lg">
          {
            ctx.props().project.links.other.iter()
              .map(|l| {
                html! {
                  <a class="animate-pulse filter invert hover:invert-0 transition-all" href={l.link.clone()} target="_blank">
                    <Icon icon={l.icon.clone()} />
                  </a>}
              })
              .collect::<Html>()
          }
          {
            if ctx.props().project.links.git != "" {
              html! {
                <a class="animate-pulse filter invert hover:invert-0 transition-all" href={format!("{}", ctx.props().project.links.git)} target="_blank">
                  <Icon icon="gh" />
                </a>
              }
            } else {
              html! {}
            }
          }

        </div>
      </div>
    }
  }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Project {
  name: String,
  img: String,
  description: String,
  links: Links,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Links {
  git: String,
  other: Vec<MiscLink>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct MiscLink {
  link: String,
  icon: String,
}

use yew::prelude::*;
use yewtil::NeqAssign;
use serde::Deserialize;
use crate::components::Icon;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub project: Project,
}

pub struct ProjectBlock {
  props: Props,
}

impl Component for ProjectBlock {
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
      <div class="flex flex-col bg-black bg-opacity-30 w-11/12 md:w-[38rem] h-auto md:h-[34rem] my-6 mx-16 md:mx-8 rounded-t-md rounded-b-lg">
        <div class="flex justify-center rounded-t-md circle-bg bg-opacity-70 bg-secondary-accent-lt">
          <h1 class="text-2xl md:text-3xl font-mulish select-none">{ &self.props.project.name }</h1>
        </div>
        <img class="max-h-[405px]" src={format!("/files/{}", &self.props.project.img)} alt={format!("{} banner", &self.props.project.name)} />
        <p class="overflow-hidden px-5 py-3 md:text-base text-xs self-center">{ &self.props.project.description }</p>

        <div class="flex justify-end mt-auto primary-accent-wavy rounded-b-lg">
          {
            self.props.project.links.other.iter()
              .map(|l| {
                html! {
                  <a class="animate-pulse hover:filter hover:invert transition-all" href={l.link.clone()} target="_blank">
                    <Icon icon={l.icon.clone()} />
                  </a>}
              })
              .collect::<Html>()
          }
          <a class="animate-pulse hover:filter hover:invert transition-all" href={format!("{}", &self.props.project.links.git)} target="_blank">
            <Icon icon="gh" />
          </a>
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
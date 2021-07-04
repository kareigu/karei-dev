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
      <div class="flex flex-col bg-black bg-opacity-30 sm:w-11/12 md:w-3/6 sm:h-80 md:h-2/6 my-6 mx-16 rounded-md">
        <div class="flex justify-center rounded-t-md bg-gradient-to-br from-tertiary-accent-md to-tertiary-accent-dk">
          <h1 class="sm:text-2xl md:text-3xl font-mulish select-none">{ &self.props.project.name }</h1>
        </div>
        <img src={format!("/files/{}", &self.props.project.img)} alt={format!("{} banner", &self.props.project.name)} />
        <p class="overflow-hidden px-5 py-3 md:text-base sm:text-xs self-center">{ &self.props.project.description }</p>

        <div class="flex justify-end bg-gradient-to-bl from-secondary-accent-md to-secondary-accent-dk rounded-b-lg">
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
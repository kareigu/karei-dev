use crate::components::{Project, ProjectBlock};
use crate::utils;
use yew::prelude::*;
use yew_services::{fetch::FetchTask, ConsoleService};
use yewtil::NeqAssign;

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {}

pub enum Msg {
  RecProjects(utils::ProjectRes),
  Nothing,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Projects {
  props: Props,
  link: ComponentLink<Self>,
  task: Option<FetchTask>,
  projects: Option<Vec<Project>>,
}

impl Component for Projects {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
      link: link.clone(),
      task: utils::get_projects(link),
      projects: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::RecProjects(p) => {
        self.projects = Some(p.projects);
        self.task = None;
        true
      }
      Msg::Nothing => {
        ConsoleService::error("Request failed");
        self.task = None;
        false
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
  }

  fn view(&self) -> Html {
    ConsoleService::log(format!("{:?}", self).as_str());
    html! {
      <div class="flex flex-col desktop:flex-row desktop:flex-wrap justify-center items-center">
        {
          if let Some(projects) = &self.projects {
            projects.iter()
              .map(|p| {
                html! { <ProjectBlock project=p.clone() /> }
              })
              .collect()
          } else {
            html! { <img src="/static/spinner.svg" alt="loading spinner" /> }
          }
        }
      </div>
    }
  }
}

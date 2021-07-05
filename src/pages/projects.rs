use yew::prelude::*;
use yewtil::NeqAssign;
use crate::components::{ProjectBlock, Project};
use serde::Deserialize;
use yew_services::{
  ConsoleService, FetchService, 
  fetch::{
    FetchTask,
    Request,
    Response
}};
use anyhow::Error;
use yew::format::{Json, Nothing};

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
}

pub enum Msg {
  RecProjects(ProjectRes),
  Nothing
}

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
      task: get_projects(link),
      projects: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::RecProjects(p) => {
        self.projects = Some(p.projects);
        self.task = None;
        true
      },
      Msg::Nothing => {
        ConsoleService::error("Request failed");
        self.task = None;
        false
      },
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


fn get_projects(link: ComponentLink<Projects>) -> Option<FetchTask> {
  let request = Request::get("/api/v1/projects")
    .body(Nothing)
    .expect("Failed to create request");

  let callback = link.callback(|response: Response<Json<Result<ProjectRes, Error>>>| {
    if let (meta, Json(Ok(body))) = response.into_parts() {
      if meta.status.is_success() {
        return Msg::RecProjects(body);
      }
    }
    Msg::Nothing
  });

  match FetchService::fetch(request, callback) {
    Ok(f) => Some(f),
    Err(e) => {ConsoleService::error(format!("{:?}", e).as_str()); None},
  }
}


#[derive(Deserialize, Debug, Clone)]
pub struct ProjectRes {
  projects: Vec<Project>,
}

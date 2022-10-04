use crate::components::{Project, ProjectBlock};
use crate::utils;
use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::console::error_1;
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties, Debug)]
pub struct Props {}

pub enum Msg {
  RecProjects(ProjectsRes),
  Nothing,
}

pub struct Projects {
  projects: Option<Vec<Project>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectsRes {
  pub projects: Vec<Project>,
}

impl Component for Projects {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    ctx.link().send_future(async {
      match utils::fetch_get::<ProjectsRes>("/api/v1/projects").await {
        Ok(projects) => Msg::RecProjects(projects),
        Err(e) => {
          error_1(&e);
          Msg::Nothing
        }
      }
    });

    Self { projects: None }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::RecProjects(p) => {
        self.projects = Some(p.projects);
        true
      }
      Msg::Nothing => {
        error_1(&JsValue::from("Request failed"));
        false
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class="flex flex-col desktop:flex-row desktop:flex-wrap justify-center items-center">
        {
          if let Some(projects) = &self.projects {
            projects.iter()
              .map(|p| {
                html! { <ProjectBlock project={ p.clone() } /> }
              })
              .collect()
          } else {
            html! { <img class="my-24" src="/static/spinner.svg" alt="loading spinner" /> }
          }
        }
      </div>
    }
  }
}

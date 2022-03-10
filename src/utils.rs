use yew::prelude::*;
use yew_router::Routable;
use crate::pages::{Projects, ProjectsMsg};
use crate::{App, router::AppRoutes};
use crate::components::Project;
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

pub fn get_current_page() -> AppRoutes {
  let string_value = match yew::utils::document().url() {
    Ok(u) => {
      let s = u.split("/").last().unwrap();
      format!("/{}", s)
    },
    Err(e) => format!("{:?}", e),
  };

  AppRoutes::recognize(&string_value).unwrap_or(AppRoutes::NotFound)
}

pub fn update_menu_bar(app: &mut App) -> bool {
  let new = get_current_page();
  if new == app.active_route {
    false
  } else {
    app.active_route = new;
    true
  }
}


pub fn get_projects(link: ComponentLink<Projects>) -> Option<FetchTask> {
  let request = Request::get("/api/v1/projects")
    .body(Nothing)
    .expect("Failed to create request");

  let callback = link.callback(|response: Response<Json<Result<ProjectRes, Error>>>| {
    if let (meta, Json(Ok(body))) = response.into_parts() {
      if meta.status.is_success() {
        return ProjectsMsg::RecProjects(body);
      }
    }
    ProjectsMsg::Nothing
  });

  match FetchService::fetch(request, callback) {
    Ok(f) => Some(f),
    Err(e) => {ConsoleService::error(format!("{:?}", e).as_str()); None},
  }
}


#[derive(Deserialize, Debug, Clone)]
pub struct ProjectRes {
  pub projects: Vec<Project>,
}
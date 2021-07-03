use yew::prelude::*;
use yewtil::NeqAssign;
use yew_services::{ConsoleService, fetch::FetchTask};
use crate::components::{Button, Colour};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Forked {
  props: Props,
  forks: Option<Vec<Fork>>,
  task: Option<FetchTask>
}

pub enum Msg {
  RecForks(Forks),
  Nothing,
}

impl Component for Forked {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
      task: get_forks(link),
      forks: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::RecForks(s) => {
        ConsoleService::log(format!("{:?}", s).as_str());
        self.forks = Some(s.forks);
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
    html! {
      <div class="flex flex-col justify-center items-center">
        <h1 class="font-mulish lg:text-4xl md:text-3xl">{"Forked YouTube Gaming"}</h1>
        <img 
          src="https://raw.githubusercontent.com/mxrr/BetterYTG/master/src/assets/icons/BetterYTG_red_128.png" 
          alt="forkedytg logo"
          class="w-64 h-64"
        />

        <div class="flex flex-row justify-center">
          <Button text="Download for Firefox" colour=Colour::Tertiary />
          <Button text="Download for Chrome" colour=Colour::Custom("bg-secondary-accent-dk".to_string()) />
        </div>
        { 
          if let Some(forks) = self.forks.clone() {
            forks.into_iter()
              .map(|f| html! {
                <div>
                  <p>{ f.version }</p>
                  <p>{ f.filename }</p>
                </div>
            }).collect::<Html>()
          } else {
            html! {
              <div>
                <p>{ "Loading" }</p>
              </div>
            }
          }
        }
      </div>
    }
  }
}

fn get_forks(link: ComponentLink<Forked>) -> Option<FetchTask> {
  use yew_services::FetchService;
  use yew_services::fetch::Request;
  use yew_services::fetch::{Response};
  use yew::format::{Nothing, Json};
  use anyhow::Error;

  let request = Request::get("/api/v1/forks")
    .body(Nothing)
    .expect("Failed to create request");

  let callback = link.callback(|response: Response<Json<Result<Forks, Error>>>| {
    if let (meta, Json(Ok(body))) = response.into_parts() {
      if meta.status.is_success() {
        return Msg::RecForks(body);
      }
    }
    Msg::Nothing
  });

  let task: Option<FetchTask> = match FetchService::fetch(request, callback) {
    Ok(f) => Some(f),
    Err(e) => {ConsoleService::error(format!("{:?}", e).as_str()); None},
  };

  task
}

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Forks {
  forks: Vec<Fork>
}
#[derive(Deserialize, Debug, Clone)]
struct Fork {
  version: String,
  filename: String,
}
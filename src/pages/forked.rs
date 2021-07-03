use yew::prelude::*;
use yewtil::NeqAssign;
use yew_services::{ConsoleService, fetch::FetchTask};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Forked {
  props: Props,
  link: ComponentLink<Forked>,
  forks: Forks,
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

    let forks = Forks {
      forks: vec![]
    };

    Self {
      props,
      link,
      task,
      forks
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::RecForks(s) => {ConsoleService::log(format!("{:?}", s).as_str()); true },
      Msg::Nothing => {ConsoleService::error("Request failed"); true },
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
  }

  fn view(&self) -> Html {
    html! {
      <div class="flex flex-col justify-center items-center">
        {"Forked YouTube Gaming"}
      </div>
    }
  }
}

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Forks {
  forks: Vec<Fork>
}
#[derive(Deserialize, Debug)]
struct Fork {
  version: String,
  filename: String,
}
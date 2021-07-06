use yew::prelude::*;
use yewtil::NeqAssign;
use yew_services::{
  ConsoleService, FetchService, 
  fetch::{
    FetchTask,
    Request,
    Response
  }};
use crate::components::{Button, Colour};
use yew::format::{Nothing, Json};
use anyhow::Error;
use serde::Deserialize;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Forked {
  props: Props,
  link: ComponentLink<Forked>,
  forks: Option<Vec<Fork>>,
  task: Option<FetchTask>,
  version_list_visible: bool,
}

pub enum Msg {
  RecForks(Forks),
  Nothing,
  ToggleVersionList,
}

impl Component for Forked {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
      link: link.clone(),
      task: get_forks(link),
      forks: None,
      version_list_visible: false,
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
      Msg::ToggleVersionList => {
        self.version_list_visible = !self.version_list_visible;
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
  }

  fn view(&self) -> Html {
    let firefox_button = html! {
      <Button 
        text="Download for Firefox" 
        colour=Colour::Custom("bg-gradient-to-br from-[#ff5b2d] to-[#ffc328] hover:from-base-lt hover:to-base-lt".to_string()) 
        icon="/static/firefox.png" 
      />
    };
    let base_url = "https://mxrr.dev/files/";

    html! {
      <div class="flex flex-col justify-center items-center mt-2 animate-blur-in">
        <h1 class="font-mulish lg:text-4xl md:text-3xl">{"Forked YouTube Gaming"}</h1>
        <img 
          src="https://raw.githubusercontent.com/mxrr/BetterYTG/master/src/assets/icons/BetterYTG_red_128.png" 
          alt="forkedytg logo"
          class="w-64 h-64"
        />
        <h2 class="mb-4 text-center">{ "ForkedYTG enhances YouTube livestreams with more emotes (Twitch, BTTV Emotes), new features, and more." }</h2>

        <div class="flex flex-row justify-center flex-wrap animate-blur-in">
          <div>
          {
            if let Some(forks) = &self.forks {
              html! {
                <>
                  <a href={format!("{}{}", base_url, forks[0].filename)}>
                    {firefox_button}
                  </a>
                  <div class="flex flex-col justify-center items-center">
                    <span onclick={self.link.callback(|_| Msg::ToggleVersionList)}>
                      <Button 
                        text={ if self.version_list_visible { "Hide all versions" } else { "Show all versions" } } 
                        colour=Colour::Secondary 
                      />
                    </span>
                    { if self.version_list_visible {
                        forks.into_iter()
                        .map(|f| html! {
                          <div class="animate-slide-down">
                            <a href={format!("{}{}", base_url, f.filename.clone())}>
                              <Button text=f.version.clone() colour=Colour::Primary />
                            </a>
                          </div>
                        }).collect::<Html>()
                      } else {
                        html!{}
                      }
                    }
                  </div>
                </>
              }
            } else {
              {firefox_button}
            }
          }
          </div>
          <div>
            <a href="https://chrome.google.com/webstore/detail/forked-youtube-gaming/dehjikmfbdokdlkkchepifefodnmkmld?hl=en">
              <Button 
                text="Download for Chrome" 
                colour=Colour::Custom("bg-[#4086f4]".to_string())
                icon="/static/chrome.png"
              />
            </a>
          </div>
        </div>
      </div>
    }
  }
}

fn get_forks(link: ComponentLink<Forked>) -> Option<FetchTask> {
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

  match FetchService::fetch(request, callback) {
    Ok(f) => Some(f),
    Err(e) => {ConsoleService::error(format!("{:?}", e).as_str()); None},
  }
}
#[derive(Deserialize, Debug, Clone)]
pub struct Forks {
  forks: Vec<Fork>
}
#[derive(Deserialize, Debug, Clone)]
struct Fork {
  version: String,
  filename: String,
}
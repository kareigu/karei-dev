use crate::{
  components::{Button, Colour},
  utils,
};
use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::console::{error_1, log_1};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct Forked {
  forks: Option<Vec<Fork>>,
  version_list_visible: bool,
}

pub enum Msg {
  RecForks(Forks),
  Nothing,
  ToggleVersionList,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Forks {
  forks: Vec<Fork>,
}
#[derive(Deserialize, Debug, Clone)]
struct Fork {
  version: String,
  filename: String,
}

impl Component for Forked {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    ctx.link().send_future(async {
      match utils::fetch_get("api/v1/forks").await {
        Ok(forks) => Msg::RecForks(forks),
        Err(e) => {
          error_1(&e);
          Msg::Nothing
        }
      }
    });

    Self {
      forks: None,
      version_list_visible: false,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::RecForks(s) => {
        log_1(&JsValue::from(format!("{:?}", s).as_str()));
        self.forks = Some(s.forks);
        true
      }
      Msg::Nothing => {
        error_1(&JsValue::from("Request failed"));
        false
      }
      Msg::ToggleVersionList => {
        self.version_list_visible = !self.version_list_visible;
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let firefox_button = html! {
      <Button
        text="Download for Firefox"
        colour={Colour::Custom("bg-gradient-to-br from-[#ff5b2d] to-[#ffc328] hover:from-base-lt hover:to-base-lt".to_string())}
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
                    <span onclick={ctx.link().callback(|_| Msg::ToggleVersionList)}>
                      <Button
                        text={ if self.version_list_visible { "Hide all versions" } else { "Show all versions" } }
                        colour={Colour::Secondary}
                      />
                    </span>
                    { if self.version_list_visible {
                        forks.into_iter()
                        .map(|f| html! {
                          <div class="animate-slide-down">
                            <a href={format!("{}{}", base_url, f.filename.clone())}>
                              <Button text={f.version.clone()} colour={Colour::Primary} />
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
                colour={Colour::Custom("bg-[#4086f4]".to_string())}
                icon="/static/chrome.png"
              />
            </a>
          </div>
        </div>
      </div>
    }
  }
}

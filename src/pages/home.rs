use crate::components::{Project, ProjectBlock};
use crate::pages::{projects::ProjectsRes, ProjectsMsg};
use crate::router::AppRoutes;
use crate::utils;
use wasm_bindgen::JsValue;
use web_sys::console::error_1;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {}

#[allow(dead_code)]
pub struct Home {
  featured_project: Option<Project>,
}

impl Component for Home {
  type Message = ProjectsMsg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    ctx.link().send_future(async {
      match utils::fetch_get::<ProjectsRes>("/api/v1/projects").await {
        Ok(projects) => ProjectsMsg::RecProjects(projects),
        Err(e) => {
          error_1(&e);
          ProjectsMsg::Nothing
        }
      }
    });

    Self {
      featured_project: None,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::RecProjects(p) => {
        if let Some(featured) = p.projects.get(1) {
          self.featured_project = Some(featured.clone());
          true
        } else {
          error_1(&JsValue::from("Request failed"));
          false
        }
      }
      Self::Message::Nothing => {
        error_1(&JsValue::from("Request failed"));
        false
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class="flex flex-col lg:flex-row justify-center lg:items-start bg-base-dk bg-opacity-80 rounded-2xl items-center my-10 animate-blur-in">
        <div class="w-52 h-52 rounded-full bg-logo bg-cover mt-8 lg:mt-52 lg:mx-8" >
          <div class="w-52 h-52 rounded-full border-primary-accent-md border-2 select-none pointer-events-none">
            <div class="w-52 h-52 rounded-full border-secondary-accent-md border-2 animate-pulse sm:animate-ping select-none pointer-events-none"/>
          </div>
        </div>

        <div class="flex flex-col gap-4 justify-center items-center lg:mx-8">
        <div class="flex flex-col gap-2 justify-center items-center">
        {
        if let Some(project) = &self.featured_project {
            html! {
            <>
                <h1 class="font-mulish lg:text-4xl md:text-3xl mt-24 lg:mt-8 select-none">{"Featured Project"}</h1>
                <ProjectBlock project={project.clone()} />
            </>
            }
        } else {
            html! { <img class="my-24 w-32 h-32" src="/static/spinner.svg" alt="loading spinner" /> }
        }
        }
        </div>

        <div class="flex flex-col justify-center transition-all items-center animate-slide-up mt-6 bg-black bg-opacity-20 mb-20 rounded-md border-2 border-black border-opacity-20 hover:border-opacity-100 hover:border-base-lt">
            <Link<AppRoutes> to={AppRoutes::Forked} classes={classes!("flex flex-col justify-center items-center pt-5 px-14".to_string())}>
                <h1 class="font-mulish lg:text-4xl md:text-3xl select-none">{"Forked YouTube Gaming"}</h1>
                <img
                    src="https://raw.githubusercontent.com/mxrr/BetterYTG/master/src/assets/icons/BetterYTG_red_128.png"
                    alt="forkedytg logo"
                    class="w-64 h-64"
                />
            </Link<AppRoutes>>
        </div>
        </div>
      </div>
    }
  }
}

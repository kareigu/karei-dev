use crate::components::{Project, ProjectBlock};
use crate::pages::ProjectsMsg;
use crate::router::AppRoutes;
use crate::utils;
use yew::prelude::*;
use yew_router::components::Link;
use yew_services::fetch::FetchTask;
use yew_services::ConsoleService;
use yewtil::NeqAssign;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[allow(dead_code)]
pub struct Home {
  props: Props,
  link: ComponentLink<Self>,
  task: Option<FetchTask>,
  featured_project: Option<Project>,
}

impl Component for Home {
  type Message = ProjectsMsg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
      link: link.clone(),
      task: utils::get_projects(link),
      featured_project: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Self::Message::RecProjects(p) => {
        if let Some(featured) = p.projects.get(1) {
          self.featured_project = Some(featured.clone());
          self.task = None;
          true
        } else {
          ConsoleService::error("Request failed");
          self.task = None;
          false
        }
      }
      Self::Message::Nothing => {
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
    html! {
      <div class="flex flex-col justify-center items-center mt-10 animate-blur-in">
        <div class="w-52 h-52 rounded-full bg-logo bg-cover mt-56" >
          <div class="w-52 h-52 rounded-full border-primary-accent-md border-2 select-none pointer-events-none">
            <div class="w-52 h-52 rounded-full border-secondary-accent-md border-2 animate-pulse sm:animate-ping select-none pointer-events-none"/>
          </div>
        </div>

        {
          if let Some(project) = &self.featured_project {
            html! {
              <>
                <h1 class="font-mulish lg:text-4xl md:text-3xl mt-24 select-none">{"Featured Project"}</h1>
                <ProjectBlock project=project.clone() />
              </>
            }
          } else {
            html! { <img src="/static/spinner.svg" alt="loading spinner" /> }
          }
        }

        <div class="flex flex-col justify-center transition-all items-center animate-slide-up mt-6 bg-black bg-opacity-20 mb-20 rounded-md border-2 border-black border-opacity-20 hover:border-opacity-100 hover:border-base-lt">
          <Link<AppRoutes> route=AppRoutes::Forked classes=classes!("flex flex-col justify-center items-center pt-5 px-14".to_string())>
            <h1 class="font-mulish lg:text-4xl md:text-3xl select-none">{"Forked YouTube Gaming"}</h1>
            <img
              src="https://raw.githubusercontent.com/mxrr/BetterYTG/master/src/assets/icons/BetterYTG_red_128.png"
              alt="forkedytg logo"
              class="w-64 h-64"
            />
          </Link<AppRoutes>>
        </div>



      </div>
    }
  }
}

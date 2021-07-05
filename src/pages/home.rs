use yew::prelude::*;
use yewtil::NeqAssign;
use yew_router::components::Link;
use crate::router::AppRoutes;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Home {
  props: Props,
}

impl Component for Home {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      props
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
  }

  fn view(&self) -> Html {
    html! {
      <div class="flex flex-col justify-center items-center mt-10">
        <div class="w-52 h-52 rounded-full bg-logo bg-cover mt-56" >
          <div class="w-52 h-52 rounded-full border-tertiary-accent-lt border-2 select-none pointer-events-none">
            <div class="w-52 h-52 rounded-full border-secondary-accent-md border-2 animate-pulse sm:animate-ping select-none pointer-events-none"/>
          </div>
        </div>

        
        <div class="flex flex-col justify-center items-center mt-56 bg-black bg-opacity-20 mb-20 pt-5 px-14 rounded-md hover:border-2 hover:border-base-lt">
          <Link<AppRoutes> route=AppRoutes::Forked classes=classes!("flex flex-col justify-center items-center".to_string())>
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
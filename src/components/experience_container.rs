use yew::{html::ChildrenRenderer, prelude::*};
use yewtil::NeqAssign;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub img: String,
  pub title: String,
  pub location: String,
  pub company: String,
  pub timespan: String,
  pub children: ChildrenRenderer<Html>,
}

pub struct ExperienceContainer {
  props: Props,
}

impl Component for ExperienceContainer {
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
    let title_styles = if self.props.title.chars().count() > 28 as usize {
      classes!("font-mulish text-l md:text-xl".to_string())
    } else if self.props.title.chars().count() > 24 as usize {
      classes!("font-mulish text-2xl".to_string())
    } else {
      classes!("font-mulish text-3xl".to_string())
    };

    let company_styles = if self.props.company.chars().count() > 34 {
      classes!("text-xs", "md:text-base")
    } else {
      classes!("text-base")
    };

    html! {
      <div class="flex flex-col justify-center animate-slide-up desktop:animate-blur-in bg-black bg-opacity-30 rounded-md w-[calc(100vw-0.75rem)] md:w-[36rem] my-6 mx-2 md:mx-8">
        <div class="flex flex-row justify-center items-center primary-accent-wavy rounded-t-md">
          <img class="ml-1 mr-auto rounded-full bg-base-lt border-2 border-tertiary-accent-lt" src={ self.props.img.clone() } alt="company logo" width="48px" height="48px" />
          <div class="flex flex-col justify-center items-center mx-auto">
            <h1 class=title_styles>{ &self.props.title }</h1>
            <h2 class=company_styles>{ &self.props.company }</h2>
            <h2>{ format!("📍{}", &self.props.location) }</h2>
          </div>
          <div class="mr-1 ml-auto md:w-[48px] md:h-[48px]" />
        </div>
        
        <div class="bg-white bg-opacity-5 rounded-sm">
          <ul>
            { self.props.children.clone() }
          </ul>
        </div>
        <div class="flex justify-center items-center py-2 bg-tertiary-accent-dk bg-opacity-70 tertiary-zigzag rounded-b-md">
          <h2 class="bg-black bg-opacity-50 px-1 rounded-md">{ format!("⏳{}", &self.props.timespan) }</h2>
        </div>
      </div>
    }
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct LProps {
  pub text: String,
}

pub struct ListItem {
  props: LProps
}

impl Component for ListItem {
  type Message = ();
  type Properties = LProps;

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
      <li class="odd:bg-white odd:bg-opacity-10 px-4 py-1 last:pb-4">{ &self.props.text }</li>
    }
  }
}
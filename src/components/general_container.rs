use yew::{html::ChildrenRenderer, prelude::*};
use yewtil::NeqAssign;

#[derive(Clone, PartialEq)]
pub enum Type {
  List,
  Table
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub title: String,
  pub content_type: Type,
  pub children: ChildrenRenderer<Html>,
}

pub struct GeneralContainer {
  props: Props,
}

impl Component for GeneralContainer {
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

    html! {
      <div class="flex flex-col justify-center bg-black bg-opacity-30 rounded-md w-[calc(100vw-0.75rem)] md:w-[36rem] my-6 mx-2 md:mx-8">
        <div class="flex flex-row justify-center items-center primary-accent-wavy py-2 rounded-t-md">
          <h1 class=title_styles>{ &self.props.title }</h1>
        </div>
        
        <div class="bg-white bg-opacity-5 rounded-sm flex justify-center py-2 items-center">
          {
            match &self.props.content_type {
              Type::List => {
                html! {
                <ul>
                  { self.props.children.clone() }
                </ul> 
                }
              },
              Type::Table => {
                html! {
                  <table>
                    { self.props.children.clone() }
                  </table>
                }
              }
            }
          }
        </div>
        <div class="flex justify-center pb-2 circle-bg bg-opacity-70 bg-secondary-accent-lt rounded-b-md h-2" />
      </div>
    }
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TProps {
  pub label: String,
  pub text: String,
}

pub struct TableItem {
  props: TProps
}

impl Component for TableItem {
  type Message = ();
  type Properties = TProps;

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
      <tr class="px-4 py-1 rounded-sm select-none">
        <td>{ &self.props.label }</td>
        <td>{ &self.props.text }</td>
      </tr>
    }
  }
}
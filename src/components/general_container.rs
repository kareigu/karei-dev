use yew::{html::ChildrenRenderer, prelude::*};

#[derive(Clone, PartialEq)]
pub enum Type {
  List,
  Table,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub title: String,
  pub content_type: Type,
  pub children: ChildrenRenderer<Html>,
}

pub struct GeneralContainer {}

impl Component for GeneralContainer {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let title_styles = if ctx.props().title.chars().count() > 28 as usize {
      classes!("font-mulish text-l md:text-xl".to_string())
    } else if ctx.props().title.chars().count() > 24 as usize {
      classes!("font-mulish text-2xl".to_string())
    } else {
      classes!("font-mulish text-3xl".to_string())
    };

    html! {
      <div class="flex flex-col justify-center animate-slide-up desktop:animate-blur-in bg-black bg-opacity-30 rounded-md w-[calc(100vw-0.75rem)] md:w-[36rem] my-6 mx-2 md:mx-8">
        <div class="flex flex-row justify-center items-center primary-accent-wavy py-2 rounded-t-md">
          <h1 class={ title_styles }>{ ctx.props().title.clone() }</h1>
        </div>

        <div class="bg-white bg-opacity-5 rounded-sm flex justify-center py-2 items-center">
          {
            match ctx.props().content_type {
              Type::List => {
                html! {
                <ul>
                  { ctx.props().children.clone() }
                </ul>
                }
              },
              Type::Table => {
                html! {
                  <table>
                    { ctx.props().children.clone() }
                  </table>
                }
              }
            }
          }
        </div>
        <div class="flex justify-center py-2 circle-bg bg-opacity-70 bg-secondary-accent-lt rounded-b-md h-2" />
      </div>
    }
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TProps {
  pub label: String,
  pub text: String,
}

pub struct TableItem {}

impl Component for TableItem {
  type Message = ();
  type Properties = TProps;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <tr class="px-4 py-1 rounded-sm select-none">
        <td>{ ctx.props().label.clone() }</td>
        <td>{ ctx.props().text.clone() }</td>
      </tr>
    }
  }
}

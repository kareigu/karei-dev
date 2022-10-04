use crate::{router::AppRoutes, App};
use serde::Deserialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew_router::Routable;

pub fn get_current_page() -> AppRoutes {
  web_sys::window()
    .and_then(|window| window.document())
    .and_then(|document| document.base_uri().ok())
    .and_then(|o| o)
    .and_then(|s| Some(s.split('/').last()?.to_string()))
    .and_then(|s| AppRoutes::recognize(&s))
    .unwrap_or(AppRoutes::NotFound)
}

pub fn update_menu_bar(app: &mut App) -> bool {
  let new = get_current_page();
  if new == app.active_route {
    false
  } else {
    app.active_route = new;
    true
  }
}

pub async fn fetch_get<T>(url: &str) -> Result<T, JsValue>
where
  T: for<'de> Deserialize<'de>,
{
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init(url, &opts)?;

  let window = web_sys::window().ok_or_else(|| JsValue::from("No window"))?;

  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let res: Response = resp_value.dyn_into()?;

  let json = JsFuture::from(res.json()?).await?;
  Ok(serde_wasm_bindgen::from_value(json)?)
}

use yew_router::Routable;
use crate::router::AppRoutes;

pub fn get_current_page() -> AppRoutes {
  let string_value = match yew::utils::document().url() {
    Ok(u) => {
      let s = u.split("/").last().unwrap();
      format!("/{}", s)
    },
    Err(e) => format!("{:?}", e),
  };

  AppRoutes::recognize(&string_value).unwrap_or(AppRoutes::NotFound)
}
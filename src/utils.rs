use yew_router::Routable;
use crate::{App, router::AppRoutes};

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

pub fn update_menu_bar(app: &mut App) -> bool {
  let new = get_current_page();
  if new == app.active_route {
    false
  } else {
    app.active_route = new;
    true
  }
}
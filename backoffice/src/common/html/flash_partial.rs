use maud::{Markup, html};
use shared::utils::flash::Flash;

pub fn flash_partial(flash: Flash) -> Markup {
    html! {
        div #alert hx-swap-oob="true" {
            (flash.as_html())
        }
    }
}

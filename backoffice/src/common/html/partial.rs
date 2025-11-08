use maud::{Markup, html};
use shared::utils::flash::Flash;

pub fn flash_partial(flash: Flash) -> Markup {
    html! {
        span hidden hx-swap-oob="beforeend:#alert" {
            (flash.as_html())
        }
    }
}

pub fn command_list_partial(commands: Vec<Markup>) -> Markup {
    html! {
        @for command in commands.iter() {
            span hidden hx-swap-oob="beforeend:#command" {
                (command)
            }
        }
    }
}

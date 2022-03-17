use yew::prelude::*;
use yew::{function_component, html, Html};

#[function_componen(Footer)]
pub fn footer() -> Html {
    html! {
        <footer id="footer" />
    }
}

pub enum Msg {}

pub struct Footer;

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This givess us a component's "Scope" which allows us to send messages,
        // etc to the component.
        html! {
            <footer id="footer">
            </footer>
        }
    }
}

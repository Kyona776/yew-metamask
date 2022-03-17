pub mod footer;
pub mod header;
use header::{Header};
use footer::{Footer};

use yew::{ html, Children, Component, };

pub struct Layout;

pub enum Msg {
}

impl Component for Layout {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>

            </>
        }
    }
}
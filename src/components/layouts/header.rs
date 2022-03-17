use yew::prelude::*;
use yew::{Component, Context, html, Html};

pub enum Msg {}

pub struct Header;

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header id="header">
            
            </header>
        }
    }
}
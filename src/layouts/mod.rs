pub mod footer;
pub mod header;
use self::header::{Header};
use self::footer::{Footer};

use yew::prelude::*;

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
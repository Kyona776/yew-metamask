use yew::prelude::*;

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

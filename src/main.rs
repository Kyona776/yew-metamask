#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use yew::prelude::{Component, Context, html, Html};
use web_sys::{console};

// mod layouts;
// use layouts::{footer, header};
// mod crate::footer;
mod layouts;
use crate::layouts::{header, footer};
// mod .::header;
// use crate::layouts::header::Header;
mod detectMetamask;
use crate::detectMetamask::{ check_provider, WalletApi };

enum Msg {
    AddOne,
    ConnectEth
}


struct App {
    value: i64,
    // ethApi: Option<dyn EthereumProvider>
    ethApi: WalletApi,
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut eth : WalletApi = Default::default();
        match eth.load_ethereum() {
            Ok(value) => console::log_1(&value),
            Err(error) => console::log_1(&error)
        }
        Self {
            value:0,
            // ethApi : WalletApi::new(),
            ethApi: eth,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::ConnectEth => {
                check_provider();
                self.ethApi.connect();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This givess us a component's "Scope" which allows us to send messages,
        // etc to the component.
        let link = ctx.link();
        html! {
        <>// id={ctx.props().id.clone()}>
            <header::Header />

                
            <main>
            <button onclick={link.callback(|_| Msg::AddOne)}>
            {"+1"}
            </button>
            <p>{self.value}</p>
            {if self.ethApi.loaded {"load"} else {"not wallet installed"}}
            <button onclick={link.callback(|_| Msg::ConnectEth)}>
                {"connect"}
            </button>
            </main>
            <footer::Footer />
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

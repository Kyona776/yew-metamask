use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window,
    console
};

use js_sys::{Error, Reflect, Boolean, JsString,
    Object, Promise, Function, Array, Proxy
};
/*methods:
    'eth_requestAccounts' : request permission for connect user metamask
    'eth_accounts' : get list of account pub


*/

#[derive(Debug, Default, Clone)]
pub struct WalletApi {
    pub loaded : bool,
    target : Option<JsValue>,
    isMetaMask: Boolean,
    currentAccount: Option<JsValue>,
    isConnectedFn: Option<Function>,
    requestFn : Option<Function>,
    onFn : Option<Function>
}

/*
enum Pramas<T, > {

}

struct RequestArguments {
    method: &JsString,
    params: &Array // array of object
}
*/

struct ConnectInfo {
    chainId: JsString
}

/*
pub trait EthereumProvider {
    fn new();
    // fn init();
    // fn isConnencted() -> Boolean;
    fn request(args: (&JsString, &Array));
    fn on(event:JsString, callback:Function);
    fn connect();
    fn isConnected();
    fn load_ethereum();
}
*/

// impl EthereumProvider for WalletApi {
impl WalletApi {
    fn new() -> Self {
        WalletApi {
            loaded: false,
            target : None,
            // isMetaMask: Boolean,
            isMetaMask: Boolean::from(false),
            currentAccount: None,
            isConnectedFn: None,
            requestFn : None,
            onFn : None
        }

        /*
        match instance::load_ethereum() {
            Ok(success) => console::log_1(success),
            Err(failure) => console::log_1(failure),
        }
        instance
        */
    }
    /*
    */
    // fn init() {}

    fn request(&self, method:&JsString, params:&Array) -> Result<JsValue, JsValue> {
        // let connected = Self::isConnected()?;

        if let Ok(connected) = self.isConnected() {
            if connected.as_bool().unwrap() {
                Ok(Reflect::apply(&self.requestFn.as_ref().unwrap(), method, params)?)
            } else {
                return Err("".into())
            }
            // if (connected)
            // Reflect::apply(self.requestFn, method, params)?
        } else { // else if let
            Err(JsValue::from_str(&"got errors"))
        }
    }

    pub fn isConnected(&self) -> Result<JsValue, JsValue> {
        if self.loaded {
            Ok(Reflect::apply(&self.isConnectedFn.as_ref().unwrap(), &"isConnected".into(), &Array::new())?)
        } else {
            Err("not loaded".into())
        }
    }

    pub async fn connect(& mut self) //-> Result<Promise, JsValue> {
        {
        // console::log_1(Reflect::apply(&self.isConnectedFn.as_ref().unwrap(), &"eth_requestAccounts".into(), &Array::new()).as_ref().unwrap());
        match self.request(&"eth_requestAccounts".into(), &Array::new()) {
            Ok(value) => {
                console::log_1(&value);
                let promise = Promise::resolve(&value);
                let output = JsFuture::from(promise).await;
                match output {
                    Ok(accounts) => {
                        console::log_1(&accounts);
                    },
                    Err(error) => {
                        console::log_1(&error);
                    }   
                }

                // console::log_1(&output.unwrap());
                // Ok(Promise::From(value))
            },
            //Err(error) => Err(error),
            Err(error) => {console::log_1(&error)},
        }
    }

    pub async fn disconnect(&self) {

    }

    pub fn load_ethereum(&mut self) -> Result<JsValue, JsValue> {
        // let window = window().expect("no global 'window' exists");
        // let EthereumProvider  = Reflect::get(&window, &JsValue::from_str("ethereum"))?;
        if let Some(window) = window() {
            match Reflect::get(&window, &JsValue::from_str(&"ethereum")) {
                Ok(value) if value != JsValue::UNDEFINED => {
                    // Proxy::from(value)
                    self.loaded = true;
                    self.target = Some(value);
                    // Self.currentAccount = JsValue::from_str("");
                    self.isMetaMask = Boolean::from(Reflect::get(&self.target.as_ref().unwrap(), &"isMetamask".into())?);
                    self.isConnectedFn = Some(Function::from(Reflect::get(&self.target.as_ref().unwrap(), &"isConnected".into())?)); // Reflect return JsValue or Jsvalue::UNDEFINED
                    self.requestFn = Some(Function::from(Reflect::get(&self.target.as_ref().unwrap(), &"request".into())?));
                    Ok(JsValue::from_str(&"loaded"))
                },
                Ok(value) if value == JsValue::UNDEFINED => Err("ethereum provider is undefiend".into()),
                    // if  ethProvider.has_type::<>,
                Ok(_) => Err("unexpected".into()),
                Err(_) => Err("sth wrong".into()),
            }
        } else {
            //JsError::new(&"No global objects, sth wrong here");
            Err("No Global objects".into())
        }
    }
}


#[wasm_bindgen]
extern "C" {
    // #[wasm_bindgen(ethereum)]
    // fn request();

    // JsValue.UNDEFINED

}

// #[wasm_bindgen(start)]


pub fn check_provider() {
    
    if let Some(window) = window() {
        if let Some(ethereum) = window.get(&"ethereum") {
            console::log_1(&JsValue::from(&ethereum));
        } else {
            console::error_1(&"wallet not installed".into());
        }
        // .unwrap_or("wallet not installed"); // 
    }
}


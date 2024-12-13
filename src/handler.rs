use bytes::Bytes; //used to store serialized body
// use crate::{Context,Response};
//crate means we are importing from our own project.
use hyper::{body::to_bytes, Body, Method, Request, StatusCode}; // codes
use serde::Deserialize; //derive macro to deserialize
use route_recognizer::Params;

#[derive(Clone, Debug)]
pub struct AppState { //stores things like db conn pool and clients
    // pub state_thing: String,
}

type Response = hyper::Response<hyper::Body>; //
type Error = Box<dyn std::error::Error+Send+Sync+'static>;

#[derive(Debug)]
pub struct Context{
    pub state:AppState, // to store required things like redis clients 
    pub req:Request<Body>, //req body
    pub params:Params, //parameters passed to it.
    body_bytes:Option<Bytes>
}

impl Context{ //Context.new() style calls ie. defines 
    pub fn new(state:AppState,req:Request<Body>,params:Params)->Context{ //init Context
        Context {
            state,
            req,
            params,
            body_bytes: None, //why?
        }
    }

    pub async fn jsonify<T:serde::de::DeserializeOwned>(&mut self)-> Result<T,Error> { //middleware like to parse 
        let body_bytes = match self.body_bytes{
            //ref keyword to make a reference 
            Some(ref v)=>v, //if set ie if not None, return the value.
            _ =>{ //default matching pattern.
                let body= to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body); //Wrapping in Some() as Option is return type.
                self.body_bytes.as_ref().expect("body_bytes was set above") //unwraps the value, thus ensuring that serialization occurred correctly.
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
    }
}



use std::{collections::HashMap, num::NonZeroU32}; //Custom datatype to ensure u32 is nonzero.


pub struct OTP(NonZeroU32); //tuple struct

impl OTP{
    pub fn new(value:u32)->Option<Self>{
        if value>=100000 && value<=999999{
            Some(Self(NonZeroU32::new(value)?))
        }else{
            None
        }
    }
    pub fn value(&self) -> u32 {
        self.0.get() //.0.get() and not .get() as its a tuple struct
    }
}

#[derive(Deserialize)]
pub struct NumberPayload{
    phone_number:String, //convert to correct type ie check if its a 10 digit number
    resend_requests:u8
}


pub async fn sendotp(ctx:Context){

}


pub async fn resendotp(ctx:Context){

}

use route_recognizer::{Router as InternalRouter};

pub struct Router{
    method_map:HashMap<Method,InternalRouter<Box<dyn Handler>>>
}

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn invoke(&self, context: Context) -> Response;
}

#[async_trait]
impl<F: Send + Sync + 'static, Fut> Handler for F
where
    F: Fn(Context) -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: IntoResponse,
{
    async fn invoke(&self, context: Context) -> Response {
        (self)(context).await.into_response()
    }
}


let mut router:Router = Router::new(); //define new router
router.get("/sendotp",Box::new(sendotp)); //routes 
router.post("/resendotp",Box::new(resendotp));



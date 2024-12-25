use bytes::Bytes; //used to store serialized body
// use crate::{Context,Response};
//crate means we are importing from our own project.
use hyper::{body::to_bytes, Body, Method, Request, StatusCode}; // codes
use serde::Deserialize; //derive macro to deserialize
use route_recognizer::Params;
use async_trait::async_trait;
use futures::future::Future;


//Trait implementations

#[async_trait]
pub trait Handler: Send + Sync + 'static {//trait which is to be implemented for all handler functions.
    async fn invoke(&self, context: Context) -> Response;
    //takes context as input and returns Response.
}

pub trait IntoResponse:Send+Sized{ //
    fn into_response(self)->Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Response::new(self.into())
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::new(self.into())
    }
}



#[async_trait]
//it implements Handler trait for any generic F passed to it.
impl<F: Send + Sync + 'static, Fut> Handler for F //F shud have Send,Sync and 'static traits
where
    F: Fn(Context) -> Fut, //F shud be a fn which has a Context type as arg and returns a Future.
    Fut: Future + Send + 'static, //type of Future
    Fut::Output: IntoResponse, //?
{
    async fn invoke(&self, context: Context) -> Response {
        (self)(context).await.into_response()
    }
}





#[derive(Clone, Debug)]
pub struct AppState { //stores things like db conn pool and clients
    // pub state_thing: String,
    pub database_connection:Option<String>
}

type Response = hyper::Response<hyper::Body>; //
type Error = Box<dyn std::error::Error+Send+Sync+'static>;



#[derive(Clone,Debug)]
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


// pub struct GeneratedOTP(NonZeroU32); //tuple struct

// impl GeneratedOTP{
//     pub fn new(value:u32)->Option<Self>{
//         if value>=100000 && value<=999999{
//             Some(Self(NonZeroU32::new(value)?))
//         }else{
//             None
//         }
//     }
//     pub fn value(&self) -> u32 {
//         self.0.get() //.0.get() and not .get() as its a tuple struct
//     }
// }

#[derive(Deserialize)]
struct NumberPayload{ //type for send_otp route req body
    phone_number:String, //convert to correct type ie check if its a 10 digit number
}

impl NumberPayload{
    fn is_valid_phone_no(&self)->Result<(),String>{ //add errors later.
        if self.phone_number.starts_with("+91") && self.phone_number.len()==13 {
            Ok(())
        }
        else{
            Err(String::from("Invalid phone number."))
        }
    }
}



pub async fn send_OTP(ctx:Context)-> impl IntoResponse{
    let body:NumberPayload = match ctx.jsonify().await{//deserializing the 
        Ok(v)=>v,
        Err(e)=>{
            return hyper::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(format!("could not parse JSON: {}", e).into())
            .unwrap()
        }
    };

    //check phone no is valid.
    body.is_valid_phone_no();

    //create otp and first send to sns and when successfully sent to user, then to db.





    "OTP sent successfully".to_string()
}


pub async fn resend_OTP(ctx:Context)-> impl IntoResponse{


    "OTP sent successfully".to_string() // Or use Response::new if needed
}

pub async fn verify_OTP(ctx:Context)-> impl IntoResponse{
    "OTP sent successfully".to_string()
}



/// Routing 


use route_recognizer::{Router as InternalRouter};

use crate::RoutesStruct;

pub struct Router{
    method_map:HashMap<Method,InternalRouter<Box<dyn Handler>>>
}   

impl Router{
    pub async fn new(routes:[&RoutesStruct;3]){
        for item in routes{
            match item.method{

            }
        }
    }

    pub async fn get(&self){

    }

    pub async fn post(&self){

    }

}










// use crate::{Context, Response};
// use hyper::Response;
// use hyper::StatusCode;
// use serde::Deserialize;

// pub async fn test_handler(ctx: Context) -> String {
//     format!("test called, state_thing was: {}", ctx.state.state_thing)
// }

// #[derive(Deserialize)]
// struct SendRequest {
//     name: String,
//     active: bool,
// }

// pub async fn send_handler(mut ctx: Context) -> Response {
//     let body: SendRequest = match ctx.body_json().await {
//         Ok(v) => v,
//         Err(e) => {
//             return hyper::Response::builder()
//                 .status(StatusCode::BAD_REQUEST)
//                 .body(format!("could not parse JSON: {}", e).into())
//                 .unwrap()
//         }
//     };

//     Response::new(
//         format!(
//             "send called with name: {} and active: {}",
//             body.name, body.active
//         )
//         .into(),
//     )
// }

// pub async fn param_handler(ctx: Context) -> String {
//     let param = match ctx.params.find("some_param") {
//         Some(v) => v,
//         None => "empty",
//     };
//     format!("param called, param was: {}", param)
// }



pub struct Threadpool{
    pool_size:NonZeroU32,

}





use queues::*;

use crate::handler::Context;

struct RequestQueue{
    req:Queue<Context>
}


// impl Threadpool{
//     fn new()
// }


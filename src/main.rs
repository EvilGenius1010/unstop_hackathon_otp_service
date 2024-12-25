
// //http server import 







// #[tokio::main]
// async fn main() -> Result<(), Error> {

// }

struct RoutesStruct{
    method:hyper::Method,
    route:String,
    handler:Box<dyn Handler> //Means any type passed to Box which implements the Handler trait.
}


mod httpserver;
mod utils;
mod schema;
mod threadpool;
mod errors;

use httpserver::{resend_OTP, send_OTP, verify_OTP, AppState, Handler, Router};
use hyper::Method;
use hyper::{service::make_service_fn,Server};


#[tokio::main]
async fn main(){
    const APPSTATE:AppState=AppState{
        //add database later on.
        database_connection:None
    };
    const routes_array:[RoutesStruct;3] = [
        RoutesStruct{
            method:Method::GET,
            route:"/sendOTP".to_string(),
            handler:Box::new(send_OTP)
        },
        RoutesStruct{
            method:Method::POST,
            route:"/resendOTP".to_string(),
            handler:Box::new(resend_OTP)
        },
        RoutesStruct{
            method:Method::POST,
            route:"/verifyOTP".to_string(),
            handler:Box::new(verify_OTP)
        }
            ];

    let mut router:Router = Router::new(routes_array); //define new router
    // router.get("/sendOTP",Box::new(sendotp())); //routes 
    // router.post("/resendOTP",Box::new(resendotp()));
    // router.post("/verifyOTP",Box::new(verifyOTP()));


    let service_fn = make_service_fn(|_|{

    });


    let addr = "0.0.0.0:8080".parse().expect("address creation works");
    let server = Server::bind(&addr).serve(service_fn);
    println!("Listening on http://{}", addr);
}
// //tokio and aws_sdk config imports
// use aws_config::meta::region::RegionProviderChain;
// use aws_sdk_sns::{Client, Error};
// use aws_sdk_sns::config::Region;

// //http server import 



// //dotenv and env imports
// use std::env;
// use dotenv::dotenv;

// // Function to generate a 6-digit OTP
// fn generate_otp() -> u32 {
//     use rand::Rng;
//     rand::thread_rng().gen_range(100000..999999)
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // Load environment variables from .env file
//     dotenv().ok();

//     // Ensure critical environment variables are loaded
//     let api_key = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
//     let api_secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
//     let region = env::var("AWS_REGION").unwrap_or_else(|_| "ap-south-1".to_string());

//     // Convert region to a type that satisfies ProvideRegion
//     let region_provider = RegionProviderChain::default_provider().or_else(Region::new(region));

//     // Configure AWS client
//     let config = aws_config::from_env().region(region_provider).load().await;
//     let client = Client::new(&config);

//     // Define phone number and OTP
//     let phone_number = "+918600228032"; // Replace with the recipient's phone number.
//     let otp = generate_otp(); // Generate OTP
//     let message = format!("Your OTP is: {}", otp);

//     // Publish the message to the given phone number
//     let result = client
//         .publish()
//         .message(message)
//         .phone_number(phone_number)
//         .send()
//         .await;

//     match result {
//         Ok(response) => {
//             println!("otp is {}",otp);
//             println!("Message sent successfully! Message ID: {:?}", response.message_id);
//         }
//         Err(e) => {
//             println!("Failed to send message: {}", e);
//         }
//     }

//     Ok(())

// }



mod handler;

fn main(){

}
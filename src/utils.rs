use diesel::prelude::*;
use diesel::insert_into;
use std::time::UNIX_EPOCH;
use crate::errors::DatabaseConnError;
use crate::schema::User::phone_no;
use crate::schema::OTP;
use crate::handler::GeneratedOTP;


struct NewOTP<'a>{
    otp:GeneratedOTP,
    timestamp:NaiveDateTime,
    status:bool,
    mainUser:&'a str
}


pub fn establish_connection(connection_url: &str) -> Result<PgConnection, DatabaseConnError> {
    PgConnection::establish(connection_url)
        .map_err(|_| DatabaseConnError) //map_err?
}



// Function to generate a 6-digit OTP
fn generate_otp() -> u32 {
    use rand::Rng;
    rand::thread_rng().gen_range(100000..999999)
}

pub fn send_otp_to_db(phone_no:&str){


    use std::time::SystemTime;
    let otp = generate_otp();
    let connection =  establish_connection();

    let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Error!");



    let new_otp = NewOTP{
        otp,
        timestamp: naive_timestamp,
        status: true, // Active status
        mainUser: phone_no,
    };

    
}


pub async fn send_to_sns()->Result<(),String>{

//tokio and aws_sdk config imports
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sns::{Client, Error};
use aws_sdk_sns::config::Region;


//dotenv and env imports
use std::env;
use dotenv::dotenv;

// Load environment variables from .env file
dotenv().ok();

    // Ensure critical environment variables are loaded
    let api_key = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    let api_secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    let region = env::var("AWS_REGION").unwrap_or_else(|_| "ap-south-1".to_string());

    // Convert region to a type that satisfies ProvideRegion
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new(region));

    // Configure AWS client
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Define phone number and OTP
    let phone_number = "+918859900177"; // Replace with the recipient's phone number.
    let otp = generate_otp(); // Generate OTP
    let message = format!("Your OTP is: {}", otp);

    // Publish the message to the given phone number
    let result = client
        .publish()
        .message(message)
        .phone_number(phone_number)
        .send()
        .await;

    match result {
        Ok(response) => {
            println!("otp is {}",otp);
            println!("Message sent successfully! Message ID: {:?}", response.message_id);
        }
        Err(e) => {
            println!("Failed to send message: {}", e);
        }
    }

    Ok(())

}
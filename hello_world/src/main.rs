use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};


/// Lists your DynamoDB tables in the default Region or us-east-1 if a default 
/// Region isn't set

#[tokio::main]
async fn main() {
    // firstly, Get the region
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    // Secondly, Get the AWS config
    let config = aws_config::from_env().region(region_provider).load().await;
    // Lastly Get the client 

    let resp = client.list_tables().send().await?

    println!("Tables:");

    
}

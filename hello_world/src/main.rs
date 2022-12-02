use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};


/// Lists your DynamoDB tables in the default Region or us-east-1 if a default 
/// Region isn't set

#[tokio::main]
async fn main() -> Result<(), Error> {
    // firstly, Get the region
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    // Secondly, Get the AWS config
    let config = aws_config::from_env().region(region_provider).load().await;
    // Lastly Get the client 
    let client = Client::new(&config);

    let resp = client.list_tables().send().await?;

    println!("Tables:");
 
    let names = resp.table_names().unwrap_or_default();

    for name in names  {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());

    Ok(())
}

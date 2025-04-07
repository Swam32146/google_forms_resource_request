use google_sheets4::yup_oauth2::ServiceAccountAuthenticator;
use google_sheets4::yup_oauth2::ServiceAccountKey;
use serde_json::Value;
use std::error::Error;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
mod get_json;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json_str: &str = r#"
    {
        "client_id": "hello world",
        "other_key": "some value",
        "another_key": 123
    }
    "#;

    let client_id: String = get_json::get_client_id(&json_str);

    println!("{}", String::from(client_id));
    println!("ZimZimzalabim");

    Ok(())

}
/* 
    let authenticator: ServiceAccountAuthenticator = ServiceAccountAuthenticator::builder(service_account_key)
            .build()
            .await?;

    let url: &str = "YOUR_API_ENDPOINT";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        println!("{:#?}", json);

        //Example of accessing a field, with error handling.
        if let Some(name) = json.get("name") {
            println!("Name: {}", name);
        }
        else {
            println!("'name' field not present in the JSON.");
        }

        if let Some(array) = json.get("items").and_then(Value::as_array){
            println!("Items array length: {}", array.len());
        }

        Ok(())

    } else {
        println!("API request failed with status: {}", response.status());
        Ok(())
    }
    */
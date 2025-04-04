use google_sheets4::yup_oauth2::ServiceAccountAuthenticator;
use google_sheets4::yup_oauth2::ServiceAccountKey;
use serde_json::Value;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::Read;
use serde_json::Value;


fn read_service_account_key(filepath: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let json: Value = serde_json::from_str(&contents)?;
    Ok(json)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let service_account_path: String = env::var("GOOGLE_SERVICE_ACCOUNT_PATH")?;

    let mut file: File = File::open(service_account_path);
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json: Value = serde_json::from_str(&contents)?;

    let service_account_key: ServiceAccountKey = ServiceAccountKey::from_file(&service_account_path).await?;
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
}
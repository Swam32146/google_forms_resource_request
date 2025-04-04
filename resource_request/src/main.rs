use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "YOUR_API_ENDPOINT";

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
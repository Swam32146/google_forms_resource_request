use google_sheets4::yup_oauth2::ServiceAccountAuthenticator;
use google_sheets4::yup_oauth2::ServiceAccountKey;
use std::error::Error;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
mod get_json;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
// This was a test function to do sometin
/* 
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
*/
    dotenvy::dotenv()?;

    let json_key_path = match env::var("GOOGLE_SERVICE_ACCOUNT") {
        Ok(json_file) => json_file,
        Err(_) => {
            eprintln!("Error: FILE AINT FOUND ion that env crodie");
            std::process::exit(1);
        }
    };

    println!("{}", &json_key_path);

    let google_json_file = match fs::read_to_string(&json_key_path) {
        Ok(opened_string) => opened_string,
        Err(__) => {
            eprintln!("Error this how aint got nothin");
            std::process::exit(1);
        }
    };

    println!("{}", String::to_string(&google_json_file));

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

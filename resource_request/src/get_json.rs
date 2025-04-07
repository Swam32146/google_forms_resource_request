use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize)]
struct json_data {
    #[serde(rename = "type")]
    type_field: Option<String>,
    project_id: Option<String>,
    private_key_id: Option<String>,
    private_key: Option<String>,
    client_email: Option<String>,
    client_id: Option<String>,
    auth_uri: Option<String>,
    token_uri: Option<String>,
    auth_provider_x509_cert_url: Option<String>,
    client_x509_cert_url: Option<String>,
}

pub fn get_type_field(json_string: String) -> String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.type_field,
        Err(_) => String::from(""),
    }
}

pub fn get_project_id(json_string: String) -> String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.project_id,
        Err(_) => String::from(""),
    }
}

pub fn get_private_key_id(json_string: String) -> String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.private_key_id,
        Err(_) => String::from(""),
    }
}

pub fn get_private_key(json_string: String) -> String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.private_key,
        Err(_) => String::from(""),
    }
}

pub fn get_client_email(json_string: String) -> String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.client_email,
        Err(_) => String::from(""),
    }
}


pub fn get_client_id(json_string: &str) -> String{

    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.client_id,
        Err(_) => String::from("UH oH why isnt this working"),
    }
}

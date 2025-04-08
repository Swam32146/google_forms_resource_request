use serde_json;
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
/*
pub fn get_type_field(json_string: String) -> std::string::String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.type_field,
        Err(_) => String::from(""),
    }
}

pub fn get_project_id(json_string: String) -> std::string::String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.project_id,
        Err(_) => String::from(""),
    }
}

pub fn get_private_key_id(json_string: String) -> std::string::String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.private_key_id,
        Err(_) => String::from(""),
    }
}

pub fn get_private_key(json_string: String) -> std::string::String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.private_key,
        Err(_) => String::from(""),
    }
}

pub fn get_client_email(json_string: String) -> std::string::String{
    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => container.client_email,
        Err(_) => String::from(""),
    }
}


pub fn get_client_id(json_string: &str) -> std::string::String{

    match serde_json::from_str::<json_data>(&json_string) {
        Ok(container) => 
        {
            if let Some(client_id) = container.client_id {
                client_id
            } else {
                String::from("client_id not found")
            }
        },
        Err(e) => {
            format!("Json parsing error: {}", e)
        }
    }
}

    let v= serde_json::from_str(&google_json_file);

*/

pub fn get_type_field(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.type_field.unwrap_or_else(|| String::from("type_field not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_project_id(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.project_id.unwrap_or_else(|| String::from("project_id not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_private_key_id(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.private_key_id.unwrap_or_else(|| String::from("private_key_id not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_private_key(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.private_key.unwrap_or_else(|| String::from("private_key not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_client_email(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.client_email.unwrap_or_else(|| String::from("client_email not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_client_id(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.client_id.unwrap_or_else(|| String::from("client_id not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_auth_uri(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.auth_uri.unwrap_or_else(|| String::from("auth_uri not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_token_uri(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.token_uri.unwrap_or_else(|| String::from("token_uri not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_auth_provider_x509_cert_url(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.auth_provider_x509_cert_url.unwrap_or_else(|| String::from("auth_provider_x509_cert_url not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

pub fn get_client_x509_cert_url(json_string: &str) -> String {
    match serde_json::from_str::<json_data>(json_string) {
        Ok(container) => container.client_x509_cert_url.unwrap_or_else(|| String::from("client_x509_cert_url not found")),
        Err(e) => format!("Json parsing error: {}", e),
    }
}

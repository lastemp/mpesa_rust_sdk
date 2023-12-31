use reqwest::StatusCode;

use crate::{models::models::AuthTokenResponseData, util::util::build_headers_generate_auth_token};

pub async fn get_auth_token(
    api_key: String,
    api_url: String,
) -> std::result::Result<String, String> {
    let client = reqwest::Client::new();

    let res = client
        .get(api_url)
        .headers(build_headers_generate_auth_token(api_key))
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    match response.json::<AuthTokenResponseData>().await {
                        Ok(auth_token_data) => {
                            // Handle success case
                            let k = String::from(""); //Default value.
                            let access_token = auth_token_data.access_token.as_ref().unwrap_or(&k);

                            return Ok(access_token.to_string());
                        }
                        Err(_err) => {
                            // Handle error case
                            return Err(_err.to_string());
                        }
                    }
                }
                s => {
                    let mut _x = String::from("Request failed processing, status code: ");
                    _x.push_str(&s.to_string());
                    return Err(_x.to_string());
                }
            }
        }
    };
}

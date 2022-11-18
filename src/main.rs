use http_client_experiment::{
    api::ApiClient,
    api_model::{RequestCredentials, RoomPostRequest},
};

fn main() {
    let client = reqwest::blocking::Client::new();

    let request = RoomPostRequest {
        credentials: RequestCredentials {
            base_url: "http://localhost:1234".to_string(),
            username: "abcd".to_string(),
        },
        name: "Room-1".to_string(),
    };

    let response = client.api_execute(request).unwrap();
}

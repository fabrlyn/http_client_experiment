use std::fmt::Debug;

use http_client_experiment::{
    api::{
        asyn::Client,
        syn::{self, AbstractRequest},
    },
    domain::{
        room_get::{RoomGetRequest, RoomGetResponse},
        room_post::RoomPostRequest,
        sync_client::ApiHttpClient,
        ApiResult, RequestCredentials,
    },
    http_impl::{mock_impl::MockClient, ureq_impl::Ureq},
};

fn something_else_more_specfic<A: syn::Client>(client: A)
where
    RoomGetRequest: AbstractRequest<A, Response = ApiResult<RoomGetResponse>>,
{
    let response = client.api_execute(RoomGetRequest {}).unwrap();
    println!("{response:?}");
}

fn something_else<A: syn::Client>(client: A)
where
    RoomGetRequest: AbstractRequest<A>,
{
    let response = client.api_execute(RoomGetRequest {}).unwrap();
    println!("{response:?}");
}

fn get_room<A: ApiHttpClient<E>, E: Debug>(client: &A) {
    let response = client.api_execute(RoomGetRequest {});
    println!("{response:?}");
}

fn create_room<A: ApiHttpClient<E>, E: Debug>(client: &A) {
    let request = RoomPostRequest {
        credentials: RequestCredentials {
            base_url: "http://localhost:1233".to_string(),
            username: "abcd".to_string(),
        },
        name: "Room-1".to_string(),
    };

    let response = client.api_execute(request);
    println!("{response:?}");
}

fn execute_room_flow<A: ApiHttpClient<E>, E: Debug>(client: A) {
    get_room(&client);
    create_room(&client);
}

/*
fn main() {
    let client = Ureq {};
    execute_room_flow(client);

    let client = MockClient {};
    something_else(client);

    let client = reqwest::blocking::Client::new();
    something_else_more_specfic(client);
}
*/

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let response = client.api_execute(RoomGetRequest).await;
    println!("{response:?}");
}

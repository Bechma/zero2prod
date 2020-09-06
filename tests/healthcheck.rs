use std::net::TcpListener;

fn spawn_app() -> String {
    let address = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let url = format!("http://127.0.0.1:{}", address.local_addr().unwrap().port());
    let server = zero2prod::run(address).expect("Failed to bind address.");
    tokio::spawn(server);
    url
}

#[actix_rt::test]
async fn healthcheck_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/healthcheck", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

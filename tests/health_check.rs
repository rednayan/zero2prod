use std::net::TcpListener;
use zero2prod;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("execute request success");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind random port success");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("bind address success");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

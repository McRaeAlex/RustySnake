use super::rocket;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_ping() {
    let client = Client::new(rocket()).expect("Failed to create client instance");
    let mut response = client.post("/ping").dispatch();
    // tests the status
    assert_eq!(response.status(), Status::Ok);
}

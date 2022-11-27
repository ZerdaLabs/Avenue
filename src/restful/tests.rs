#[cfg(test)]
#[cfg(feature="restful")]
use super::*;


#[test]
fn client_get() {
    println!("I ran a test")
}


#[test]
fn client_post() {
    println!("I ran a test")
}


#[cfg(all(feature = "restful", feature = "client"))]
#[test]
fn new_client() {
    super::client::Client::new("httpbin");
}


#[cfg(all(feature = "http2", feature = "restful"))]
#[test]
fn server_push() {
    println!("I ran a test")
}
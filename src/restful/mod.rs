#[cfg(feature = "restful")]

mod tests;

//#[cfg(feature = "client")]
pub mod client {

    use http::{Request, StatusCode};
    use hyper::{client::conn::{self, Connection}, Body, Error};
    use tokio::net::TcpStream;



    pub trait Endpoint<T> {
        fn endpoint(params: T) -> Result<String, Error>;
    }

    pub struct Client {
        baseurl: String,
        headers: hyper::HeaderMap,
    }
    /*
    impl Client {
        pub fn new(url: &str) -> Result<Client, Error> {
            Client::build(url, Client::builder())
        }

        fn build(url: &str, builder: Builder) -> Result<Client, Error> {
            let client = match builder.client {
                Some(client) => client,
                None => {
                    Client::builder().build(HttpsConnector::new())
                }
            };
    
            let baseurl = Url::parse(url).map_err(|_| Error::UrlError)?;
    
            debug!("new client for {}", baseurl);
            Ok(RestClient {
                client,
                baseurl,
                auth: None,
                headers: HeaderMap::new(),
                timeout: builder.timeout,
                send_null_body: builder.send_null_body,
                body_wash_fn: std::convert::identity,
            })
        }
    }
     */






    pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let target_stream = TcpStream::connect("example.com:80").await?;
        let (mut request_sender, connection) = conn::handshake(target_stream).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error in connection: {}", e);
            }
        });
    let request = Request::builder()
        .header("Host", "example.com")
        .method("GET")
        .body(Body::from(""))?;
    println!("{:?}", request.uri());
    let response = request_sender.send_request(request).await?;
    println!("{:?}", response);
    assert!(response.status() == StatusCode::OK);
        Ok(())
    }



}






















#[cfg(feature = "server")]
pub mod server {
    use hyper::server;
}
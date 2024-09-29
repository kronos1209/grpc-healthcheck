use std::str::FromStr;

use grpc_server::MyService;
use proto_definition::proto::my_service::my_service_server::MyServiceServer;
use tonic::{server::NamedService, transport::channel};
use tonic_health::pb::HealthCheckRequest;

#[tokio::main]
async fn main() {
    let channel = channel::Endpoint::from_str("grpc://127.0.0.1:8080")
        .unwrap()
        .connect()
        .await
        .unwrap();

    let mut client = tonic_health::pb::health_client::HealthClient::new(channel);
    let res = client
        .check(HealthCheckRequest {
            service: MyServiceServer::<MyService>::NAME.to_string(),
        })
        .await
        .unwrap();
    println!("health check result: {:?}", res);
}

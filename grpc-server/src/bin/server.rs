use std::time::Duration;

use grpc_server::MyService;
use proto_definition::proto::my_service::my_service_server::MyServiceServer;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    env_logger::init();

    // ヘルスチェックのステータスマップに MyService を追加する
    // この時点では MyService は NotServing として登録する
    let (mut reporter, health_server) = tonic_health::server::health_reporter();
    reporter
        .set_not_serving::<MyServiceServer<MyService>>()
        .await;

    let my_service = MyService {};
    let my_server =
        proto_definition::proto::my_service::my_service_server::MyServiceServer::new(my_service);

    // 30秒後にサービスが利用可能になっているようにする
    let mut clone_reporter = reporter.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(30)).await;
        clone_reporter
            .set_serving::<MyServiceServer<MyService>>()
            .await;
    });

    // サーバーを起動する
    let addr = "0.0.0.0:8080".parse().unwrap();
    tonic::transport::Server::builder()
        .add_service(my_server)
        .add_service(health_server)
        .serve(addr)
        .await
        .unwrap();
}

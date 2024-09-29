pub struct MyService {}

#[tonic::async_trait]
impl proto_definition::proto::my_service::my_service_server::MyService for MyService {
    async fn get(
        &self,
        _request: tonic::Request<proto_definition::proto::my_service::MyRequest>,
    ) -> Result<tonic::Response<proto_definition::proto::my_service::MyResponse>, tonic::Status>
    {
        Ok(tonic::Response::new(
            proto_definition::proto::my_service::MyResponse {},
        ))
    }
}

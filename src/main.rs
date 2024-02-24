mod bus;

use bus::bus::Bus;
use proto::tulip_server::{Tulip, TulipServer};
use std::sync::Mutex;
use tonic::transport::Server;

mod proto {
  tonic::include_proto!("tulip");
}

#[derive(Debug, Default)]
struct TulipService {
  b: Mutex<Bus>,
}

#[tonic::async_trait]
impl Tulip for TulipService {
  async fn publish(
    &self,
    request: tonic::Request<proto::PublishRequest>,
  ) -> Result<tonic::Response<proto::PublishResponse>, tonic::Status> {
    println!("Publish request processed: {:?}", request);

    let input = request.get_ref();

    let bus: &mut Bus = &mut self.b.lock().unwrap();

    match bus.add_message(input.topic.to_string(), input.payload.to_string()) {
      Ok(response) => {
        let res = proto::PublishResponse {
          message: response,
          error: "".to_string(),
        };

        return Ok(tonic::Response::new(res));
      }
      Err(err) => return Err(tonic::Status::internal(err.to_string())),
    };
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;

  let tulip = TulipService::default();

  Server::builder()
    .add_service(TulipServer::new(tulip))
    .serve(addr)
    .await?;

  Ok(())
}

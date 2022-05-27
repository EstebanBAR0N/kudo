use tonic::{transport::Server, Request, Response, Status};

use node_agent::instance_service_server::{InstanceService, InstanceServiceServer};
use node_agent::{Instance, InstanceStatus, MaxResource, Type};

pub mod node_agent {
    tonic::include_proto!("node_agent");
}

#[derive(Debug, Default)]
pub struct ControllerInstanceService {}

#[tonic::async_trait]
impl InstanceService for ControllerInstanceService {
    type createStream = tonic::Streaming<InstanceStatus>;

    async fn create(
        &self,
        _request: Request<Instance>,
    ) -> Result<Response<Self::createStream>, Status> {
        println!("create");
        unimplemented!()
    }

    async fn start(&self, _request: Request<Instance>) -> Result<Response<()>, Status> {
        println!("start");
        unimplemented!()
    }

    async fn stop(&self, _request: Request<Instance>) -> Result<Response<()>, Status> {
        println!("stop");
        unimplemented!()
    }

    async fn destroy(&self, _request: Request<Instance>) -> Result<Response<()>, Status> {
        println!("destroy");
        unimplemented!()
    }

    async fn kill(&self, _request: Request<Instance>) -> Result<Response<()>, Status> {
        println!("kill");
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:56789".parse()?;
    let controller = ControllerInstanceService::default();

    Server::builder()
        .add_service(InstanceServiceServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())
}

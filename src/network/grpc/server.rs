


use super::root::common_server::{ Common, CommonServer };
use super::root::{ SimpleRequest, SimpleReply, StreamRequest, StreamReply };

use futures_core::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;

#[derive(Debug, Default)]
pub struct RootService<F>
where F: Fn(u32, Vec<u8>) -> Vec<u8> + std::marker::Sync + Send + 'static, 
{
    router: F,
}

#[tonic::async_trait]
impl<F> Common for RootService<F> 
where F: Fn(u32, Vec<u8>) -> Vec<u8> + std::marker::Sync + Send + 'static
{
    async fn simple(&self, request: Request<SimpleRequest>) -> Result<Response<SimpleReply>, Status> {
        
    
        Ok(Response::new(SimpleReply::default()))
    }

    type ServerStream = ReceiverStream<Result<StreamReply, Status>>;

    async fn server(
        &self,
        request: Request<SimpleRequest>,
    ) -> Result<Response<Self::ServerStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        
        tokio::spawn(async move {
            
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn client(
        &self,
        _request: Request<tonic::Streaming<StreamRequest>>,
    ) -> Result<Response<SimpleReply>, Status> {
        unimplemented!()
    }

    type BothStream = Pin<Box<dyn Stream<Item = Result<StreamReply, Status>> + Send  + 'static>>;

    async fn both(
        &self,
        _request: Request<tonic::Streaming<StreamRequest>>,
    ) -> Result<Response<Self::BothStream>, Status> {
        unimplemented!()
    }
}

async fn start<F>(router:F) -> Result<(), Box<dyn std::error::Error>> 
where F: Fn(u32, Vec<u8>) -> Vec<u8> + std::marker::Sync + Send + 'static
{
    let addr = "[::1]:10000".parse().unwrap();

    println!("CommonServer listening on: {}", addr);

    let service = RootService::<F> { router };

    let svc = CommonServer::new(service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
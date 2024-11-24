
// create a new load balancer.
// keep a list of all the nodes along with the range of prefix character starting for the trie residing in that node.
// Start: if there is just one node, then the range is from 'a' to 'z'


// this LB will serve the say_hello requests by figuring out which node should serve the request.
// it will then forward the request to the node and return the response.
// the node will then search the trie for the prefix and return the result.

// extension in future: add the logic of adding/removing nodes from the LB.
// there'll be an partitioning algorithm to figure out which node should serve which range of prefixes.
// for now, let's start with a single node serving all the prefixes.

use std::collections::HashMap;


use tonic::{transport::Server, Request, Response, Status};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use hello_world::greeter_client::GreeterClient;

pub mod hello_world {
    tonic::include_proto!("helloworld"); 
}

pub struct LoadBalancer {
    // For now just store single backend node address
    node_addr: String
}

impl LoadBalancer {
    pub fn new() -> Self {
        LoadBalancer {
            node_addr: "http://[::1]:50051".to_string()
        }
    }
}

#[tonic::async_trait]
impl Greeter for LoadBalancer {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Load balancer received request: {:?}", request);

        // Create client to backend node
        let mut client = GreeterClient::connect(self.node_addr.clone())
            .await
            .map_err(|e| Status::internal(format!("Failed to connect to backend: {}", e)))?;

        // Forward the request
        let response = client
            .say_hello(request)
            .await
            .map_err(|e| Status::internal(format!("Backend request failed: {}", e)))?;

        Ok(response)
    }
}

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50052".parse()?;
    let lb = LoadBalancer::new();

    println!("Load balancer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(lb))
        .serve(addr)
        .await?;

    Ok(())
}



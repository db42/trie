use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

mod indexer;

use indexer::Indexer;
// GRPC server implementation for prefix search service.
// 
// This module implements a GRPC server that provides prefix-based word search functionality
// using the trie data structure. The server supports multiple tenants, with each tenant
// having its own word dictionary and trie.
// 
// # Server Implementation
// 
// The server exposes a single GRPC endpoint `SayHello` that accepts:
// - A prefix string to search for
// - A tenant ID to determine which dictionary to search in
// 
// And returns:
// - A list of words from the tenant's dictionary that match the given prefix
// 
// # Example Usage
// 
// ```bash
// # Start the server
// cargo run --bin helloworld-server
// 
// # Make a GRPC request
// grpcurl -plaintext -import-path ./proto -proto helloworld.proto \
//   -d '{"name": "apr", "tenant": "thoughtspot"}' \
//   '[::1]:50051' helloworld.Greeter/SayHello
// ```
// 
// # Implementation Details
// 
// - Uses Tonic for GRPC server implementation
// - Maintains separate tries per tenant using the `Indexer` 
// - Loads word dictionaries from files at startup
// - Supports concurrent requests across tenants


pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct MyGreeter {
    indexer: Indexer
}

impl MyGreeter {
    fn new() -> Self {
        let mut indexer = Indexer::new();
        let tenant1 = "thoughtspot";
        let path1 = "./words.txt";
        indexer.indexFile(&tenant1, &path1);

        let tenant2 = "power";
        let path2 = "./words_alpha.txt";
        indexer.indexFile(&tenant2, &path2);


        let greeter = MyGreeter {
            indexer: indexer
        };

        greeter
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {

    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        let inner = request.into_inner();
        let word = inner.name;
        let tenant = inner.tenant;
        //search for this word in
        let matches = self.indexer.prefixMatch(&tenant, &word);

        let reply = HelloReply {
            message: format!("Hello {} matches: {:?}!", word, matches),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;
    // let greeter = MyGreeter::default();
    let greeter = MyGreeter::new();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
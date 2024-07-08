use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

mod indexer;

use indexer::Indexer;

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
        let path1 = "/Users/dushyant.bansal/work/rprojects/helloworld-tonic/words.txt";

        let tenant2 = "powerbi";
        let path2 = "/Users/dushyant.bansal/work/rprojects/helloworld-tonic/words_alpha.txt";
        indexer.indexFile(&tenant1, &path1);

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
        let word = request.into_inner().name;
        //search for this word in
        let matches = self.indexer.prefixMatch("thoughtspot", &word);

        let reply = HelloReply {
            message: format!("Hello {} matches: {:?}!", word, matches),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    // let greeter = MyGreeter::default();
    let greeter = MyGreeter::new();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
## How to run
```
rustc trie.rs && ./trie
```

### GRPC
`cd helloworld-tonic`

**server**
```
cargo run --bin helloworld-server
```

**client**
```
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "apr"}' '[::1]:50051' helloworld.Greeter/SayHello
```

Example response
```
{
  "message": "Hello apr matches: [\"april\"]!"
}
```



## Todo
1. run this for a large dataset of english words
2. run as a separate service and call using grpc
3. distributed trie
4. Return different prefix and exact matches
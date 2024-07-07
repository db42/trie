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
0. read dataset from file - Done
1. run this for a large dataset of english words - 370K - Done
2. run as a separate service and call using grpc - DONE
3. Return different prefix and exact matches // test with 'whiplash' - DONE
4. distributed trie
5. UTs, ITs
6. debugging using breakpoint
7. deployment using K8S
8. [Long shot] ubr kind of ranking
9. How does ES compare to this?

## details

**Memory**
calculation
10 bytes * 370K = 3.7MB

26 items in array for each character

26*4 MB = 100 MB

Actual memory usage = 700 MB

## Resources
1. https://github.com/kiyoka/distributed-trie
2. https://medium.com/@prefixyteam/how-we-built-prefixy-a-scalable-prefix-search-service-for-powering-autocomplete-c20f98e2eff1 
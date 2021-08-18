# rust-rocket-micro-service

This project started as a proof of concept to determine how a Rust [Rocket crate] micro-service that retrieves persisted data from a Cassandra database could perform under heavy load (many concurrent requests for a long time).

However I ended up adding "almost" all unit tests. I wrote "almost" because main.rs is not covered, the idea is to cover it by using integration tests, and then add also some performance tests.

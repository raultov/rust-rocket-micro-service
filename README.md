# rust-rocket-micro-service
This project started as a proof of concept to determine how a Rust [Rocket crate] micro-service that retrieves persisted data from a Cassandra database could perform under heavy load (many concurrent requests for a long time).

However, I also ended up adding "almost" all unit tests. I wrote "almost" because `main.rs` is not covered by unit tests, actually it is covered by integration tests.

## 'Dockerized' app
There is a `Dockerfile` that will build a docker image containing our app. Next you can see the steps followed to achieve that:
* Testing
1. Pull a Cassandra docker image to be used as build base image
2. Install Rustup
3. Run unit-tests (cargo tests)
4. Build the app (cargo build)
5. Run integration tests (`launch-it-tests.sh`)
* Build release app
1. Pull an ubuntu docker image to be used as release base image
2. Expose port 8000
3. Push docker image

### Integration tests
Integration tests consist of a shell script `launch-it-tests.sh` which execute the next steps:
1. Start Cassandra
2. Setup Vehicle data
3. Build and run rust app
4. Run tests located in the `tests` folder

### Performance tests
Regarding `2-get-vehicle-performance.sh` and `4-create-vehicle-performance.sh` which are executed as Integration Tests, it is important to clarify that the threshold requests/second is calculated based on my machine, that is, it is really coupled to a specific hardware, in other words most likely this threshold is not suitable for other machine/hardware. I would even recommend to not take that test as a reliable performance test as it relies on Apache ab which is measuring how app performs under a specific load for 30 seconds. Reliable tests should take into account additional parameters, loads and situations.

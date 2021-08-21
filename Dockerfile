#############
### build ###
#############

# base image
FROM cassandra:3.11.11 as build

# install rustc
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH
RUN rustup update
RUN apt-get update && apt-get install -yq build-essential jq

# set working directory
WORKDIR /app

# add app
COPY . /app

# run unit tests
RUN cargo test

# run integration tests
RUN bash IT/launch-it-tests.sh

###############
### release ###
###############

# base image
FROM ubuntu:latest

# copy artifact build from the 'build environment'
COPY --from=build /app/target/release/rust_rocket_micro_service /rust_rocket_micro_service

# expose port 8000
EXPOSE 8000

# set CASSANDRA_NODE variable
ENV CASSANDRA_NODE 192.168.1.148:9042

# run app
CMD ["/rust_rocket_micro_service"]

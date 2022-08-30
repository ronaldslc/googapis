FROM gr8ninja/rust
RUN apt-get update -y &&\
    apt-get install -y protobuf-compiler &&\
    apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

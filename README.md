# CartPole-v1 inference in Rust

## Requirements
- Install Libtorch v1.13.0
- Env variables that need to be present:
```
export LIBTORCH=~/pytorch-install
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
export LIBTORCH_INCLUDE=~/pytorch-install
export LIBTORCH_LIB=~/pytorch-install
```

## Getting Started

logs are better seen with [bunyan](https://github.com/LukeMathWalker/bunyan), and the log level can be controlled by setting RUST_LOG (default is info).

### Run server
To run the server, please use ```cargo run --bin rl-proto | bunyan```

### Load test
To run the load test, make sure server is already running and please use ```cargo run --bin load -- --host http://127.0.0.1:8080```

### Run integration/unit test
To run the tests, please use ```RUST_ENV=test cargo test | bunyan```

## API

- inference request:
```
curl -XPOST http://127.0.0.1:8080/predict \
  -H "Content-Type: application/json" \
  -d '{ "cart_position": 0.1, "cart_velocity": 50.0, "pole_angle": 0.13, "pole_angular_velocity": 0.1}'
```

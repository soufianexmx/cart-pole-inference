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

<img width="1660" alt="Screenshot 2023-02-07 at 17 12 46" src="https://user-images.githubusercontent.com/123186384/217300175-ca96a343-b6f5-418f-b6fd-3feedeb02bb0.png">

### Load test
For load tests, [Goose](https://docs.rs/goose/latest/goose/). To fire the load, make sure server is already running and please use ```cargo run --bin load -- --host http://127.0.0.1:8080```. You can stop the load with ctrl + c, the output will log some statistics about the average response time, number of requests, number of failing requests; all for the predict endpoint.

https://user-images.githubusercontent.com/123186384/217301060-a7de0c45-b53e-4bb0-b9d4-34e633035984.mov

### Run integration/unit test
To run the tests, please use ```RUN_ENV=test cargo test | bunyan```. By declaring RUST_ENV=test, you will use the test config file and with that a different port for each integration test.

## API

- inference request:
```
curl -XPOST http://127.0.0.1:8080/predict \
  -H "Content-Type: application/json" \
  -d '{ "cart_position": 0.1, "cart_velocity": 50.0, "pole_angle": 0.13, "pole_angular_velocity": 0.1}'
```
The response should be in the json form ```{"action":"Right"}```.

## Metrics

To be able to collect and visualize metrics please use ```docker compose up```. Grafana can be accessed from http://127.0.0.1:3000. 

<img width="1167" alt="image" src="https://user-images.githubusercontent.com/123186384/217311400-d4f88554-14ae-48de-9801-f888043d75e8.png">


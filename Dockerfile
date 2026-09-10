FROM rust:1.77 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    ros-humble-ros-base \
    python3-colcon-common-extensions \
    cmake \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ros2-project-gen /usr/local/bin/

ENTRYPOINT ["ros2-project-gen"]

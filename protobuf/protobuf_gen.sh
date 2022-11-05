#!/bin/bash

cd "$(dirname $0)"
cd ".."
cargo run --bin protoGen
protoc -I=.  --python_out=.  protobuf/samples.proto
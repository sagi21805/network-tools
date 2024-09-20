#! /bin/bash

project_name="network-tools"

cargo build --release
mv target/release/$project_name ./$project_name
sudo ./$project_name
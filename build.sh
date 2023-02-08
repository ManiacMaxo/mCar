#!/bin/bash

NUM_CPUS=${NUM_CPUS:-$(sysctl -n hw.ncpu)}
export CROSS_BUILD_OPTS="--build-arg NUM_CPUS=${NUM_CPUS}"
cross build

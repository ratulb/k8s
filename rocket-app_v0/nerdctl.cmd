#!/bin/bash

sudo nerdctl run --rm -v $(pwd):/workspace -v ~/.docker/config.json:/workspace/config.json \
--env DOCKER_CONFIG=/workspace gcr.io/kaniko-project/executor -d ratulb/rocket-app_v0:latest

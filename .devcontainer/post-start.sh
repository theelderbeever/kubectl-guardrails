#!/bin/bash

echo "$(date) - post-start start" >> ~/status

# this runs in background each time the container starts

# Configuring default guardrails
echo "$(date) - post-create config default guardrails" >> ~/status
mkdir -p $HOME/.kube
cp .devcontainer/guardrails.yaml $HOME/.kube/guardrails

# Building guardrails binary from source
echo "$(date) - post-create building kubectl-guardrails binary from source" >> ~/status
cargo build

echo "$(date) - post-start complete" >> ~/status
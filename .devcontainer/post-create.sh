#!/bin/bash

echo "$(date) - post-create start" >> ~/status

# this runs in background after UI is available
if [ -f "/workspaces/.codespaces/shared/first-run-notice.txt" ]; then
    cp .devcontainer/first-run-notice.txt /workspaces/.codespaces/shared/first-run-notice.txt
else
    echo "$(date) - post-create First-run notice file does not exist."
fi

# Setting up alias and PATH
echo "$(date) - post-create setting up alias and PATH" >> ~/status
cat .env.local >> ~/.zshrc
source ~/.zshrc

# Set Docker host to the correct mounted Docker socket
export DOCKER_HOST=unix:///var/run/docker-host.sock

# Creating local kind cluster
echo "$(date) - post-create creating local kind cluster" >> ~/status
kind delete cluster -n demo-cluster
kind create cluster --config .devcontainer/kind-cluster.yaml --wait 20s >> ~/status 2>&1

# Deploying sample pod
echo "$(date) - post-create deploying sample pod" >> ~/status
kubectl apply -f .devcontainer/example-pod.yaml

echo "$(date) - post-create complete" >> ~/status
#!/bin/bash
# Copyright 2020-2021 The Bigbytesdb Authors.
# SPDX-License-Identifier: Apache-2.0.

set -ex

BUILD_PROFILE=${BUILD_PROFILE:-debug}

curl -s https://raw.githubusercontent.com/k3d-io/k3d/main/install.sh | TAG=v5.6.0 bash
k3d registry create registry.localhost --port 0.0.0.0:5111 -i registry:latest
k3d cluster create --config ./scripts/ci/meta-chaos/k3d.yaml meta-chaos

echo "127.0.0.1 k3d-registry.localhost" | sudo tee -a /etc/hosts

if kubectl version --client; then
	echo "kubectl client already installed"
else
	echo "install kubectl client"
	curl -LO "https://dl.k8s.io/release/v1.29.5/bin/linux/amd64/kubectl"
	chmod +x kubectl
	sudo mv kubectl /usr/local/bin/
fi

if helm version; then
	echo "helm already installed"
else
	echo "install helm"
	curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3
	chmod 700 get_helm.sh
	./get_helm.sh
fi

echo "make bigbytesdb-meta image"
ls -lh ./target/"${BUILD_PROFILE}"
mkdir -p temp/distro/amd64
cp ./target/"${BUILD_PROFILE}"/bigbytesdb-meta ./temp/distro/amd64
cp ./target/"${BUILD_PROFILE}"/bigbytesdb-metactl ./temp/distro/amd64
cp tests/metaverifier/cat-logs.sh ./temp/distro/amd64
docker build -t bigbytesdb-meta:meta-chaos --build-arg TARGETPLATFORM="amd64" -f ./docker/meta-chaos/meta.Dockerfile temp
docker tag bigbytesdb-meta:meta-chaos k3d-registry.localhost:5111/bigbytesdb-meta:meta-chaos
docker push k3d-registry.localhost:5111/bigbytesdb-meta:meta-chaos

echo "make bigbytesdb-metaverifier image"
rm -rf temp/distro/amd64/*
cp ./target/"${BUILD_PROFILE}"/bigbytesdb-metaverifier ./temp/distro/amd64
cp tests/metaverifier/start-verifier.sh ./temp/distro/amd64
cp tests/metaverifier/cat-logs.sh ./temp/distro/amd64
docker build -t bigbytesdb-metaverifier:meta-chaos --build-arg TARGETPLATFORM="amd64" -f ./docker/meta-chaos/verifier.Dockerfile temp
docker tag bigbytesdb-metaverifier:meta-chaos k3d-registry.localhost:5111/bigbytesdb-metaverifier:meta-chaos
docker push k3d-registry.localhost:5111/bigbytesdb-metaverifier:meta-chaos

echo "install chaos mesh on k3d"
curl -sSL https://mirrors.chaos-mesh.org/v2.6.3/install.sh | bash -s -- --k3s

kubectl get pods -A -o wide
kubectl get pvc -A

echo "kubectl delete bigbytesdb-meta pvc"
kubectl delete pvc --namespace bigbytesdb data-test-bigbytesdb-meta-0 data-test-bigbytesdb-meta-1 data-test-bigbytesdb-meta-2 --ignore-not-found

helm repo add bigbytesdb https://charts.bigbytesdb.com
helm install test bigbytesdb/bigbytesdb-meta \
	--namespace bigbytesdb \
	--create-namespace \
	--values scripts/ci/meta-chaos/meta-ha.yaml \
	--set image.repository=k3d-registry.localhost:5111/bigbytesdb-meta \
	--set image.tag=meta-chaos \
	--wait || true

sleep 10
echo "check if bigbytesdb-meta nodes is ready"
kubectl -n bigbytesdb wait \
	--for=condition=ready pod \
	-l app.kubernetes.io/name=bigbytesdb-meta \
	--timeout 120s || true

kubectl get pods -A -o wide

kubectl -n bigbytesdb exec test-bigbytesdb-meta-0 -- /bigbytesdb-metactl status

echo "create verifier pod.."
kubectl apply -f scripts/ci/meta-chaos/verifier.yaml

echo "check if bigbytesdb-metaverifier node is ready"
kubectl -n bigbytesdb wait \
	--for=condition=ready pod \
	-l app.kubernetes.io/name=bigbytesdb-metaverifier \
	--timeout 120s || true

echo "logs bigbytesdb-metaverifier.."
kubectl logs bigbytesdb-metaverifier --namespace bigbytesdb

kubectl get pods -n bigbytesdb -o wide

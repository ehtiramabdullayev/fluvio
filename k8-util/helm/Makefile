VERSION := $(shell cat ../../VERSION)
GIT_VERSION=$(shell git log -1 --pretty=format:"%H")
REGISTRY=infinyon
CLOUD=minikube
SC_LOG=info
IMAGE_POLICY=IfNotPresent
NAME=

dry-run:
	helm install --debug --dry-run goodly-guppy ./fluvio --set fluvioVersion=$(GIT_VERSION)

dry-run-aws:
	helm install --debug --dry-run goodly-guppy ./fluvio --set cloud=aws

install:
	helm install $(NAME) ./fluvio \
		--set fluvioVersion=$(GIT_VERSION) \
		--set registry=$(REGISTRY)	\
		--set cloud=$(CLOUD) \
		--set scLog=$(SC_LOG) \
		--set imagePolicy=$(IMAGE_POLICY)

test-install:	SC_LOG="kf\=trace\,sc\=trace\,flv\=trace\,k8\=trace"
test-install:	IMAGE_POLICY=Always
test-install:	install
test-install:	NAME=test

install_minikube:	REGISTRY=localhost:5000/infinyon
install_minikube:	test-install

install_aws:	test-install
install_aws:	CLOUD=aws
install_aws:	REGISTRY=$(AWS_ECR)


uninstall-test:
	helm uninstall test
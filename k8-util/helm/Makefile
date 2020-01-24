VERSION := $(shell cat ../../VERSION)
GIT_VERSION=$(shell git log -1 --pretty=format:"%H")
REGISTRY=infinyon
CLOUD=minikube
SC_LOG=info
IMAGE_POLICY=IfNotPresent
NAME=fluvio
NS=default

dry-run:
	helm install --debug --dry-run goodly-guppy ./fluvio --set fluvioVersion=$(GIT_VERSION)

dry-run-aws:
	helm install --debug --dry-run goodly-guppy ./fluvio --set cloud=aws

install:
	helm install $(NAME) ./fluvio -n $(NS) \
		--set fluvioVersion=$(GIT_VERSION) \
		--set registry=$(REGISTRY)	\
		--set cloud=$(CLOUD) \
		--set scLog=$(SC_LOG) \
		--set imagePolicy=$(IMAGE_POLICY)

test-install:	SC_LOG="kf\=debug\,sc\=debug\,flv\=debug\,k8\=debug"
test-install:	IMAGE_POLICY=Always
test-install:	install
test-install:	NAME=fluviotest
test-install:	NS=default

install_minikube:	REGISTRY=localhost:5000/infinyon
install_minikube:	test-install

test_aws_install:	CLOUD=aws
test_aws_install:	REGISTRY=$(AWS_ECR)
test_aws_install:	test-install


test_aws_uninstall:	uninstall
test_aws_uninstall:	NS=test

uninstall:
	helm uninstall $(NAME) -n $(NS)
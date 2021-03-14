IMAGE_NAME := fast-ip
IMAGE_TAG := $$(tomlq -r '.package.version' Cargo.toml)
IMAGE := ${IMAGE_NAME}:${IMAGE_TAG}

clean:
	podman image rm localhost/${IMAGE}

build:
	buildah bud --tag ${IMAGE}

run:
	podman run --cidfile=.cid --rm -dp 3000:3000/tcp localhost/${IMAGE}

stop:
	-podman stop --cidfile=.cid
	-rm .cid

default: build run

.DEFAULT_GOAL = default

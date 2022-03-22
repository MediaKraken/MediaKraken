NAME = abhas/apt-mirror

all: build

build:
	docker build -t $(NAME):latest --rm .

armhf:
	docker build -f Dockerfile.armhf -t abhas/apt-mirror:armhf --rm .

push:
	docker push $(NAME)

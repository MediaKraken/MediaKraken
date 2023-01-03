## Build a Debian / Ubuntu / Raspbian mirror for local / offline use.

This container enables you to build a local mirror for Debian, Ubuntu and Raspbian.

## Building the container

Just run `make` - that will build the `abhas/apt-mirror` Docker image.

## Running the container

First, create a directory that will be used to store your mirror contents. And then pass that to the Docker run command line as follows:

```
mkdir /tmp/apt-mirror
docker run -ti --rm --name apt-mirror -v /tmp/apt-mirror:/var/spool/apt-mirror abhas/apt-mirror
```


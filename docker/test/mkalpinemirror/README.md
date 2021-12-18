# Alpine Linux Package Mirror - Docker Image

A Dockerfile which builds an alpine linux package mirror server. It may also be used to host a private package repo for packages that aren't part of the core distribution.

Based on [How_to_setup_a_Alpine_Linux_mirror][]

To configure alpine to use your new package mirror see [Alpine_Linux_package_management][]

## disk usage

Make sure that you have enough disk space; each v3.x branch has around 20 GiB.

Current (20170510) disk usage:

|  edge |  v2.4 |  v2.5 |  v2.6 |  v2.7 |  v3.0 |  v3.1 |  v3.2 |  v3.3 |  v3.4 |  v3.5 |  v3.6 | Total  |
|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|--------|
| 53.1G | 18.8G | 10.4G | 13.0G | 16.5G | 16.5G | 17.5G | 14.5G | 19.0G | 23.2G | 32.5G | 34.4G | 269.5G |


## custom download branch

you can disable download what do you want branch or cpu version

/etc/rsync/exclude.txt
```
edge/
v2.*/
v3.0/
v3.1/
v3.2/
v3.3/
v3.4/
v3.5/
aarch64/
armhf/
ppc64le/
s390x/
x86/
```

edge+v2.*+v3.x is alpine version,aarch64,armhf,ppc64le,s390x,x86,x86_64 is cpu architecture.

e.g. v3.6 + x86_64  only 11 GB

run this code to check your cpu architecture.

```bash
lscpu | grep Architecture
```
output like 
```
lscpu | grep Architecture
Architecture:          x86_64
```

## cron jobs

Dockerfile
```
# ...
COPY     ./conf/rsync.sh /etc/periodic/hourly/package-rsync
#...
```

it will sync every hour. (given cron runs). you can change it to daily , weekly or monthly.

## docker hub 

[![Automated build](https://img.shields.io/docker/automated/anjia0532/alpine-package-mirror.svg)](https://hub.docker.com/r/anjia0532/alpine-package-mirror/) [![Docker Pulls](https://img.shields.io/docker/pulls/anjia0532/alpine-package-mirror.svg)](https://hub.docker.com/v2/repositories/anjia0532/alpine-package-mirror/)


```bash
docker pull anjia0532/alpine-package-mirror
```

[How_to_setup_a_Alpine_Linux_mirror]: http://wiki.alpinelinux.org/wiki/How_to_setup_a_Alpine_Linux_mirror
[Alpine_Linux_package_management]: http://wiki.alpinelinux.org/wiki/Alpine_Linux_package_management

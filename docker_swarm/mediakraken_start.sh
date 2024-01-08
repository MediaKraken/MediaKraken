#!/bin/sh
sysctl -q -w vm.max_map_count=262144
docker-compose config | docker stack deploy --compose-file - mkstack

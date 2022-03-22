#!/bin/sh
crond -b -d 6 &
nginx -g'daemon off;'
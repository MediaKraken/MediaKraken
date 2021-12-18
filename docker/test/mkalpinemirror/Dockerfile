FROM nginx:alpine
MAINTAINER  AnJia <anjia0532@gmail.com>

COPY     ./conf/rsync.sh /etc/periodic/hourly/package-rsync
COPY     ./conf/exclude.txt /etc/rsync/exclude.txt
COPY     ./conf/nginx.conf /etc/nginx/nginx.conf
COPY     entrypoint.sh /entrypoint.sh

# default date zone is GMT+8
RUN     \
        apk update && apk --no-cache add rsync  tzdata && \
        cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && \
        echo "Asia/Shanghai" >  /etc/timezone && \
        rm -rf /var/cache/apk/* && \
                
        rm -rf /usr/share/nginx/html && \
        mkdir -p /usr/share/nginx/html

VOLUME  /usr/share/nginx/html

ENTRYPOINT ["/entrypoint.sh"]

EXPOSE 80 
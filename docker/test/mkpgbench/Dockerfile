FROM alpine:3.21.3 as builder

RUN apk add --no-cache postgresql17==17.4-r0

FROM alpine:3.21.3

RUN apk add --no-cache libpq==17.4-r0
COPY --from=builder /usr/bin/pgbench /usr/bin/pgbench

ENTRYPOINT ["pgbench"]
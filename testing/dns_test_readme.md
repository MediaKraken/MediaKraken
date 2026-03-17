kubectl run -it --rm --restart=Never test-dns --image=busybox:1.28 -- nslookup elasticsearch-es-http.elastic-stack

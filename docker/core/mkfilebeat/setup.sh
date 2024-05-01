docker run \
elastic/filebeat:7.17.20 \
setup -E setup.kibana.host=mkstack_elk:5601 \
-E output.elasticsearch.hosts=["mkstack_elk:9200"] 
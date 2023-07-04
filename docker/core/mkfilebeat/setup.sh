docker run \
elastic/filebeat:7.17.10 \
setup -E setup.kibana.host=mkelk.beaverbay.local:5601 \
-E output.elasticsearch.hosts=["mkelk.beaverbay.local:9200"] 
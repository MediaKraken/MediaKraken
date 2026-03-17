

##############
run the cloudnativepg to create cluster and db

sign into pgadmin to verify db exists
pgcluster-with-metrics-rw.cnpg-system

helm package mkstack-mediakraken

helm install --create-namespace --namespace mediakraken mkstack-mediakraken https://github.com/MediaKraken/MediaKraken/raw/dev/k8s/mkstack-mediakraken-0.1.13.tgz --debug

kubectl create secret generic run-env-type --from-literal=debug=true -n mediakraken

setup the db-password
kubectl create secret generic db-password --from-literal=username=metaman --from-literal=password='metaman' -n mediakraken


helm uninstall mkstack-mediakraken -n mediakraken


#expanding disk
sudo vgextend /dev/mapper/vgk8s2 /dev/sdd && \
sudo lvextend -l +100%FREE /dev/mapper/vgk8s2-vgk8s2lv && \
sudo resize2fs /dev/mapper/vgk8s2-vgk8s2lv







##############
run the cloudnativepg to create cluster and db

sign into pgadmin to verify db exists
pgcluster-with-metrics-rw.cnpg-system

setup the db-password
kubectl create secret generic db-password --from-literal=username=metaman --from-literal=password='metaman' -n mediakraken

echo -n 'metam/root/MediaKraken/k8s/environment/playbooksan' | base64  # Outputs: bWV0YW1hbg==
echo -n 'postgres' | base64  # Outputs: cG9zdGdyZXM=
echo -n 'metamansuper' | base64  # Outputs: bWV0YW1hbnN1cGVy

helm package mkstack-mediakraken

helm install --create-namespace --namespace mediakraken mkstack-mediakraken https://github.com/MediaKraken/MediaKraken/raw/dev/k8s/mkstack-mediakraken-0.1.9.tgz --debug

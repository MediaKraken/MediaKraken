






##############
helm package mkstack-mediakraken

helm install --create-namespace --namespace mediakraken mkstack-mediakraken https://github.com/MediaKraken/MediaKraken/raw/dev/k8s/mkstack-mediakraken-0.1.8.tgz

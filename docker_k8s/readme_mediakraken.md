# checkout dev branch of mediakraken
# do the storage setup/claim/etc on master
sudo apt-get install git
git clone https://github.com/MediaKraken/MediaKraken
cd MediaKraken && git checkout dev
cd docker_k8s
kubectl apply -f nfs-pvc.yaml
kubectl apply -f nfs-pvc-8k.yaml
kubectl apply -f portainer-manifest.yaml

# install all secrets
python3 mediakraken_setup.py

# install all namespaces
kubectl apply -f namespace

# install all claims
kubectl apply -f claim

# install all services
kubectl apply -f service

# install all deployments
kubectl apply -f deployment









kubectl get pods --namespace portainer
kubectl logs mkstack-portainer-5f887dc8b4-vzksq --namespace portainer
kubectl logs mkstack-portainer-5f887dc8b4-vzksq --all-containers --namespace portainer
# https://raw.githubusercontent.com/portainer/k8s/master/deploy/manifests/portainer/portainer.yaml


/*
kubectl get persistentvolume --namespace mediakraken
kubectl get pvc --namespace=mediakraken

kubectl delete persistentvolumeclaim mkstack-database-claim  --namespace mediakraken

kubectl get pods --namespace=mediakraken

kubectl rollout restart deployment --namespace=mediakraken

kubectl rollout restart deployment mkstack-portainer -n mediakraken

kubectl logs mkstack-grafana-78669db887-5d7tf --all-containers=true

# secrets
# kubectl create secret generic db-password --from-literal=username=devuser --from-literal=password='S!B\*d$zDsb='

kubectl create secret generic db-password --from-literal=username=devuser --from-literal=password='S!B\*d$zDsb='

kubectl get secrets --namespace=mediakraken

kubectl get deployment --namespace=mediakraken

kubectl delete deployment --all --namespace=mediakraken

kubectl get service --all-namespaces

# see if rbac enabled
kubectl api-versions | grep rbac
*/


/*
# upgrade of cluster...master nodes
kubectl drain --ignore-daemonsets mkcube1.beaverbay.local

echo "deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.30/deb/ /" | sudo tee /etc/apt/sources.list.d/kubernetes.list
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.30/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg

sudo apt update

sudo apt-mark unhold kubeadm && \
sudo apt-get update && sudo apt-get install -y kubeadm='1.30.2-*' && \
sudo apt-mark hold kubeadm
*/
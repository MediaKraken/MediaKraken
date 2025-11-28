# storage
zfs create wdblack/sharenfs
zfs set atime=off wdblack/sharenfs

# experiment
zfs create wdblack/sharenfs8k
zfs set atime=off wdblack/sharenfs8k
zfs set recordsize=8K wdblack/sharenfs8k
zfs set primarycache=metadata wdblack/sharenfs8k
zfs set logbias=throughput wdblack/sharenfs8k

<!-- zfs set sharenfs=on wdblack/sharenfs
# zfs set sharenfs="on,rw=@192.168.1.0/24" wdblack/sharenfs
zfs get sharenfs
zfs share -a

zfs set sharenfs=off pool54_62/batocera
zfs set sharenfs=off wdblack/sharenfs
zfs unshare -a
systemctl restart nfs-kernel-server -->

chown -R nobody:nogroup /wdblack/sharenfs && chmod -R 777 /wdblack/sharenfs/

chown -R nobody:nogroup /wdblack/sharenfs8k && chmod -R 777 /wdblack/sharenfs8k/

apt install nfs-kernel-server rpcbind
nano /etc/exports
/wdblack/sharenfs 192.168.1.0/24(rw,sync,no_subtree_check,no_root_squash)
/wdblack/sharenfs8k 192.168.1.0/24(rw,sync,no_subtree_check,no_root_squash)
exportfs -a
exportfs -r
exportfs -v



# checkout dev branch of mediakraken
# do the storage setup/claim/etc on master
sudo apt-get install git
git clone https://github.com/MediaKraken/MediaKraken
kubectl apply -f nfs-pvc.yaml
kubectl apply -f nfs-pvc-8k.yaml
kubectl get storageclasses
kubectl describe storageclasses nfs-csi

kubectl get storageclass

# helm stuff to play with
https://artifacthub.io/packages/helm/cluster-autoscaler/cluster-autoscaler
https://artifacthub.io/packages/helm/cadvisor/cadvisor    container monitor
https://artifacthub.io/packages/helm/utkuozdemir/transmission-exporter

https://artifacthub.io/packages/helm/radar-base/cert-manager-letsencrypt
https://artifacthub.io/packages/helm/certs/certs

https://github.com/killemov/Shift    transmission web ui
instead of docker registry, look at https://goharbor.io/

/*
docker run --name semaphore \
-p 3000:3000 \
-e SEMAPHORE_DB_DIALECT=bolt \
-e SEMAPHORE_ADMIN=admin \
-e SEMAPHORE_ADMIN_PASSWORD=changeme \
-e SEMAPHORE_ADMIN_NAME="Admin" \
-e SEMAPHORE_ADMIN_EMAIL=admin@localhost \
-v semaphore_data:/var/lib/semaphore \
-d semaphoreui/semaphore:v2.10.20
*/

# installed all claims
kubectl apply -f claim

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

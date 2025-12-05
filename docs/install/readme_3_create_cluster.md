# pull down repository in home directory
```
git clone https://github.com/MediaKraken/MediaKraken
```

# create controller and worker nodes in proxmox
```
TODO setup variables
cd ~/MediaKraken/k8s/proxmox
tofu init
tofu plan
tofu apply
```

# use kubespray/ansible/helm to build out k8s cluster 
```
cd ~/MediaKraken/k8s/cluster
TODO edit the aws route password/keys
tofu init
tofu plan
tofu apply
some things might fail....wait a bit for parts to spin up and rerun 'tofu apply'
```

# get token for k8sdashboard
```
ssh metaman@192.168.50.50
kubectl get secret admin-user -n kubernetes-dashboard -o jsonpath={".data.token"} | base64 -d
```

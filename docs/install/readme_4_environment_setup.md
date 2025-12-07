# run tofu from environment directory to build k8s operators/etc and mediakraken itself
```
cd ~/MediaKraken/k8s/environment
tofu init
tofu plan
tofu apply
some things might fail....wait a bit for parts to spin up and rerun 'tofu apply'
```

# longhorn security on control node
USER=metaman; PASSWORD=metaman; echo "${USER}:$(openssl passwd -stdin -apr1 <<< ${PASSWORD})" >> auth
kubectl -n longhorn-system create secret generic basic-auth --from-file=auth

# longhorn storage setup
remove schedule from preexiting drives
add disks to each nodes
    call em metaman1, 2, 3
    do NOT set reserved space
/mnt/volume/disk2   is the one for images/etc

volume, create volume
volume 1900gb metadata
attach to host.....worker1
now you have a shiney replicating volume

/dev/mapper/vgk8s1-vgk8s1lv 512gb
/dev/mapper/vgk8s2-vgk8s2lv  2tb


#############################################

# setup graphana dash for rabbitmq
import 10991

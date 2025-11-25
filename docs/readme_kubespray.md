# checkout the devluster_deploy readme for venv and kubespray update

cp ~/bmoenv/opentofu/cluster/hosts.yml ~/kubespray/inventory/bmocluster/hosts.yml
ansible-playbook upgrade-cluster.yml -b -i inventory/bmocluster/hosts.yml -e kube_version=1.33.1

# took 1 hour 8 minutes

metaman@mkcontrol1:~$ kubectl get node
NAME         STATUS   ROLES           AGE   VERSION
mkcontrol1   Ready    control-plane   7d    v1.34.2
mkcontrol2   Ready    control-plane   7d    v1.34.2
mkcontrol3   Ready    control-plane   7d    v1.34.2
mkworker1    Ready    <none>          7d    v1.34.2
mkworker2    Ready    <none>          7d    v1.34.2
mkworker3    Ready    <none>          7d    v1.34.2
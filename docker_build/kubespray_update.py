import os

# remove old files
os.system("rm -rf ../../kubespray/inventory/mkcluster")
os.system("rm -rf ../../kubespray/inventory/mkclusterdev")

# copy kubespray inventory to mk dirs
os.system("cp -rf ../../kubespray/inventory/sample ../../kubespray/inventory/mkcluster")
os.system("cp -rf ../../kubespray/inventory/sample ../../kubespray/inventory/mkclusterdev")

# SED the cluster names
os.system("sed -i -e 's/cluster_name: cluster.local/cluster_name: mkcluster.local/g' ../../kubespray/inventory/mkcluster/group_vars/k8s_cluster/k8s-cluster.yml")
os.system("sed -i -e 's/cluster_name: cluster.local/cluster_name: mkclusterdev.local/g' ../../kubespray/inventory/mkclusterdev/group_vars/k8s_cluster/k8s-cluster.yml")

# copy the inventory to mk dirs
os.system("cp -f ../k8s/cluster/inventory.ini ../../kubespray/inventory/mkcluster/.")
os.system("cp -f ../k8s_devenv/cluster/inventory.ini ../../kubespray/inventory/mkclusterdev/.")

# install/upgrade requirements
os.system("pip3 install -r ../../kubespray/requirements.txt --break-system-packages")
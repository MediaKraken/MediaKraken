resource "terraform_data" "kubespray" {
  # create the cluster via kubespray
  provisioner "local-exec" {
    command     = "ansible-playbook -b -v -u metaman -i inventory/mkclusterdev/inventory.ini cluster.yml --ssh-common-args='-o StrictHostKeyChecking=accept-new'"
    working_dir = "../kubespray"
  }

  # setup the kube config
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u metaman -i kubespray/mkclusterdev/inventory.ini playbooks/kube.yml --ask-sudo-pass"
  }

  # setup helm
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u metaman -i kubespray/mkclusterdev/inventory.ini playbooks/helm.yml"
  }

  # setup operator for cluster
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u metaman -i kubespray/mkclusterdev/inventory.ini playbooks/operator.yml"
  }

  # setup the NFS layer
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u metaman -i kubespray/mkclusterdev/inventory.ini playbooks/nfs.yml --ask-sudo-pass"
  }
}

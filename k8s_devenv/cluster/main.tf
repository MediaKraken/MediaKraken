resource "terraform_data" "kubespray" {
  # create the cluster via kubespray
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory/mkclusterdev/inventory.ini cluster.yml --ssh-common-args='-o StrictHostKeyChecking=accept-new'"
    working_dir = "../../../kubespray"
  }
}

resource "terraform_data" "kubeconfig" {
  # setup the kube config
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i kubespray/mkclusterdev/inventory.ini playbooks/kube.yml"
  }
  depends_on = [
    terraform_data.kubespray
  ]
}

resource "terraform_data" "helm" {
  # setup helm
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i kubespray/mkclusterdev/inventory.ini playbooks/helm.yml"
  }
  depends_on = [
    terraform_data.kubeconfig
  ]
}

resource "terraform_data" "operator" {
  # setup operator for cluster
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i kubespray/mkclusterdev/inventory.ini playbooks/operator.yml"
  }
  depends_on = [
    terraform_data.helm
  ]
}

resource "terraform_data" "nfs" {
  # setup the NFS layer
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i kubespray/mkclusterdev/inventory.ini playbooks/nfs.yml"
  }
  depends_on = [
    terraform_data.operator
  ]
}

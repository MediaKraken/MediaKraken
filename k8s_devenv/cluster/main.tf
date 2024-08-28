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

resource "terraform_data" "dragonfly" {
  # setup dragonfly operator and cluster
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i kubespray/mkclusterdev/inventory.ini playbooks/dragonflydb.yml"
  }
  depends_on = [
    terraform_data.nfs
  ]
}

resource "terraform_data" "k8sdashboard" {
  # setup k8s dashboard
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i kubespray/mkclusterdev/inventory.ini playbooks/k8sdashboard.yml"
  }
  depends_on = [
    terraform_data.dragonfly
  ]
}

resource "terraform_data" "mailhog" {
  # setup mailhog
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i kubespray/mkclusterdev/inventory.ini playbooks/mailhog.yml"
  }
  depends_on = [
    terraform_data.k8sdashboard
  ]
}

resource "terraform_data" "docker_registry" {
  # setup docker registry
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i kubespray/mkclusterdev/inventory.ini playbooks/docker-registry.yml"
  }
  depends_on = [
    terraform_data.mailhog
  ]
}

resource "terraform_data" "jenkins" {
  # setup jenkins
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i kubespray/mkclusterdev/inventory.ini playbooks/jenkins.yml"
  }
  depends_on = [
    terraform_data.docker_registry
  ]
}

resource "terraform_data" "dashboard_access" {
  # setup dashboard access
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i kubespray/mkclusterdev/inventory.ini playbooks/k8sdashboard_user.yml"
  }
  depends_on = [
    terraform_data.jenkins
  ]
}
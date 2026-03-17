resource "terraform_data" "localstoragedisk" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/local_storage_disks.yml --ssh-common-args='-o StrictHostKeyChecking=accept-new'"
  }
}

resource "terraform_data" "kubespray" {
  # create the cluster via kubespray
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory/mkclusterdev/inventory.ini cluster.yml --ssh-common-args='-o StrictHostKeyChecking=accept-new'"
    working_dir = "../../../kubespray"
  }
  depends_on = [
    terraform_data.localstoragedisk
  ]
}

resource "terraform_data" "kubeconfig" {
  # setup the kube config
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/kube.yml"
  }
  depends_on = [
    terraform_data.kubespray
  ]
}

resource "terraform_data" "helm" {
  # setup helm
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/helm_version.yml"
  }
  depends_on = [
    terraform_data.kubeconfig
  ]
}

resource "terraform_data" "operator" {
  # setup operator for cluster
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/operator.yml"
  }
  depends_on = [
    terraform_data.helm
  ]
}

resource "terraform_data" "key_setup" {
  # setup keys
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/key_setup.yml"
  }
  depends_on = [
    terraform_data.operator
  ]
}

resource "terraform_data" "certissuer" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/certissuer.yml"
  }
  depends_on = [
    terraform_data.key_setup
  ]
}

resource "terraform_data" "metallb" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/metallb.yml"
  }
  depends_on = [
    terraform_data.certissuer
  ]
}

resource "terraform_data" "nginxingress" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/nginx_ingress.yml"
  }
  depends_on = [
    terraform_data.metallb
  ]
}

resource "terraform_data" "localstorage" {
  # setup the local storage driver
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/local_path_storage.yml"
  }
  depends_on = [
    terraform_data.nginxingress
  ]
}

resource "terraform_data" "longhorn" {
  # setup the longhorn storage driver
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/longhorn.yml"
  }
  depends_on = [
    terraform_data.localstorage
  ]
}

resource "terraform_data" "longhorningress" {
  # setup the longhorn storage driver
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/longhorn_ingress.yml"
  }
  depends_on = [
    terraform_data.longhorn
  ]
}

resource "terraform_data" "nfs" {
  # setup the NFS layer
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -e 'ansible_sudo_pass=${var.vm_user_password}' -i inventory.ini playbooks/nfs.yml"
  }
  depends_on = [
    terraform_data.longhorningress
  ]
}

resource "terraform_data" "monitoring" {
  # setup monitoring
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/grafana_prometheus.yml"
  }
  depends_on = [
    terraform_data.nfs
  ]
}

resource "terraform_data" "k8sdashboard" {
  # setup k8s dashboard
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/k8sdashboard.yml"
  }
  depends_on = [
    terraform_data.monitoring
  ]
}

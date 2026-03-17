resource "terraform_data" "dragonfly" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/dragonflydb.yml"
  }
}

resource "terraform_data" "rabbitmq" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/rabbitmq.yml"
  }
  depends_on = [
    terraform_data.dragonfly
  ]
}

resource "terraform_data" "postfix" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/postfix.yml"
  }
  depends_on = [
    terraform_data.rabbitmq
  ]
}

resource "terraform_data" "wireguard" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/wireguard.yml"
  }
  depends_on = [
    terraform_data.postfix
  ]
}

# resource "terraform_data" "stackgres" {
#   provisioner "local-exec" {
#     command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/stackgres.yml"
#   }
#   depends_on = [
#     terraform_data.wireguard
#   ]
# }

resource "terraform_data" "localai" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/local_ai.yml"
  }
  depends_on = [
    terraform_data.wireguard
  ]
}

resource "terraform_data" "cloudnativepg" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/cloudnativepg.yml"
  }
  depends_on = [
    terraform_data.wireguard
  ]
}

resource "terraform_data" "pgadmin4" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/pgadmin4.yml"
  }
  depends_on = [
    terraform_data.cloudnativepg
  ]
}

# resource "terraform_data" "elastic" {
#   provisioner "local-exec" {
#     command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/elastic.yml"
#   }
#   depends_on = [
#     terraform_data.pgadmin4
#   ]
# }

# resource "terraform_data" "mediakraken" {
#   # setup mediakraken
#   provisioner "local-exec" {
#     command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/mediakraken.yml"
#   }
#   depends_on = [
#     terraform_data.pgadmin4
#   ]
# }

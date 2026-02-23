terraform {
  required_providers {
    proxmox = {
      source  = "telmate/proxmox"
      version = "3.0.2-rc06"
    }
  }
}

provider "proxmox" {
  pm_api_url          = var.api_url
  pm_api_token_id     = var.token_id
  pm_api_token_secret = var.token_secret
  pm_tls_insecure     = true
}

resource "proxmox_vm_qemu" "mkcontrol" {
  vmid        = "300${count.index}"
  name        = "mkcontrol${count.index + 1}"
  description = "k8s Control Plane"
  count       = 3
  target_node = var.proxmox_host
  clone       = "debian-12-cloudinit-template-mk"
  hotplug     = "network,disk"
  cpu {
    cores       = 2
    sockets     = 2
    type        = "host"
    numa        = true
  }
  memory      = 16384
  agent       = 1
  os_type     = "cloud-init"
  full_clone  = "true"
  scsihw      = "virtio-scsi-pci"
  boot        = "order=scsi0"
  bootdisk    = "scsi0"
  start_at_node_boot      = "true"
  ipconfig0   = "ip=192.168.50.5${count.index}/24,gw=192.168.50.1"
  nameserver  = "192.168.1.4"
  ciuser      = var.vm_user
  cipassword  = var.vm_user_password
  sshkeys     = file("~/.ssh/id_rsa.pub")
  tags        = "k8s"

  disks {
    ide {
      ide3 {
        cloudinit {
          storage = "local-lvm"
        }
      }
    }
    scsi {
      scsi0 {
        disk {
          size    = "64G"
          storage = var.storage_name
        }
      }
    }
  }

  network {
    id        = 0
    model     = "virtio"
    bridge    = var.nic_name
    firewall  = false
    link_down = false
  }

  vga {
    type = "std"
  }

  lifecycle {
    ignore_changes = [
      network,
    ]
  }
}

resource "proxmox_vm_qemu" "mkworker" {
  vmid        = "400${count.index}"
  name        = "mkworker${count.index + 1}"
  description = "k8s Worker Node"
  count       = 3
  target_node = var.proxmox_host
  clone       = "debian-12-cloudinit-template-mk"
  hotplug     = "network,disk"
  cpu {
    cores       = 10
    sockets     = 2
    type        = "host"
    numa        = true
  }
  memory      = 131072
  agent       = 1
  os_type     = "Linux"
  full_clone  = "true"
  scsihw      = "virtio-scsi-pci"
  boot        = "order=scsi0"
  bootdisk    = "scsi0"
  start_at_node_boot      = "true"
  ipconfig0   = "ip=192.168.50.6${count.index}/24,gw=192.168.50.1"
  nameserver  = "192.168.1.4"
  ciuser      = var.vm_user
  cipassword  = var.vm_user_password
  sshkeys     = file("~/.ssh/id_rsa.pub")
  tags        = "k8s"

  disks {
    ide {
      ide3 {
        cloudinit {
          storage = "local-lvm"
        }
      }
    }
    scsi {
      scsi0 {
        disk {
          size    = "128G"
          storage = var.storage_name
        }
      }
      scsi1 {
        disk {
          size    = "1T"
          storage = var.storage_name
        }
      }
      scsi2 {
        disk {
          size    = "2T"
          storage = var.storage_name
        }
      }
    }
  }

  network {
    id        = 0
    model     = "virtio"
    bridge    = var.nic_name
    firewall  = false
    link_down = false
  }

  vga {
    type = "std"
  }

  lifecycle {
    ignore_changes = [
      network,
    ]
  }
}

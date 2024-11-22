terraform {
  required_providers {
    proxmox = {
      source  = "telmate/proxmox"
      version = "3.0.1-rc4"
    }
  }
}

provider "proxmox" {
  pm_api_url          = var.api_url
  pm_api_token_id     = var.token_id
  pm_api_token_secret = var.token_secret
  pm_tls_insecure     = true
}

resource "proxmox_vm_qemu" "mkk8scontroldev" {
  vmid        = "100${count.index}"
  name        = "mkcontroldev${count.index + 1}"
  desc        = "k8s Control Plane"
  count       = 1
  target_node = var.proxmox_host
  clone       = "debian-12-cloudinit-template-mk"
  hotplug     = "network,disk"
  cores       = 2
  sockets     = 1
  cpu         = "host"
  memory      = 8192
  numa        = true
  agent       = 1
  os_type     = "cloud-init"
  full_clone  = "true"
  scsihw      = "virtio-scsi-pci"
  boot        = "order=scsi0"
  bootdisk    = "scsi0"
  onboot      = "true"
  ipconfig0   = "ip=192.168.1.5${count.index}/24,gw=192.168.1.1"
  nameserver  = "192.168.1.1"
  ciuser      = var.vm_user
  cipassword  = var.vm_user_password
  sshkeys     = file("~/.ssh/id_rsa.pub")
  tags        = "k8sdev"

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

resource "proxmox_vm_qemu" "mkk8sworkerdev" {
  vmid        = "110${count.index}"
  name        = "mkworkerdev${count.index + 1}"
  desc        = "k8s Worker Node"
  count       = 2
  target_node = var.proxmox_host
  clone       = "debian-12-cloudinit-template-mk"
  hotplug     = "network,disk"
  cores       = 4
  sockets     = 2
  cpu         = "host"
  memory      = 32768
  numa        = true
  agent       = 1
  os_type     = "Linux"
  full_clone  = "true"
  scsihw      = "virtio-scsi-pci"
  boot        = "order=scsi0"
  bootdisk    = "scsi0"
  onboot      = "true"
  ipconfig0   = "ip=192.168.1.6${count.index}/24,gw=192.168.1.1"
  nameserver  = "192.168.1.1"
  ciuser      = var.vm_user
  cipassword  = var.vm_user_password
  sshkeys     = file("~/.ssh/id_rsa.pub")
  tags        = "k8sdev"

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
    }
  }

  network {
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

# this setup is run from a debian linux install
it will create proxmox vm's via opentofu
it will then install k8s cluster via kubespray

# install process
## Setup Proxmox
### Setup roles/etc for OpenTofu
Run the following commands on your Proxmox node

```pveum role add terraform-role -privs "VM.Allocate VM.Clone VM.Config.CDROM VM.Config.CPU VM.Config.Cloudinit VM.Config.Disk VM.Config.HWType VM.Config.Memory VM.Config.Network VM.Config.Options VM.Monitor VM.Audit VM.PowerMgmt Datastore.AllocateSpace Datastore.Audit User.Modify Sys.Audit Sys.Console Sys.Modify VM.Migrate Pool.Allocate SDN.Use"```

```pveum user add terraform@pve```

```pveum aclmod / -user terraform@pve -role terraform-role```

```pveum user token add terraform@pve terraform-token --privsep=0```

### Setup template to use
Run the following commands on your Proxmox node

```apt-get update && apt install libguestfs-tools -y```

```wget https://cloud.debian.org/images/cloud/trixie/20251006-2257/debian-13-genericcloud-amd64-20251006-2257.qcow2```


# Setup Proxmox
## Setup roles/etc for OpenTofu
Run the following commands on your Proxmox node

```
pveum role add terraform-role -privs "VM.Allocate VM.Clone VM.Config.CDROM VM.Config.CPU VM.Config.Cloudinit VM.Config.Disk VM.Config.HWType VM.Config.Memory VM.Config.Network VM.Config.Options VM.Audit VM.PowerMgmt Datastore.AllocateSpace Datastore.Audit User.Modify Sys.Audit Sys.Console Sys.Modify VM.Migrate Pool.Allocate Pool.Audit SDN.Use"

pveum user add terraform@pve

pveum aclmod / -user terraform@pve -role terraform-role

pveum user token add terraform@pve terraform-token --privsep=0
```

## Setup template to use
### Run the following commands on your Proxmox node
### dhcpcd-base is required for virt-customize to run!
```
apt-get update && apt install libguestfs-tools dhcpcd-base -y

wget https://cloud.debian.org/images/cloud/trixie/20251117-2299/debian-13-genericcloud-amd64-20251117-2299.qcow2

virt-customize -a debian-13-genericcloud-amd64-20251117-2299.qcow2 --install qemu-guest-agent --run-command 'systemctl enable qemu-guest-agent.service'

virt-customize -a debian-13-genericcloud-amd64-20251006-2257.qcow2 --run-command "echo -n > /etc/machine-id"

qm create 10000 --name "debian-13-cloudinit-template-mk" --memory 2048 --cores 2 --net0 virtio,bridge=vmbr0
qm importdisk 10000 debian-13-genericcloud-amd64-20251117-2299.qcow2 local-lvm
qm set 10000 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-10000-disk-0
qm set 10000 --boot c --bootdisk scsi0
qm set 10000 --ide2 local-lvm:cloudinit
qm set 10000 --agent enabled=1
qm template 10000
```

TODO copy keys to var.tf and stuff
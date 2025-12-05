# acquire domain name with Route53
## give aws money

# setup IAM policy
## login to console
## iam
## policies - left hand panel
## define - json
## name/create/save
```
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": "route53:GetChange",
      "Resource": "arn:aws:route53:::change/*"
    },
    {
      "Effect": "Allow",
      "Action": [
        "route53:ChangeResourceRecordSets",
        "route53:ListResourceRecordSets"
      ],
      "Resource": "arn:aws:route53:::hostedzone/*",
      "Condition": {
        "ForAllValues:StringEquals": {
          "route53:ChangeResourceRecordSetsRecordTypes": ["TXT"]
        }
      }
    },
    {
      "Effect": "Allow",
      "Action": "route53:ListHostedZonesByName",
      "Resource": "*"
    }
  ]
}
```

# setup garage S3
## setup linux of choice with docker and docker compose
```
debian 13 - garage.mediakraken.media
vgcreate garagevg /dev/sdb
lvcreate -l 100%FREE -n garagelv garagevg
mkfs.ext4 /dev/mapper/garagevg-garagelv
mkdir /mnt/garage
mount /dev/mapper/garagevg-garagelv /mnt/garage

run the k8s/prep/garage.toml in the /mnt/garage directory
```
## setup bucket in s3
```
http://garage.mediakraken.media:3903

cluster/dashboard
  assign on attached node
create bucket mkdbbackups
create key
  go to bucket mkdbbackups
    add access key as read/write

for each bucket........create dns record (pfsense in this case)
dns forwarder
custom options
server: local-zone: "mkdbbackups.garage.mediakraken.media"
redirect local-data: "mkdbbackups.garage.mediakraken.media 86400 IN A 192.168.1.x"
```
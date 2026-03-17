MOUNT the drive
mount /dev/mapper/garagevg-lvol0 /mnt/minio

http://mkdbbackups.garage.mediakraken.media::3900

get to the webui
http://192.168.1.191:3909


# find the id
ssh metaman@192.168.1.191
su -
docker exec garage /garage node id


404c793ba9ca2b952d30086d2aed44f934ea9a90054144c81a399c0df2a672f7@127.0.0.1:3901

need to add to pihole dns

mkdbbackups.garage.mediakraken.media



kubectl logs pgcluster-with-metrics-1 -n cnpg-system
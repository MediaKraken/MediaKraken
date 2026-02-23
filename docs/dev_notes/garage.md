http://mkdbbackups.garage.mediakraken.media::3900

get to the webui
http://192.168.1.191:3909


# find the id
ssh metaman@192.168.1.191
su -
docker exec garage /garage node id


012afe46f81ae47a6386d3de5fa18ab5e2ed01525ce422411e5518c706dd5c46@192.168.1.191:3901


need to add to pihole dns

mkdbbackups.garage.mediakraken.media



kubectl logs pgcluster-with-metrics-1 -n cnpg-system
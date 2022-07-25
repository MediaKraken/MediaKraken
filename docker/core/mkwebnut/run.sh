#!/bin/bash

# Auto generate passwords if not provided
if [[ -z "$API_PASSWORD" ]];
then
   API_PASSWORD=$(date | md5sum | cut -c1,5,9,7,4,3,5,1,7,2,3)
fi

if [[ -z "$ADMIN_PASSWORD" ]];
then
   ADMIN_PASSWORD=$(date | md5sum | cut -c1,5,9,7,4,3,5,1,7,2,3)
fi

# Populate nut-upsd config files
if [[ ! -f /etc/nut/ups.conf ]]; then
  cat >/etc/nut/ups.conf <<EOF
[$UPS_NAME]
	desc = "$UPS_DESC"
	driver = $UPS_DRIVER
	port = $UPS_PORT
EOF
fi

if [[ ! -f /etc/nut/upsd.conf ]]; then
  cat >/etc/nut/upsd.conf <<EOF
LISTEN 0.0.0.0 3493
EOF
fi

if [[ ! -f /etc/nut/upsd.users ]]; then
  cat >/etc/nut/upsd.users <<EOF
[admin]
	password = $ADMIN_PASSWORD
	actions = set
	actions = fsd
	instcmds = all
[$API_USER]
	password = $API_PASSWORD
	upsmon master
EOF
fi

if [[ ! -f /etc/nut/upsmon.conf ]]; then
  cat >/etc/nut/upsmon.conf <<EOF
MONITOR $UPS_NAME@localhost 1 $API_USER $API_PASSWORD master
NOTIFYFLAG ONLINE   SYSLOG+WALL
SHUTDOWNCMD "$SHUTDOWN_CMD"
EOF
fi

# Create webNUT config file
if [[ ! -f /app/webNUT/webnut/config.py ]]; then
  cat >/app/webNUT/webnut/config.py <<EOF
server = '127.0.0.1'
port = '3493'
username = '$API_USER'
password = '$API_PASSWORD'
EOF
fi

# Set to standalone mode
sed -i 's/MODE=none/MODE=standalone/g' /etc/nut/nut.conf

# Set permissions
chgrp -R nut /etc/nut /dev/bus/usb
chmod -R o-rwx /etc/nut

# Start nut services in order
exec /sbin/upsdrvctl start &
sleep 45
/sbin/upsd
sleep 15
/sbin/upsmon
sleep 30

# Disable UPS beep (blazer_usb)
/bin/upscmd ups beeper.toggle &


cd /app/webNUT/webnut
pserve ../production.ini

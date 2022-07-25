# Network UPS Tools server with webNUT interface
Docker image combining Network UPS Tools server and webNUT in 1 docker

## Usage
This image provides a complete UPS monitoring service (USB driver only).

Start the container:
```
# docker run \
	--name nut-upsd \
	-p 3493:3493 \
  	-p 6543:6543 \
	-e SHUTDOWN_CMD="echo 'System shutdown not configured!'" \
	-v 'nut-volume':'/etc/nut/' \
	-v 'webnut-volume':'/app/webNUT/webnut/' \
	--device /dev/bus/usb/xxx/yyy \ 
	upshift/nut-upsd
```

## Variables
Auto configuration via environment variables
This image supports customization via environment variables.

#### UPS_NAME
Default value: ups

The name of the UPS.

#### UPS_DESC
Default value: UPS

This allows you to set a brief description that upsd will provide to clients that ask for a list of connected equipment.

#### UPS_DRIVER
Default value: blazer_usb

This specifies which program will be monitoring this UPS.

#### UPS_PORT
Default vaue: auto

This is the serial port where the UPS is connected.

#### API_USER
Default vaue: monuser

This is the username used for communication between upsmon and upsd processes.

#### API_PASSWORD
Default vaue: ***auto-generated***

This is the password for the upsmon user.

#### ADMIN_PASSWORD
Default vaue: ***auto-generated***

This is the admin password.

#### SHUTDOWN_CMD
Default vaue: echo 'System shutdown not configured!'


## Sources:
webNUT: https://github.com/rshipp/webNUT<br/>
Network UPS Tools: https://networkupstools.org/<br/>
nut-upsd docker sample: https://github.com/upshift-docker/nut-upsd/<br/>
webNUT docker sample: https://github.com/jakezp/arm32v7-webnut/<br/>



This is the command upsmon will run when the system needs to be brought down. The command will be run from inside the container.

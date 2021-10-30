import subprocess
import shlex

# shouldn't need to build out Rust since that's container build
# TODO do I really need a registry for the test/dev environment?

# build out docker and docker-compose
subprocess.call(shlex.split('python3 ../docker_compose/mediakraken_setup.py'),
                stdout=subprocess.PIPE, shell=False)

# build the jenkins build for mediakraken
subprocess.call(shlex.split('docker build ../docker/test/mkjenkins/. -t mkjenkins'),
                stdout=subprocess.PIPE, shell=False)

# TODO setup the mailcow
# subprocess.call(shlex.split('python3 ../docker_compose/mediakraken_setup.py'),
#                 stdout=subprocess.PIPE, shell=False)

# selenium mediakraken test image
subprocess.call(shlex.split('docker build ../docker/test/mkselenium/. -t mkselenium'),
                stdout=subprocess.PIPE, shell=False)

# trac instance for bug tracking
subprocess.call(shlex.split('docker build ../docker/test/mktrac/. -t mktrac'),
                stdout=subprocess.PIPE, shell=False)

"""
  Copyright (C) 2021 Quinn D Granfor <spootdev@gmail.com>

  This program is free software; you can redistribute it and/or
  modify it under the terms of the GNU General Public License
  version 2, as published by the Free Software Foundation.

  This program is distributed in the hope that it will be useful, but
  WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
  General Public License version 2 for more details.

  You should have received a copy of the GNU General Public License
  version 2 along with this program; if not, write to the Free
  Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
  MA 02110-1301, USA.
"""

import os
import shlex
import subprocess
import time
from dotenv import load_dotenv
from docker_build import network_email as common_network_email

# load .env stats
load_dotenv()

CWD_HOME_DIRECTORY = os.getcwd().rsplit('MediaKraken', 1)[0]
print(CWD_HOME_DIRECTORY, flush=True)

#####################################
# start up the application so can see running images for several tools
#####################################
os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken', 'docker_compose'))
pid_proc = subprocess.Popen(shlex.split('./mediakraken_start.sh'),
                            stdout=subprocess.PIPE, shell=False)
while True:
    line = pid_proc.stdout.readline()
    if not line:
        break
    print(line.rstrip(), flush=True)
pid_proc.wait()
# this sleep is here so that everything has time to fully start like pika
time.sleep(60)

"""
ab -n 10000 -c 30 https://th-mediakraken-1/
"""

#####################################
# stop the application
#####################################
os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken', 'docker_compose'))
pid_proc = subprocess.Popen(shlex.split('./mediakraken_stop.sh'),
                            stdout=subprocess.PIPE, shell=False)
pid_proc.wait()
# this sleep is here so that everything has time to fully stop like pika
time.sleep(60)

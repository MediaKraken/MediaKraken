import argparse
import os
import shlex
import subprocess
import sys
import time

CWD_HOME_DIRECTORY = os.getcwd().rsplit('MediaKraken', 1)[0]
os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken/src'))

# TODO check for failures?

# publish to kellnr
libs_to_publish = ["mk_lib_common",
                   "mk_lib_compression",
                   "mk_lib_filler",
                   "mk_lib_image",
                   "mk_lib_logging",
                   "mk_lib_rabbitmq",
                   "mk_lib_database",
                   "mk_lib_file",
                   "mk_lib_hash",
                   "mk_lib_network",
                   "mk_lib_hardware",
                   "mk_lib_metadata"]

for lib_name in libs_to_publish:
    print(lib_name)
    # build the library
    pid_proc = subprocess.Popen(['cargo', 'build', '-p', lib_name])
    pid_proc.wait()
    # TODO send to mailhog the results, don't publish if failed
    # publish the libary
    pid_proc = subprocess.Popen(["cargo", "publish", "--registry", "kellnr", "-p", lib_name, "--allow-dirty", "--token=3dPvCZSd1V8xKpiCGqpXCBbkj4Jtyyqc"])
    pid_proc.wait()

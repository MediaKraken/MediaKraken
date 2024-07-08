import os
import shlex
import subprocess
from base64 import b64encode

if not os.path.isfile('./mkstack-db-password.txt'):
    file_handle = open('./mkstack-db-password.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.Popen(shlex.split('kubectl create secret generic db-password --from-file=./mkstack-db-password.txt --namespace=mediakraken'),
                                  stdout=subprocess.PIPE, shell=False)
    while True:
        line = install_pid.stdout.readline()
        if not line:
            break
        print(line.rstrip(), flush=True)
    install_pid.wait()

if not os.path.isfile('./mkstack-secure-key.txt'):
    file_handle = open('./mkstack-secure-key.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.Popen(shlex.split('kubectl create secret generic secure-key --from-file=./mkstack-secure-key.txt --namespace=mediakraken'),
                                  stdout=subprocess.PIPE, shell=False)
    while True:
        line = install_pid.stdout.readline()
        if not line:
            break
        print(line.rstrip(), flush=True)
    install_pid.wait()

if not os.path.isfile('./mkstack-csrf-key.txt'):
    file_handle = open('./mkstack-csrf-key.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.Popen(shlex.split('kubectl create secret generic csrf-key --from-file=./mkstack-csrf-key.txt --namespace=mediakraken'),
                                  stdout=subprocess.PIPE, shell=False)
    while True:
        line = install_pid.stdout.readline()
        if not line:
            break
        print(line.rstrip(), flush=True)
    install_pid.wait()

if not os.path.isfile('./mkstack-nut.txt'):
    file_handle = open('./mkstack-nut.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.Popen(shlex.split('kubectl create secret generic nut-password --from-file=./mkstack-nut.txt --namespace=mediakraken'),
                                  stdout=subprocess.PIPE, shell=False)
    while True:
        line = install_pid.stdout.readline()
        if not line:
            break
        print(line.rstrip(), flush=True)
    install_pid.wait()

print("Suggest transfering the key files to safe location!")

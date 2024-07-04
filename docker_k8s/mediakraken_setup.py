import os
import shlex
import subprocess
from base64 import b64encode

if not os.path.isfile('./mkstack-db_password.txt'):
    file_handle = open('./mkstack-db_password.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.call(shlex.split('kubectl create secret generic db-password --from-file=./mkstack-db_password.txt --namespace=mediakraken'),
                                  stdout=subprocess.PIPE, shell=False)

if not os.path.isfile('./mkstack-secure_key.txt'):
    file_handle = open('./mkstack-secure_key.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.call(shlex.split('kubectl create secret generic secure-key --from-file=./mkstack-secure_key.txt --namespace=mediakraken'),
                                  stdout=subprocess.PIPE, shell=False)

if not os.path.isfile('./mkstack-csrf_key.txt'):
    file_handle = open('./mkstack-csrf_key.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.call(shlex.split('kubectl create secret generic csrf-key --from-file=./mkstack-csrf_key.txt --namespace=mediakraken'),
                                  stdout=subprocess.PIPE, shell=False)

if not os.path.isfile('./mkstack-nut.txt'):
    file_handle = open('./mkstack-nut.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.call(shlex.split('kubectl create secret generic nut-password --from-file=./mkstack-nut.txt --namespace=mediakraken'),
                                  stdout=subprocess.PIPE, shell=False)

# TODO when production, remove the secret files

import os
import shlex
import subprocess
from base64 import b64encode

if not os.path.isfile('./mkstack-db_password.txt'):
    install_pid = subprocess.call(shlex.split('kubectl get secrets -n stackgres mkdatabase -o jsonpath=\'{.data.superuser-password}\' | base64 -d > ./mkstack-db_password.txt'),
                                  stdout=subprocess.PIPE, shell=False)
    install_pid = subprocess.call(shlex.split('kubectl create secret generic db-password -n mediakraken --from-file=./mkstack-db_password.txt'),
                                  stdout=subprocess.PIPE, shell=False)

if not os.path.isfile('./mkstack-secure_key.txt'):
    file_handle = open('./mkstack-secure_key.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.call(shlex.split('kubectl create secret generic secure-key -n mediakraken --from-file=./mkstack-secure_key.txt'),
                                  stdout=subprocess.PIPE, shell=False)

if not os.path.isfile('./mkstack-csrf_key.txt'):
    file_handle = open('./mkstack-csrf_key.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.call(shlex.split('kubectl create secret generic csrf-key -n mediakraken --from-file=./mkstack-csrf_key.txt'),
                                  stdout=subprocess.PIPE, shell=False)

if not os.path.isfile('./mkstack-nut.txt'):
    file_handle = open('./mkstack-nut.txt', 'w+')
    random_key = b64encode(os.urandom(32)).decode('utf-8')
    file_handle.write(random_key.replace(
        '"', '').replace("'", '').replace("%", ''))
    file_handle.close()
    install_pid = subprocess.call(shlex.split('kubectl create secret generic nut-password -n mediakraken --from-file=./mkstack-nut.txt'),
                                  stdout=subprocess.PIPE, shell=False)

# TODO when production, remove the secret files

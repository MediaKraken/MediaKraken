'''
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
'''

# must login to docker hub first *IF* one wants to push to dockerhub
# docker login --username=mediakraken

import argparse
import os
import shlex
import subprocess
import sys

from dotenv import load_dotenv

import docker_images_list
import network_email

# TODO proxy docker build -t mediakraken/mkbase38py3 --build-arg http_proxy="http://proxyip:8080"
#  --build-arg ALPMIRROR=dl-cdn.alpinelinux.org --build-arg PIPMIRROR=pypi.python.org .

parser = argparse.ArgumentParser(description='This program builds and deploys MediaKraken')
parser.add_argument('-b', '--base', required=False,
                    help='Base images only', action="store_true")
# set args.image variable if entered - ex. ComposeMediaKrakenBaseFFMPEG
parser.add_argument('-i', '--image', metavar='image', required=False,
                    help='Image to build')
parser.add_argument('-r', '--rebuild', required=False,
                    help='Force rebuild with no cached layers', action="store_true")
parser.add_argument('-s', '--security', required=False,
                    help='Build security images', action="store_true")
parser.add_argument('-t', '--testing', required=False,
                    help='Build testing images', action="store_true")
parser.add_argument('-v', '--version', metavar='version', required=False,
                    help='The build version dev/prod')
args = parser.parse_args()

# load .env stats
load_dotenv()

print('Number of arguments:', len(sys.argv), 'arguments.')
print('Argument List:', args)


def build_email_push(build_group, email_subject, branch_tag, push_hub_image=False):
    if args.rebuild:
        docker_no_cache = '--pull --no-cache'  # include pull to force update to new image
    else:
        docker_no_cache = ''
    for docker_images in build_group:
        if args.image is None or (args.image is not None and docker_images == args.image):
            # do the actual build process for docker image
            os.chdir(os.path.join(CWD_HOME_DIRECTORY,
                                  'MediaKraken/docker',
                                  build_group[docker_images][2],
                                  docker_images))
            # TODO check for errors/warnings and stop if found
            # Let the mirror's be passed, if not used it will just throw a warning
            pid_build_proc = subprocess.Popen(shlex.split('docker build %s'
                                                          ' -t mediakraken/%s:%s'
                                                          ' --build-arg BRANCHTAG=%s'
                                                          ' --build-arg ALPMIRROR=%s'
                                                          ' --build-arg DEBMIRROR=%s'
                                                          ' --build-arg PIPMIRROR=%s .' %
                                                          (docker_no_cache,
                                                           build_group[docker_images][0],
                                                           branch_tag, branch_tag,
                                                           docker_images_list.ALPINE_MIRROR,
                                                           docker_images_list.DEBIAN_MIRROR,
                                                           docker_images_list.PYPI_MIRROR)),
                                              stdout=subprocess.PIPE, shell=False)
            email_body = ''
            while True:
                line = pid_build_proc.stdout.readline()
                if not line:
                    break
                email_body += line.decode("utf-8")
                print(line.rstrip(), flush=True)
            pid_build_proc.wait()
            subject_text = ' FAILED'
            if email_body.find('Successfully tagged mediakraken') != -1:
                subject_text = ' SUCCESS'
                # tag for local repo
                pid_tag_proc = subprocess.Popen(
                    shlex.split('docker tag mediakraken/%s:%s %s/mediakraken/%s:%s'
                                % (build_group[docker_images][0],
                                   branch_tag,
                                   docker_images_list.DOCKER_REPOSITORY,
                                   build_group[docker_images][0],
                                   branch_tag)),
                    stdout=subprocess.PIPE, shell=False)
                while True:
                    line = pid_tag_proc.stdout.readline()
                    if not line:
                        break
                    print(line.rstrip(), flush=True)
                pid_tag_proc.wait()
                # push to local repo
                pid_push_proc = subprocess.Popen(
                    shlex.split('docker push %s/mediakraken/%s:%s'
                                % (docker_images_list.DOCKER_REPOSITORY,
                                   build_group[docker_images][0],
                                   branch_tag)),
                    stdout=subprocess.PIPE, shell=False)
                while True:
                    line = pid_push_proc.stdout.readline()
                    if not line:
                        break
                    print(line.rstrip(), flush=True)
                pid_push_proc.wait()
                # push to remote repo
                if push_hub_image:
                    pid_push_proc = subprocess.Popen(
                        shlex.split('docker push mediakraken/%s:%s'
                                    % (build_group[docker_images][0],
                                       branch_tag)),
                        stdout=subprocess.PIPE, shell=False)
                    while True:
                        line = pid_push_proc.stdout.readline()
                        if not line:
                            break
                        print(line.rstrip(), flush=True)
                    pid_push_proc.wait()
            # send success/fail email
            network_email.com_net_send_email(os.environ['MAILUSER'],
                                             os.environ['MAILPASS'],
                                             os.environ['MAILUSER'],
                                             email_subject
                                             + build_stages[docker_images][0]
                                             + subject_text,
                                             email_body,
                                             smtp_server=os.environ['MAILSERVER'],
                                             smtp_port=os.environ['MAILPORT'])


# start
CWD_HOME_DIRECTORY = os.getcwd().rsplit('MediaKraken', 1)[0]
# grab version to build via git branch
pid_git_proc = subprocess.Popen(
    shlex.split('git branch'), stdout=subprocess.PIPE, shell=False)
git_branch = None
while True:
    line = pid_git_proc.stdout.readline()
    if not line:
        break
    print(line.rstrip(), flush=True)
    if line.rstrip().decode('utf-8').find('*') == 0:
        git_branch = line.rstrip().decode('utf-8').split(' ')[1]
        break
pid_git_proc.wait()
if git_branch is None:
    print('Can\'t find Git branch!  Exiting!')
    sys.exit()
else:
    print('Found Git branch: %s' % git_branch)

if not os.path.exists(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken')):
    # backup to main dir with checkouts
    os.chdir(CWD_HOME_DIRECTORY)
    pid_proc = subprocess.Popen(
        shlex.split('git clone -b %s https://github.com/MediaKraken/MediaKraken'
                    % git_branch))
    pid_proc.wait()
else:
    # cd to MediaKraken_Deployment dir
    os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken'))
    # pull down latest code
    pid_proc = subprocess.Popen(['git', 'pull'])
    pid_proc.wait()
    pid_proc = subprocess.Popen(['git', 'checkout', git_branch])
    pid_proc.wait()

# below is needed for the source sync to work!
os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken/docker_build'))

# begin build process
if args.base:
    for build_stages in (docker_images_list.STAGE_ONE_IMAGES,
                         docker_images_list.STAGE_ONE_GAME_SERVERS,):
        build_email_push(build_stages, 'Build base dev image: ',
                         branch_tag=git_branch, push_hub_image=True)

if args.security:
    for build_stages in (docker_images_list.STAGE_ONE_SECURITY_TOOLS,
                         docker_images_list.STAGE_TWO_SECURITY_TOOLS,):
        build_email_push(build_stages, 'Build security image: ',
                         branch_tag=git_branch, push_hub_image=False)

if args.testing:
    for build_stages in (docker_images_list.STAGE_ONE_TESTING_TOOLS,
                         docker_images_list.STAGE_TWO_TESTING_TOOLS):
        build_email_push(build_stages, 'Build testing image: ',
                         branch_tag=git_branch, push_hub_image=False)

if args.version == 'dev' or args.version == 'prod':
    for build_stages in (docker_images_list.STAGE_TWO_IMAGES,
                         docker_images_list.STAGE_CORE_IMAGES,
                         docker_images_list.STAGE_TWO_GAME_SERVERS):
        if args.version == 'dev':
            build_email_push(build_stages, 'Build dev image: ',
                             branch_tag=git_branch, push_hub_image=False)
        else:
            build_email_push(build_stages, 'Build prod image: ',
                             branch_tag=git_branch, push_hub_image=True)

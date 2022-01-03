import shlex
import subprocess
import docker_images_list

try:
    from gomatic import *
except ModuleNotFoundError:
    install_pid = subprocess.Popen(shlex.split('apt install -y python3-pip'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    install_pid = subprocess.Popen(shlex.split('pip3 install gomatic'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    from gomatic import *

# TODO install pyflakes
# TODO install pylint
# TODO python3-pip wget
"""
wget https://github.com/hadolint/hadolint/releases/download/v2.8.0/hadolint-Linux-x86_64
mv hadolint-Linux-x86_64 /usr/bin/hadolint
chmod +x /usr/bin/hadolint
"""


# connect to gocd instance
configurator = GoCdConfigurator(HostRestClient("th-mkbuild-1:8153"))
pipeline = configurator \
    .ensure_pipeline_group("MediaKraken") \
    .ensure_replacement_of_pipeline("mediakraken_code_pipeline") \
    .set_git_url("https://github.com/MediaKraken/MediaKraken")

# start creating stages and jobs
stage = pipeline.ensure_stage("code_count")
job = stage.ensure_job("cloc_job")
job.add_task(ExecTask(['cloc', '.']))

stage = pipeline.ensure_stage("linting")
job = stage.ensure_job("lint_docker")
job.add_task(ExecTask(['hadolint', '$(git ls-files | grep Dockerfile)']))
job = stage.ensure_job("lint_python")
job.add_task(ExecTask(['pylint', '$(git ls-files *.py)']))
job.add_task(ExecTask(['pyflakes', '.']))  # it didn't like the git method above
# job = stage.ensure_job("lint_rust")
# job.add_task(ExecTask(['cloc', '.']))

pipeline = configurator \
    .ensure_pipeline_group("MediaKraken") \
    .ensure_replacement_of_pipeline("mediakraken_build_pipeline") \
    .set_git_url("https://github.com/MediaKraken/MediaKraken")
stage = pipeline.ensure_stage("docker_build_base")
job = stage.ensure_job("build_base")
for build_group in (docker_images_list.STAGE_ONE_IMAGES,
                     docker_images_list.STAGE_ONE_GAME_SERVERS,):
    for docker_images in build_group:
        job.add_task(ExecTask(['docker', 'build -t mediakraken/%s:refactor' % (build_group[docker_images][0])]))

# stage = pipeline.ensure_stage("docker_build_core")

# stage = pipeline.ensure_stage("docker_build_games")

# stage = pipeline.ensure_stage("test_mediakraken")

# pipeline = configurator \
#     .ensure_pipeline_group("MediaKraken") \
#     .ensure_replacement_of_pipeline("mediakraken_test_pipeline") \
#     .set_git_url("https://github.com/MediaKraken/MediaKraken")
#
# pipeline = configurator \
#     .ensure_pipeline_group("MediaKraken") \
#     .ensure_replacement_of_pipeline("mediakraken_deploy_pipeline") \
#     .set_git_url("https://github.com/MediaKraken/MediaKraken")

configurator.save_updated_config()


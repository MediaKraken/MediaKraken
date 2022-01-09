import shlex
import subprocess
import docker_images_list

try:
    from gomatic import ExecTask, GoCdConfigurator, HostRestClient
except ModuleNotFoundError:
    install_pid = subprocess.Popen(shlex.split('apt-get install -y python3-pip'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    install_pid = subprocess.Popen(shlex.split('pip3 install gomatic'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    from gomatic import ExecTask, GoCdConfigurator, HostRestClient

# TODO pip3 install pylint bandit pyflakes vulture dead bashate
# TODO for python/linting....can't import
# TODO pip3 install pytest selenium psutil flask guessit gomatic
# TODO python3-pip wget shellcheck cppcheck
# TODO chmod 666 /var/run/docker.sock

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
# If a directory is given instead of a filename, *.cpp, *.cxx, *.cc, *.c++, *.c,
# *.tpp, and *.txx files are checked recursively from the given directory.
# TODO should I use --quiet flag?
job = stage.ensure_job("c_cpp_cppcheck")
job.add_task(ExecTask(['cppcheck', '--enable=all', '--platform=unix64', '.']))

job = stage.ensure_job("dockerfile_hadolint")
# TODO use -t error later on to fail only on error or above
# TODO maybe use --no-fail.......but then it doesn't error and display nice in gocd
job.add_task(ExecTask(['bash', '-c', 'hadolint $(git ls-files | grep Dockerfile)']))

job = stage.ensure_job("python_pyflakes")
job.add_task(ExecTask(['pyflakes', '.']))  # it didn't like the git method
job = stage.ensure_job("python_pylint")
job.add_task(ExecTask(['bash', '-c', 'pylint $(git ls-files *.py)']))

job = stage.ensure_job("shell_shellcheck")
job.add_task(ExecTask(['bash', '-c', 'shellcheck $(git ls-files *.sh)']))
job = stage.ensure_job("shell_bashate")
job.add_task(ExecTask(['bash', '-c', 'bashate $(git ls-files *.sh)']))

# job = stage.ensure_job("rust_clippy")
# job.add_task(ExecTask(['cloc', '.']))


stage = pipeline.ensure_stage("dead_code")
job = stage.ensure_job("python_vulture")
job.add_task(ExecTask(['bash', '-c', 'vulture $(git ls-files *.py)']))

job = stage.ensure_job("python_dead")
job.add_task(ExecTask(['bash', '-c', 'dead']))  # --files $(git ls-files *.py)'])) needs to be regex


stage = pipeline.ensure_stage("code_security")
job = stage.ensure_job("bandit_python")
job.add_task(ExecTask(['bandit', '-r', '.']))

pipeline = configurator \
    .ensure_pipeline_group("MediaKraken") \
    .ensure_replacement_of_pipeline("mediakraken_build_pipeline") \
    .set_git_url("https://github.com/MediaKraken/MediaKraken")

stage = pipeline.ensure_stage("docker_build_base")
for build_group in (docker_images_list.STAGE_ONE_IMAGES,
                    docker_images_list.STAGE_TWO_IMAGES,
                    docker_images_list.STAGE_ONE_GAME_SERVERS,):
    for docker_images in build_group:
        job = stage.ensure_job("build_base_%s" % build_group[docker_images][0])
        job.add_task(ExecTask(['bash', '-c', 'docker build -t mediakraken/%s:refactor'
                                             ' --build-arg ALPMIRROR=%s'
                                             ' --build-arg DEBMIRROR=%s'
                                             ' --build-arg PIPMIRROR=%s'
                                             ' ./docker/%s/%s/.' % (build_group[docker_images][0],
                                                                    docker_images_list.ALPINE_MIRROR,
                                                                    docker_images_list.DEBIAN_MIRROR,
                                                                    docker_images_list.PYPI_MIRROR,
                                                                    build_group[docker_images][2],
                                                                    docker_images)]))


stage = pipeline.ensure_stage("docker_build_core")
for build_group in (docker_images_list.STAGE_CORE_IMAGES,):
    for docker_images in build_group:
        job = stage.ensure_job("build_core_%s" % build_group[docker_images][0])
        job.add_task(ExecTask(['bash', '-c', 'docker build -t mediakraken/%s:refactor'
                                             ' --build-arg ALPMIRROR=%s'
                                             ' --build-arg DEBMIRROR=%s'
                                             ' --build-arg PIPMIRROR=%s'
                                             ' ./docker/%s/%s/.' % (build_group[docker_images][0],
                                                                    docker_images_list.ALPINE_MIRROR,
                                                                    docker_images_list.DEBIAN_MIRROR,
                                                                    docker_images_list.PYPI_MIRROR,
                                                                    build_group[docker_images][2],
                                                                    docker_images)]))


stage = pipeline.ensure_stage("docker_build_games")
for build_group in (docker_images_list.STAGE_TWO_GAME_SERVERS,):
    for docker_images in build_group:
        job = stage.ensure_job("build_games_%s" % build_group[docker_images][0])
        job.add_task(ExecTask(['bash', '-c', 'docker build -t mediakraken/%s:refactor'
                                             ' --build-arg ALPMIRROR=%s'
                                             ' --build-arg DEBMIRROR=%s'
                                             ' --build-arg PIPMIRROR=%s'
                                             ' ./docker/%s/%s/.' % (build_group[docker_images][0],
                                                                    docker_images_list.ALPINE_MIRROR,
                                                                    docker_images_list.DEBIAN_MIRROR,
                                                                    docker_images_list.PYPI_MIRROR,
                                                                    build_group[docker_images][2],
                                                                    docker_images)]))


stage = pipeline.ensure_stage("docker_security")
job = stage.ensure_job("docker_dockerbench")
job.add_task(ExecTask(['./docker/test/docker_bench_security.sh']))


# pipeline = configurator \
#     .ensure_pipeline_group("MediaKraken") \
#     .ensure_replacement_of_pipeline("mediakraken_test_pipeline") \
#     .set_git_url("https://github.com/MediaKraken/MediaKraken")
# stage = pipeline.ensure_stage("test_mediakraken")
# TODO start mediakraken docker-compose
# TODO test selenium
# TODO security test website
# TODO ab website
# TODO stop mediakraken docker-compose
#
# pipeline = configurator \
#     .ensure_pipeline_group("MediaKraken") \
#     .ensure_replacement_of_pipeline("mediakraken_deploy_pipeline") \
#     .set_git_url("https://github.com/MediaKraken/MediaKraken")
# TODO push to dockerhub

configurator.save_updated_config()

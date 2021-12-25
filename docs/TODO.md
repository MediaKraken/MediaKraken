dev build
debian 11
git pull code
git checkout refactor

cd ./docker_build
python3 build_dev_environment.py

cd ../docker/test
docker-compose -f docker-compose-mailcow.yml pull
wait
docker-compose -f docker-compose-mailcow.yml up -d


docker login --username=mediakraken
python3 build_and_deploy.py -b
mkalpinemirror - Mirror for Alpine linux
    docker/test/mkalpinemirror image

mkcode - VSS Code build/git
    accessible via SSH Key only
    docker installed
    apt install python3-pip
    cd docker_build
        python3 build_dev_environment.py
    standard rustup installed
        root@mkcode:~/.cargo# cat config
        [source.crates-io]
        replace-with = "kellnr-cratesio"
        [source.kellnr-cratesio]
        registry = "git://mkkellrn:9418/cratesio"    
    nano /etc/docker/daemon.json
        {
        "insecure-registries" : ["mkregistry:5000"]
        }
    curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | sudo bash
    apt-get install git-lfs
    in code repo
        git lfs install
        git lfs track --filename docker/game_server/mkgameq3a_cpma/cpma/cpma-mappack-full.zip
        # git rm --cached docker/game_server/mkgameq3a_cpma/cpma/cpma-mappack-full.zip   fix the previous add

mkelk - Elasticsearch, Logstash, Kibana (ELK)
    docker/test/mkelk image
    Access at http://mkelk:5601
    See all indexes: curl 'localhost:9200/_cat/indices?v'

mkftpserver - FTP server for testing
    docker/test/mkftpserver - image

mkgocd - https://www.gocd.org/  FREE & OPEN SOURCE CI/CD SERVER
    docker_build/build_gocd_gomatic.py
    chmod 666 /var/run/docker.sock
    npm install --save-dev htmlhint
    npm install -g --save-dev stylelint stylelint-config-standard
    http://mkgocd:8153/

mkjenkins - Jenkins is an open source automation server.
    docker/test/mkjenkins image

mkkillrn - Rust Cargo repo/proxy
    start.sh script in home with key/etc
    http://mkkellrn:8000/#/
        root@mkcode:~/.cargo# cat config
        [source.crates-io]
        replace-with = "kellnr-cratesio"
        [source.kellnr-cratesio]
        # Default port for Helm deployments is 30418
        registry = "git://mkkellrn:9418/cratesio"

mkmailhog - fake email server to view mail via web interface (receives build and deploy emails)
    http://mkmailhog:8025

mkprod - Live production
    docker swarm from DockerHub
    docker installed

mkregistry
    docker registry

mkselenium - Selenium python
    docker/test/mkselenium image
    includes source of testing webserver code

mksonatype - Security check, repo host, etc
    docker/test/mksonatype howto file
    http://mksonatype:8081/
    admin - metaman
    yum - http://mksonatype:8081/repository/repo_yum/
    apt - http://mksonatype:8081/repository/repo_apt/
    pypi - http://mksonatype:8081/repository/pypi/
        pip3 install --no-cache-dir --trusted-host mksonatype -i http://mksonatype:8081/repository/pypi/simple -r requirements.txt
    docker - http://mksonatype:8081/repository/docker_group/   should hold private and proxy for hub

mkstage - Test DB

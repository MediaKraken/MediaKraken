# dev environment
## k8s dashboard
### https://mkdev.mediakraken.org
#### kubectl get secret admin-user -n kubernetes-dashboard -o jsonpath={".data.token"} | base64 -d
## grafana dashboard
### mkdevgrafana.mediakraken.org
## jenkins
### mkdevjenkins.mediakraken.org
## nexus artifactory - 404 ERROR!!!!!!!
mkdevnexus.mediakraken.org:8081
## kellnr repo - NO CONNECT
mkdevkellnr.mediakraken.org
http://mkdevkellnr:8000/api/v1/crates/
## gocd
### mkdevgocd.mediakraken.org
## prometheus dashboard
### mkdevprometheus.mediakraken.org
## mailhog ui
### mkdevmailhog.mediakraken.org
## mailhog smtp
### mkdevmailhogsmtp.mediakraken.org 	192.168.1.66
## kellnr repo
mkkellrn.mediakraken.org 	192.168.1.111
http://mkkellnr:8000/api/v1/crates/
## longhorn ui
### mkdevlonghorn.mediakraken.org

mkdevdragonflydb.mediakraken.org
mkdevjfrog.mediakraken.org
mkdevharbor.mediakraken.org

# prod environment
## k8s dashboard
https://mkk8s.mediakraken.org 	192.168.50.200
## grafana dashboard
mkgrafana.mediakraken.org
## stackgres ui
mkstackgres.mediakraken.org
## prometheus dashboard
mkprometheus.mediakraken.org
## rabbitmq ui
mkrabbitmq.mediakraken.org
## longhorn dashboard
mklonghorn.mediakraken.org

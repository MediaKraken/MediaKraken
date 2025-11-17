# dev environment
## k8s dashboard
### https://mkdev.beaverbay.local
#### kubectl get secret admin-user -n kubernetes-dashboard -o jsonpath={".data.token"} | base64 -d
## grafana dashboard
### mkdevgrafana.beaverbay.local
## jenkins
### mkdevjenkins.beaverbay.local
## nexus artifactory - 404 ERROR!!!!!!!
mkdevnexus.beaverbay.local:8081
## kellnr repo - NO CONNECT
mkdevkellnr.beaverbay.local
http://mkdevkellnr:8000/api/v1/crates/
## gocd
### mkdevgocd.beaverbay.local
## prometheus dashboard
### mkdevprometheus.beaverbay.local
## mailhog ui
### mkdevmailhog.beaverbay.local
## mailhog smtp
### mkdevmailhogsmtp.beaverbay.local 	192.168.1.66
## kellnr repo
mkkellrn.beaverbay.local 	192.168.1.111
http://mkkellnr:8000/api/v1/crates/
## longhorn ui
### mkdevlonghorn.beaverbay.local

mkdevdragonflydb.beaverbay.local
mkdevjfrog.beaverbay.local
mkdevharbor.beaverbay.local

# prod environment
## k8s dashboard
https://mkk8s.beaverbay.local 	192.168.50.200
## grafana dashboard
mkgrafana.beaverbay.local
## stackgres ui
mkstackgres.beaverbay.local
## prometheus dashboard
mkprometheus.beaverbay.local
## rabbitmq ui
mkrabbitmq.beaverbay.local
## longhorn dashboard - 503 ERROR!!!!!!! 
mklonghorn.beaverbay.local

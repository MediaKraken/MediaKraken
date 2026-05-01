kubectl cnpg status pgcluster-with-metrics -n cnpg-system

kubectl cnpg restart pgcluster-with-metrics -n cnpg-system



kubectl edit cluster pgcluster-with-metrics -n cnpg-system


**************************

kubectl -n cnpg-system exec -it pgcluster-with-metrics-2 -- \
  psql -U postgres -c "select pg_is_in_recovery(), pg_current_wal_lsn(), pg_control_checkpoint();"

You want:

pg_is_in_recovery = false


kubectl -n cnpg-system get pvc | grep pgcluster-with-metrics-1
    pvc-ed4203fc-3ece-43ab-9e32-2e4acafb761e

kubectl -n cnpg-system patch cluster pgcluster-with-metrics \
  --type='merge' \
  -p '{"spec":{"instances":1}}'

  kubectl -n cnpg-system get pods -l cnpg.io/cluster=pgcluster-with-metrics

kubectl -n cnpg-system delete pvc pvc-ed4203fc-3ece-43ab-9e32-2e4acafb761e

kubectl -n cnpg-system patch cluster pgcluster-with-metrics \
  --type='merge' \
  -p '{"spec":{"instances":3}}'
SELECT
  pid,
  usename,
  datname,
  state,
  now() - query_start AS duration,
  wait_event_type,
  wait_event,
  query
FROM pg_stat_activitya
WHERE state = 'active'
  AND pid <> pg_backend_pid()
ORDER BY duration DESC;
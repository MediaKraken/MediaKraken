BEGIN;

CREATE INDEX cover_art_idx_release ON cover_art (release);
CREATE UNIQUE INDEX art_type_idx_gid ON art_type (gid);

COMMIT;
/*

pull media via upc and discid and makekv rips via ffprobe

partition on mm_release_media_type
CREATE TABLE IF NOT EXISTS mm_releases (
        mm_release_uuid UUID NOT NULL,    <- uuid7 so added/etc not needed as column (primary key)
        mm_release_media_type INT NOT NULL,  <- from how about an enum since I have a list elsewhere
        mm_release_upc text
        mm_release_ean text
        mm_release_asin text
        mm_release_discid text - think it's cd only (maybe dvd)
        mm_release_blake3_hash text
        mm_release_json JSONB NOT NULL,
        )
 PARTITION BY LIST (mm_release_media_type);


 CREATE TABLE mm_releases_us PARTITION OF mm_releases
    FOR VALUES IN ('US');


*/

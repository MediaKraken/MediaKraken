-- 1. Add the column using SRID 4326 (the standard WGS 84 GPS coordinate system)
ALTER TABLE zCache_Ads ADD GeoPoint POINT NOT NULL SRID 4326;

-- 2. Populate it using your existing columns (Note: Latitude is first!)
UPDATE zCache_Ads SET GeoPoint = POINT(GeoLat, GeoLong);

-- 3. Create the native SPATIAL R-Tree index
ALTER TABLE zCache_Ads ADD SPATIAL INDEX idx_spatial_geo (GeoPoint);





ALTER TABLE zCache_Ads ADD GeoPoint POINT NOT NULL SRID 4326;
UPDATE zCache_Ads SET GeoPoint = POINT(GeoLat, GeoLong);
ALTER TABLE zCache_Ads ADD SPATIAL INDEX idx_spatial_geo (GeoPoint);

ALTER TABLE zCache_Ads_FullText ADD GeoPoint POINT NOT NULL SRID 4326;
UPDATE zCache_Ads_FullText SET GeoPoint = POINT(GeoLat, GeoLong);
ALTER TABLE zCache_Ads_FullText ADD SPATIAL INDEX idx_spatial_geo (GeoPoint);

can't be nulls........so, store   - pg18 doesn't have this issue
POINT(0, 0)


this is built via a copy?
zzTmp_savedsearch_zCache_Ads_FullText


# how to query
SELECT `PrimaryCategoryId` , COUNT ( `AdId` ) AS `AdCount` 
FROM `zCache_Ads` 
WHERE `PrimaryCategoryId` = ? 
  -- Find points within 80467 meters (50 miles) of your target center point
  AND ST_Distance_Sphere(GeoPoint, POINT([TargetLat], [TargetLong])) <= 80467
GROUP BY `PrimaryCategoryId`;


# find shit queries
SELECT DIGEST_TEXT, COUNT_STAR, AVG_TIMER_WAIT/1000000000 AS avg_ms  FROM performance_schema.events_statements_summary_by_digest  ORDER BY SUM_TIMER_WAIT DESC LIMIT 3\G






AMD EPYC 7R32 vs AMD EPYC 9R14 Processor or AMD EPYC 9R45 Processor
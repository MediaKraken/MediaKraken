--
-- PostgreSQL database dump
--

-- Dumped from database version 13.3
-- Dumped by pg_dump version 13.3

-- Started on 2021-07-31 15:10:01

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 3526 (class 0 OID 0)
-- Dependencies: 6
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA public IS 'standard public schema';

CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 204 (class 1259 OID 410475)
-- Name: mm_cron_jobs; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_cron_jobs (
    mm_cron_guid uuid NOT NULL,
    mm_cron_name text,
    mm_cron_description text,
    mm_cron_enabled boolean,
    mm_cron_schedule_type text,
    mm_cron_last_run timestamp with time zone,
    mm_cron_json jsonb,
    mm_cron_schedule_time smallint
);


ALTER TABLE public.mm_cron_jobs OWNER TO postgres;

--
-- TOC entry 208 (class 1259 OID 410499)
-- Name: mm_game_dedicated_servers; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_game_dedicated_servers (
    mm_game_server_guid uuid NOT NULL,
    mm_game_server_name text,
    mm_game_server_json jsonb
);


ALTER TABLE public.mm_game_dedicated_servers OWNER TO postgres;

--
-- TOC entry 205 (class 1259 OID 410481)
-- Name: mm_hardware_device; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_hardware_device (
    mm_device_guid uuid NOT NULL,
    mm_device_json jsonb,
    mm_device_spec_guid uuid
);


ALTER TABLE public.mm_hardware_device OWNER TO postgres;

--
-- TOC entry 245 (class 1259 OID 417792)
-- Name: mm_hardware_manufacturer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_hardware_manufacturer (
    mm_hardware_manu_guid uuid NOT NULL,
    mm_hardware_manu_name text,
    mm_hardware_manu_gc_id integer
);


ALTER TABLE public.mm_hardware_manufacturer OWNER TO postgres;

--
-- TOC entry 209 (class 1259 OID 410505)
-- Name: mm_hardware_specifications; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_hardware_specifications (
    mm_hardware_spec_guid uuid NOT NULL,
    mm_hardware_spec_model text,
    mm_hardware_spec_json jsonb,
    mm_hardware_spec_ir_json jsonb,
    mm_hardware_spec_manufacturer_guid uuid
);


ALTER TABLE public.mm_hardware_specifications OWNER TO postgres;

--
-- TOC entry 246 (class 1259 OID 417812)
-- Name: mm_hardware_type; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_hardware_type (
    mm_hardware_type_guid uuid NOT NULL,
    mm_hardware_type_name text
);


ALTER TABLE public.mm_hardware_type OWNER TO postgres;

--
-- TOC entry 212 (class 1259 OID 410526)
-- Name: mm_library_dir; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_library_dir (
    mm_media_dir_guid uuid NOT NULL,
    mm_media_dir_path text,
    mm_media_dir_last_scanned timestamp with time zone,
    mm_media_dir_status jsonb,
    mm_media_dir_class_enum smallint,
    mm_media_dir_username text,
    mm_media_dir_password text
);


ALTER TABLE public.mm_library_dir OWNER TO postgres;

--
-- TOC entry 210 (class 1259 OID 410511)
-- Name: mm_library_link; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_library_link (
    mm_link_guid uuid NOT NULL,
    mm_link_name text,
    mm_link_json jsonb,
    mm_link_username text,
    mm_link_password text
);


ALTER TABLE public.mm_library_link OWNER TO postgres;

--
-- TOC entry 211 (class 1259 OID 410520)
-- Name: mm_media; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_media (
    mm_media_guid uuid NOT NULL,
    mm_media_metadata_guid uuid,
    mm_media_path text,
    mm_media_ffprobe_json jsonb,
    mm_media_json jsonb,
    mm_media_class_enum smallint
);


ALTER TABLE public.mm_media OWNER TO postgres;

--
-- TOC entry 203 (class 1259 OID 410469)
-- Name: mm_media_channel; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_media_channel (
    mm_channel_guid uuid NOT NULL,
    mm_channel_name text,
    mm_channel_media_id jsonb,
    mm_channel_country_guid uuid,
    mm_channel_logo_guid uuid
);


ALTER TABLE public.mm_media_channel OWNER TO postgres;

--
-- TOC entry 213 (class 1259 OID 410532)
-- Name: mm_media_remote; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_media_remote (
    mmr_media_guid uuid NOT NULL,
    mmr_media_link_guid uuid,
    mmr_media_uuid uuid,
    mmr_media_metadata_guid uuid,
    mmr_media_ffprobe_json jsonb,
    mmr_media_json jsonb,
    mmr_media_class_enum smallint
);


ALTER TABLE public.mm_media_remote OWNER TO postgres;

--
-- TOC entry 232 (class 1259 OID 410646)
-- Name: mm_media_sync; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_media_sync (
    mm_sync_guid uuid NOT NULL,
    mm_sync_path text,
    mm_sync_path_to text,
    mm_sync_options_json jsonb
);


ALTER TABLE public.mm_media_sync OWNER TO postgres;

--
-- TOC entry 214 (class 1259 OID 410538)
-- Name: mm_metadata_album; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_album (
    mm_metadata_album_guid uuid NOT NULL,
    mm_metadata_album_name text,
    mm_metadata_album_id jsonb,
    mm_metadata_album_json jsonb,
    mm_metadata_album_musician_guid uuid,
    mm_metadata_album_user_json jsonb,
    mm_metadata_album_localimage jsonb
);


ALTER TABLE public.mm_metadata_album OWNER TO postgres;

--
-- TOC entry 215 (class 1259 OID 410544)
-- Name: mm_metadata_anime; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_anime (
    mm_metadata_anime_guid uuid NOT NULL,
    mm_metadata_anime_name text,
    mm_metadata_anime_json jsonb,
    mm_metadata_anime_mapping jsonb,
    mm_metadata_anime_mapping_before text,
    mm_metadata_anime_localimage jsonb,
    mm_metadata_anime_user_json jsonb,
    mm_metadata_anime_media_id integer
);


ALTER TABLE public.mm_metadata_anime OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 410550)
-- Name: mm_metadata_book; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_book (
    mm_metadata_book_guid uuid NOT NULL,
    mm_metadata_book_isbn text,
    mm_metadata_book_isbn13 text,
    mm_metadata_book_name text,
    mm_metadata_book_json jsonb,
    mm_metadata_book_user_json jsonb,
    mm_metadata_book_localimage jsonb
);


ALTER TABLE public.mm_metadata_book OWNER TO postgres;

--
-- TOC entry 217 (class 1259 OID 410556)
-- Name: mm_metadata_collection; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_collection (
    mm_metadata_collection_guid uuid NOT NULL,
    mm_metadata_collection_name jsonb,
    mm_metadata_collection_media_ids jsonb,
    mm_metadata_collection_json jsonb,
    mm_metadata_collection_imagelocal jsonb
);


ALTER TABLE public.mm_metadata_collection OWNER TO postgres;

--
-- TOC entry 206 (class 1259 OID 410487)
-- Name: mm_metadata_download_que; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_download_que (
    mm_download_guid uuid NOT NULL,
    mm_download_provider text,
    mm_download_que_type smallint,
    mm_download_new_uuid uuid,
    mm_download_provider_id integer,
    mm_download_status text,
    mm_download_path text
);


ALTER TABLE public.mm_metadata_download_que OWNER TO postgres;

--
-- TOC entry 218 (class 1259 OID 410562)
-- Name: mm_metadata_game_software_info; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_game_software_info (
    gi_id uuid NOT NULL,
    gi_system_id uuid,
    gi_game_info_short_name text,
    gi_game_info_name text,
    gi_game_info_json jsonb,
    gi_game_info_localimage jsonb,
    gi_game_info_sha1 text,
    gi_game_info_blake3 text
);


ALTER TABLE public.mm_metadata_game_software_info OWNER TO postgres;

--
-- TOC entry 219 (class 1259 OID 410568)
-- Name: mm_metadata_game_systems_info; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_game_systems_info (
    gs_id uuid NOT NULL,
    gs_game_system_name text,
    gs_game_system_alias text,
    gs_game_system_json jsonb,
    gs_game_system_localimage jsonb
);


ALTER TABLE public.mm_metadata_game_systems_info OWNER TO postgres;

--
-- TOC entry 220 (class 1259 OID 410574)
-- Name: mm_metadata_logo; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_logo (
    mm_metadata_logo_guid uuid NOT NULL,
    mm_metadata_logo_media_guid jsonb,
    mm_metadata_logo_image_path text
);


ALTER TABLE public.mm_metadata_logo OWNER TO postgres;

--
-- TOC entry 221 (class 1259 OID 410580)
-- Name: mm_metadata_movie; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_movie (
    mm_metadata_movie_guid uuid NOT NULL,
    mm_metadata_movie_media_id integer,
    mm_metadata_movie_name text,
    mm_metadata_movie_json jsonb,
    mm_metadata_movie_localimage_json jsonb,
    mm_metadata_movie_user_json jsonb
);


ALTER TABLE public.mm_metadata_movie OWNER TO postgres;

--
-- TOC entry 222 (class 1259 OID 410586)
-- Name: mm_metadata_music; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_music (
    mm_metadata_music_guid uuid NOT NULL,
    mm_metadata_media_music_id jsonb,
    mm_metadata_music_name text,
    mm_metadata_music_json jsonb,
    mm_metadata_music_album_guid uuid,
    mm_metadata_music_user_json jsonb
);


ALTER TABLE public.mm_metadata_music OWNER TO postgres;

--
-- TOC entry 223 (class 1259 OID 410592)
-- Name: mm_metadata_music_video; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_music_video (
    mm_metadata_music_video_guid uuid NOT NULL,
    mm_metadata_music_video_media_id jsonb,
    mm_metadata_music_video_band text,
    mm_metadata_music_video_song text,
    mm_metadata_music_video_json jsonb,
    mm_metadata_music_video_localimage_json jsonb,
    mm_metadata_music_video_user_json jsonb
);


ALTER TABLE public.mm_metadata_music_video OWNER TO postgres;

--
-- TOC entry 224 (class 1259 OID 410598)
-- Name: mm_metadata_musician; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_musician (
    mm_metadata_musician_guid uuid NOT NULL,
    mm_metadata_musician_name text,
    mm_metadata_musician_id jsonb,
    mm_metadata_musician_json jsonb,
    mm_metadata_musician_localimage jsonb
);


ALTER TABLE public.mm_metadata_musician OWNER TO postgres;

--
-- TOC entry 225 (class 1259 OID 410604)
-- Name: mm_metadata_person; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_person (
    mm_metadata_person_guid uuid NOT NULL,
    mm_metadata_person_media_id integer,
    mm_metadata_person_meta_json jsonb,
    mm_metadata_person_image text,
    mm_metadata_person_name text
);


ALTER TABLE public.mm_metadata_person OWNER TO postgres;

--
-- TOC entry 231 (class 1259 OID 410640)
-- Name: mm_metadata_review; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_review (
    mm_review_guid uuid NOT NULL,
    mm_review_metadata_guid uuid,
    mm_review_json jsonb
);


ALTER TABLE public.mm_metadata_review OWNER TO postgres;

--
-- TOC entry 226 (class 1259 OID 410610)
-- Name: mm_metadata_sports; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_sports (
    mm_metadata_sports_guid uuid NOT NULL,
    mm_metadata_media_sports_id jsonb,
    mm_metadata_sports_name text,
    mm_metadata_sports_json jsonb,
    mm_metadata_sports_user_json jsonb,
    mm_metadata_sports_image_json jsonb
);


ALTER TABLE public.mm_metadata_sports OWNER TO postgres;

--
-- TOC entry 227 (class 1259 OID 410616)
-- Name: mm_metadata_tvshow; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_metadata_tvshow (
    mm_metadata_tvshow_guid uuid NOT NULL,
    mm_metadata_tvshow_name text,
    mm_metadata_tvshow_json jsonb,
    mm_metadata_tvshow_localimage_json jsonb,
    mm_metadata_tvshow_user_json jsonb,
    mm_metadata_media_tvshow_id integer
);


ALTER TABLE public.mm_metadata_tvshow OWNER TO postgres;

--
-- TOC entry 228 (class 1259 OID 410622)
-- Name: mm_notification; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_notification (
    mm_notification_guid uuid NOT NULL,
    mm_notification_text text,
    mm_notification_time timestamp with time zone,
    mm_notification_dismissable boolean
);


ALTER TABLE public.mm_notification OWNER TO postgres;

--
-- TOC entry 229 (class 1259 OID 410628)
-- Name: mm_options_and_status; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_options_and_status (
    mm_options_and_status_guid uuid NOT NULL,
    mm_options_json jsonb,
    mm_status_json jsonb
);


ALTER TABLE public.mm_options_and_status OWNER TO postgres;

--
-- TOC entry 230 (class 1259 OID 410634)
-- Name: mm_radio; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_radio (
    mm_radio_guid uuid NOT NULL,
    mm_radio_name text,
    mm_radio_description text,
    mm_radio_address text,
    mm_radio_active boolean
);


ALTER TABLE public.mm_radio OWNER TO postgres;

--
-- TOC entry 207 (class 1259 OID 410493)
-- Name: mm_software_category; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_software_category (
    mm_software_category_guid uuid NOT NULL,
    mm_software_category_category text
);


ALTER TABLE public.mm_software_category OWNER TO postgres;

--
-- TOC entry 243 (class 1259 OID 410882)
-- Name: mm_software_developer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_software_developer (
    mm_developer_guid uuid NOT NULL,
    mm_developer_name text,
    mm_developer_json jsonb
);


ALTER TABLE public.mm_software_developer OWNER TO postgres;

--
-- TOC entry 244 (class 1259 OID 410891)
-- Name: mm_software_publisher; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_software_publisher (
    mm_publisher_guid uuid NOT NULL,
    mm_publisher_name text,
    mm_publisher_json jsonb
);


ALTER TABLE public.mm_software_publisher OWNER TO postgres;

--
-- TOC entry 233 (class 1259 OID 410652)
-- Name: mm_tv_schedule; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_tv_schedule (
    mm_tv_schedule_id uuid NOT NULL,
    mm_tv_schedule_station_id text,
    mm_tv_schedule_date date,
    mm_tv_schedule_json jsonb
);


ALTER TABLE public.mm_tv_schedule OWNER TO postgres;

--
-- TOC entry 234 (class 1259 OID 410658)
-- Name: mm_tv_schedule_program; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_tv_schedule_program (
    mm_tv_schedule_program_guid uuid NOT NULL,
    mm_tv_schedule_program_id text,
    mm_tv_schedule_program_json jsonb
);


ALTER TABLE public.mm_tv_schedule_program OWNER TO postgres;

--
-- TOC entry 235 (class 1259 OID 410664)
-- Name: mm_tv_stations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_tv_stations (
    mm_tv_stations_id uuid NOT NULL,
    mm_tv_station_name text,
    mm_tv_station_id text,
    mm_tv_station_channel text,
    mm_tv_station_json jsonb,
    mm_tv_station_image text
);


ALTER TABLE public.mm_tv_stations OWNER TO postgres;

--
-- TOC entry 236 (class 1259 OID 410670)
-- Name: mm_user; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_user (
    id integer NOT NULL,
    username text,
    email text,
    password text,
    created_at timestamp with time zone,
    active boolean,
    is_admin boolean,
    user_json jsonb,
    lang text
);


ALTER TABLE public.mm_user OWNER TO postgres;

--
-- TOC entry 237 (class 1259 OID 410676)
-- Name: mm_user_activity; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_user_activity (
    mm_activity_guid uuid NOT NULL,
    mm_activity_name text,
    mm_activity_overview text,
    mm_activity_short_overview text,
    mm_activity_type text,
    mm_activity_item_guid uuid,
    mm_activity_user_guid uuid,
    mm_activity_datecreated timestamp with time zone,
    mm_activity_log_severity text
);


ALTER TABLE public.mm_user_activity OWNER TO postgres;

--
-- TOC entry 238 (class 1259 OID 410682)
-- Name: mm_user_group; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_user_group (
    mm_user_group_guid uuid NOT NULL,
    mm_user_group_name text,
    mm_user_group_description text,
    mm_user_group_rights_json jsonb
);


ALTER TABLE public.mm_user_group OWNER TO postgres;

--
-- TOC entry 239 (class 1259 OID 410688)
-- Name: mm_user_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.mm_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mm_user_id_seq OWNER TO postgres;

--
-- TOC entry 3527 (class 0 OID 0)
-- Dependencies: 239
-- Name: mm_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.mm_user_id_seq OWNED BY public.mm_user.id;


--
-- TOC entry 240 (class 1259 OID 410690)
-- Name: mm_user_profile; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_user_profile (
    mm_user_profile_guid uuid NOT NULL,
    mm_user_profile_name text,
    mm_user_profile_json jsonb
);


ALTER TABLE public.mm_user_profile OWNER TO postgres;

--
-- TOC entry 241 (class 1259 OID 410696)
-- Name: mm_user_queue; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_user_queue (
    mm_user_queue_guid uuid NOT NULL,
    mm_user_queue_name text,
    mm_user_queue_user_id uuid,
    mm_user_queue_media_type_enum smallint
);


ALTER TABLE public.mm_user_queue OWNER TO postgres;

--
-- TOC entry 242 (class 1259 OID 410702)
-- Name: mm_version; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.mm_version (
    mm_version_number integer
);


ALTER TABLE public.mm_version OWNER TO postgres;

--
-- TOC entry 3147 (class 2604 OID 410705)
-- Name: mm_user id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_user ALTER COLUMN id SET DEFAULT nextval('public.mm_user_id_seq'::regclass);


--
-- TOC entry 3478 (class 0 OID 410475)
-- Dependencies: 204
-- Data for Name: mm_cron_jobs; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_cron_jobs (mm_cron_guid, mm_cron_name, mm_cron_description, mm_cron_enabled, mm_cron_schedule_type, mm_cron_last_run, mm_cron_json, mm_cron_schedule_time) FROM stdin;
0d6f545f-2682-4bfd-8d9a-620eaae36690	The Movie Database	Grab updated metadata for movie(s) and TV show(s)	t	Week(s)	1969-12-31 18:00:01-06	{"Type": "Update Metadata", "program": "/mediakraken/async_metadata_themoviedb_updates.py", "route_key": "themoviedb", "exchange_key": "mkque_metadata_ex"}	1
128d11cd-c0c2-44d7-ae16-cf5de96207d7	DB Vacuum	PostgreSQL Vacuum Analyze all tables	t	Week(s)	1969-12-31 18:00:01-06	{"Type": "Cron Run", "program": "/mediakraken/subprogram_postgresql_vacuum.py", "route_key": "mkque", "exchange_key": "mkque_ex"}	1
3da17df9-fae9-4a3a-a70b-5f429d5c1821	Retro game data	Grab updated metadata for retro game(s)	t	Week(s)	1969-12-31 18:00:01-06	{"Type": "Cron Run", "program": "/mediakraken/subprogram_metadata_games.py", "route_key": "mkque", "exchange_key": "mkque_ex"}	1
47cad101-9e87-4596-ba02-2bcea8ce3575	Anime	Match anime via Scudlee and Manami data	t	Week(s)	1969-12-31 18:00:01-06	{"Type": "Anime Xref", "program": "/mediakraken/subprogram_match_anime_id.py", "route_key": "Z", "exchange_key": "mkque_metadata_ex"}	1
9e07954c-26e5-4752-863b-f6142b5f6e6a	Trailer	Download new trailer(s)	t	Day(s)	1969-12-31 18:00:01-06	{"Type": "HDTrailers", "route_key": "mkdownload", "exchange_key": "mkque_download_ex"}	2
c1f8e43d-c657-435c-a6e1-ac296b3bfba9	Sync	Sync and transcode media	t	Hour(s)	1969-12-31 18:00:01-06	{"Type": "Cron Run", "program": "/mediakraken/subprogram_sync.py", "route_key": "mkque", "exchange_key": "mkque_ex"}	1
de374320-56f7-45cd-b42c-9c8147feb81f	Media Scan	Scan for new media	t	Minute(s)	1969-12-31 18:00:01-06	{"Type": "Library Scan", "route_key": "mkque", "exchange_key": "mkque_ex"}	15
631ea52e-2807-4342-8b59-2f8263da0ef2	Collections	Create and update collection(s)	t	Day(s)	1969-12-31 18:00:01-06	{"Type": "Update Collection", "program": "/mediakraken/subprogram_metadata_update_create_collections.py", "route_key": "themoviedb", "exchange_key": "mkque_metadata_ex"}	1
f039f4d3-ec26-491a-a498-60ea2b1f314b	Backup	Backup PostgreSQL DB	t	Day(s)	1969-12-31 18:00:01-06	{"Type": "Cron Run", "program": "/mediakraken/subprogram_postgresql_backup.py", "route_key": "mkque", "exchange_key": "mkque_ex"}	1
f82f2aa4-3b4b-4a78-ab5a-5564c414ab1d	Schedules Direct	Fetch TV schedules from Schedules Direct	t	Day(s)	1969-12-31 18:00:01-06	{"Type": "Update", "program": "/mediakraken/subprogram_schedules_direct_updates.py", "route_key": "schedulesdirect", "exchange_key": "mkque_metadata_ex"}	1
\.


--
-- TOC entry 3482 (class 0 OID 410499)
-- Dependencies: 208
-- Data for Name: mm_game_dedicated_servers; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_game_dedicated_servers (mm_game_server_guid, mm_game_server_name, mm_game_server_json) FROM stdin;
\.


--
-- TOC entry 3479 (class 0 OID 410481)
-- Dependencies: 205
-- Data for Name: mm_hardware_device; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_hardware_device (mm_device_guid, mm_device_json, mm_device_spec_guid) FROM stdin;
\.


--
-- TOC entry 3519 (class 0 OID 417792)
-- Dependencies: 245
-- Data for Name: mm_hardware_manufacturer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_hardware_manufacturer (mm_hardware_manu_guid, mm_hardware_manu_name, mm_hardware_manu_gc_id) FROM stdin;
f8303155-1245-477a-be5a-d17ce1a2c82b	10Moons	1
22c7224b-6de1-4063-9c7f-178828ee2ac1	2Wire	4
376ea42c-4d09-484c-bb81-76a52bfc5d0b	3D Optics	7
eca662b3-1681-4eec-a0f9-c1d9b2526ae8	3M	10
6f92227c-ab31-4d29-9bf9-2048616f4b59	3S Digital	13
95f50e2f-650f-42c9-8f17-5ba019a57942	3view	16
f10da91c-5fb3-4960-9e17-9c4070ef2521	4geek	19
abdfc5d9-768d-4f12-8f32-584f7e60c17f	A-NeuVideo	22
fa6ecf03-68d9-4a4b-96b9-aa5107210913	A-Technology	25
b853373e-25ee-456b-8778-ffad6fc54193	AAXA Technologies	28
96ee8906-b153-42b7-8bae-78829ad6fcf6	AB-Com	31
a85450ee-065b-4843-852b-c5bc9519df75	ABS CBN	34
8fbbb3d2-3a2f-4904-ae13-c07a15f6127a	ABox42	37
3113d819-25f1-4886-8b05-9d2259738815	AC Ryan	40
90fa3635-b9e8-417c-a9cf-a8b94e34e131	ACom Digital	43
e0890159-6fb5-447d-8f8a-3782f2d378fd	ADA	46
ff48298a-fe17-45b4-8ca4-559aa9e25b5c	ADB	49
6522ba7c-e2f1-44ea-ab93-a4822484cb80	AEG	52
7b299187-18fd-4043-8ec1-c888923ff678	AFK	55
e317359a-b52c-4f81-91fb-7f56a5f94f06	AGK Nordic	58
1272c5ef-2e67-4a15-8eb5-b01889775e69	AGPtek	61
863a1678-f2ba-4812-8ea4-94dfbdd513d4	AMC	64
6ae7f72c-d0df-41a2-8d5a-965c7a15e569	AMLogic	67
d4752f21-1efe-4100-8746-573bd56b4d36	AMR	70
d7f34c57-5726-496f-b059-d02b1da634ae	AMTC	73
52485fda-ae91-4605-b656-064fd87bf6df	AMW	76
cd49eaf7-3c8e-4f5f-9b71-df5e61e8b1ee	AOC	79
d27dee11-80a3-45ca-9b60-9e6bd02c642c	AOpen	82
1bda47c2-ea65-4ecd-a10d-c1a3d686f91d	ASDA	85
613ba179-53a6-48f2-8052-f5d18f6f6257	ASRock	88
b677799f-afd4-4d74-b137-2709a3c64de8	ATC	91
fa032c58-ef63-4ade-9e26-71a5096b9756	ATI	94
40a305ac-29c3-4edd-b6bd-1e2914b59c08	ATN	97
92dcd4e8-60e6-45c9-ac73-819ebd4f32e8	AUKEY	100
2056f10f-8eb2-4eaa-ba67-1da5666019e7	AV Tech	103
29b2448d-b300-4ca3-99ac-a96c60fd4639	AVANTEK	106
53811cdf-a164-49ae-bf65-fa6ca2a06864	AVF	109
490b5231-35d7-4564-9375-525852182777	AVI	112
119c4161-fa76-495d-8ced-b9e8041cf55a	AVM	115
0fc5c972-d6d2-42cc-a575-ac80cceae5d4	AVOL	118
4a57e8e7-0a8c-4ff6-a412-0e845f3dca83	AVPro Edge	121
36f9f0b7-dd3b-4ef6-a070-aa3038370c69	AVProConnect	124
45460f55-75df-4183-8555-beb94ce5e28d	AVerMedia	127
5b8eb0d2-49fd-4cae-86c0-02461fd078c7	AVocation	130
94a12c40-f84b-4e64-9e2f-b4e96436a42f	AWA	133
7398e70f-0fd7-4573-aaf0-f68d81744a2a	AX Technology	136
efc5fc44-c49c-47d5-838f-2559ba96342c	AZ America	139
0e602bea-9ffc-4cb6-990b-38edda22f88c	AZbox	142
c626c6ce-94d0-4e22-ae7a-c8e6c5dc243a	Abrahamsen	145
74b511db-4f4a-47bd-aa11-9a088acd7a37	Accele	148
5b635007-4b0e-4b0a-b9aa-3515e4e82502	Accell	151
09a804ee-310e-41a5-817f-511a5acb7a22	Access HD	154
e96c78fa-d210-423e-95b0-98939fbc28f4	Acco Nobo	157
d7015e0c-52c9-48aa-be26-74d0be88311d	Accuphase	160
4028b3c7-60a5-4156-91f5-18d2ecb8d8ef	Accurian	163
aa937834-3e6b-4ec0-92d3-16d4a738534b	Ace	166
d6ce6506-e34e-43bf-87c8-f05f2d281f6e	Acer	169
e642157c-76d7-41e1-bad2-7fbf20c2db32	Acesonic	172
43c96e83-4315-4b10-894a-b9f0a9bab545	Acoustic Energy	175
f79e976f-cf75-4f8c-81d3-770e45b451fb	Acoustic Solutions	178
c77c6a8f-b1ef-46c9-b28b-2400af9978bc	Acurus	181
91ecdd86-56fe-4825-8402-2a3d56dd9f02	Ad Notam	184
8cf2c8b3-7dd2-4707-a1d4-9f1a802a83d8	Adcom	187
b8642fea-2029-4fcc-92f5-1947fb163b2d	Adesso	190
de492d3e-c5c0-4bd2-9e4f-4c36ab34a3b4	Admiral	193
85bc94c7-2106-42c6-a6b8-fd468e949ca2	Advance	196
b7e4a5d5-1012-4d4f-a1dc-3c1801432536	Advance Acoustic	199
df03d6a3-d035-4de6-94ae-6615f846d8fa	Advent	202
3f1a989b-9990-4202-b5e3-0700f5a010f2	Advueu	205
09eb499f-27ec-444d-9ff8-471d2efd3ccd	AeonAir	208
0ff2391a-4d89-41cc-b068-997be522e4ae	AerialBox	211
e538b6e6-670d-46ee-9fa9-a9e1a0ff8540	Aesthetix	214
6dc0491f-d5c7-44aa-915f-c9dd3e31101c	Affinity	217
07ffc5d8-06cf-4629-b3b9-3903efd82bb3	Aftron	220
d0af0235-3e2f-4342-9289-5dc505437d4a	Agath	223
c76f393b-384a-4ec9-bc78-30b5715465b4	AirTV	226
17e9fa8d-0480-4e27-a663-e9920684758f	AirTies	229
9791ef34-a8e9-4424-95cd-2130cde0431c	Airis	232
2c8cb749-a962-4a54-bd8e-ef82af464fa3	Airlink	235
5da13bf5-481c-4774-a692-cfe9f2735e6d	Airsat	238
d99e39e4-26b1-48f4-9cc1-4c954f89c127	Airtel	241
1afbe59c-09f4-42a0-a520-246c0814f2f9	Airties	244
e9479b61-9be7-41f5-85d2-b80987db3f87	Airton	247
afcaa465-aab1-41fb-bcf8-5d3e81d12fee	Aiwa	250
580dbb1b-b888-4445-bc0f-bcd4e0bf696a	Akado	253
e6ae820c-38cd-4c74-8219-097b4ffc97c5	Akai	256
455187b5-4bad-42a8-bfcf-e112d5bba852	Akaso	259
690b8ab1-22cc-4c29-9be9-857206876e86	Akira	262
9341ab23-3f01-4545-b8a4-10537f34bc30	Akura	265
89a4600b-f435-475c-9618-34e805c8ec03	Alba	268
e9b5c079-fec3-4148-8f06-aef2b4634b4f	Albis	271
ad58a4d6-aea9-4a78-acc8-176902fd4267	Alcor	274
021a2402-84f9-4853-86b4-eee794307e55	Alfa View	277
0e6375e4-cd16-4796-a913-94bc36670282	Algolith	280
be192c91-10fa-4253-88b4-499c9090630f	Alice Tv	283
fc7126e1-cdfe-4a80-b1c9-fca4a5c65257	Allegro	286
d162f8f2-40ab-48fc-b492-bff8f8ce63eb	Allen & Heath	289
2f64b34b-bb87-4ca3-802f-20175b89a049	Alma	292
d562c31c-b68f-44b7-a465-a79af9e77e89	Altec Lansing	295
480202dc-059c-4a36-99ee-53c8143e37f5	Altech	298
6fbf0ecb-46b6-42a9-8888-774b8e7655da	Altech UEC	301
2fd1ff3d-8359-4356-8956-e8798ce65e5c	Altek	304
f77dc4ff-0004-437c-9c93-9f856fb2cbb8	Altibox	307
16776f61-8cbe-4adf-81fd-1dedfb5ed1ea	Altice	310
cafd393b-3147-4efe-a455-805c4feec23e	Aluratek	313
964d8e6e-e1c7-48e7-834d-2e53d241b172	Amazon	316
a526f8f2-cbda-4837-a523-f48766c8ddb1	Ambertech	319
c8ca4ca9-d429-4a24-b09a-ba85b79cd004	Amiko	322
ed2fc5bc-b8ba-45fc-bbf4-fe8b759f5dd5	Amino	325
d4877773-2980-421c-9aa1-8486a6e57673	Amitech	328
e2aba888-a8d7-4e4e-b497-74b57c54f4ca	Amphion Mediaworks	331
a49dc49d-c427-479e-96ca-278d9880cda8	Amstrad	334
802b8d92-3eb0-464e-815d-7a20f8646d35	Anadol	337
13c22c28-fe5e-48e1-897b-fcc5d8941e5f	Analog Way	340
dd7aefb7-843e-48f7-8dbf-cad29632aef3	Andersson	343
2bfb1df7-c73f-4cb7-9754-e03fd29b826b	Antelope	346
655e01ef-c2b9-4acb-b7fa-bb36ce378042	Anthem	349
8ef90630-72ca-411a-9ed8-678166a258b8	Antik	352
0d3e2967-9f1e-44b8-827d-8b31e3ae8fd7	Apacer	355
0cac83bd-e687-40b4-8a49-01a238a07d89	Apex Digital	358
4de4f95e-f59b-4e40-a372-d242ab8d906c	Apogee	361
7097ff5e-9a6d-41a0-872a-acf56cc1edd7	Apple	364
dd767c04-2f3a-44e9-b352-e9ddad75488a	Aquario	367
81ea51b1-d4ae-43d1-822e-d556c011acf8	Aquavision	370
4c18f318-d646-4871-b83e-faebdd28e747	Aragon	373
90fba758-3843-4107-96b5-24d3c3ff22b1	Arantia	376
4fbc8b0c-d199-42f1-b8b3-abdce9e58e5f	Arcadyan	379
b14c5b2f-1484-4982-9ba9-f5dc503934bf	Arcam	382
2a9a2a84-5b6d-4955-834b-cde68e8ae80b	Arcelik	385
5fbddad2-acf2-481f-8811-c256d7ee08bd	Archos	388
0eb758c1-00be-4739-8910-f0a0785c9280	Arcon	391
b4ba84eb-72f4-45e3-b2fd-0593c09d2a80	Argon	394
c12744bf-6601-48d5-9460-929ff1000658	Ario	397
6b5382a2-4152-4794-b831-4a9af0d5a931	Arion	400
c9b3e6a5-8b30-444b-9c14-8f98e7a73714	Aristona	403
c03f555a-65a5-49df-8286-42fc79b45ae0	Arrgo	406
ebe0366b-fd30-4747-b8e5-87efc3ff4c64	Arris	409
23709695-ec7c-489c-a7ec-0a14bd1603fa	Artec	412
a33608ad-2daf-4e92-af70-629f10b124a7	Artel	415
b563fadc-a1f0-4684-93e9-444ec9a6035d	Artison	418
c46dcd21-6ffa-4246-a54d-88ce6769ed66	Arvani	421
5a821687-8c2e-418d-bc9a-0516342c10aa	Astar	424
29867f61-1628-458c-b4be-daa57feeed9b	Aston	427
34a85149-358a-4296-ac6f-30606e4174ff	Astone	430
e7451f09-69fa-4c8d-9fb4-3f3260dafa55	Astra	433
b8fd10d0-3029-4bfb-99e6-0c2dd213f057	Astro	436
10c9ba55-7cba-4e7b-81be-7e339f2e44fb	Asus	439
da339161-bd95-4a5a-afab-f73180f24bc8	Atec	442
0fd5c588-7f43-4e28-97af-5e3fb0866642	Atemio	445
b8be4123-878d-4259-a109-1753f7fa397b	Aten	448
b81f4479-4d1c-4099-8d2f-e0ef8918ca38	Atevio	451
09460ecd-4bc3-4c34-be1b-bb96fa769fce	Atlanta DTH	454
d8c280fa-752e-44ce-ac02-aabd25b3e7dd	Atlantic Technology	457
7fbc260d-a932-4580-afd6-63c49aa9f67b	Atlona	460
fbfb4d63-37eb-4c75-8792-b3b431a519c0	Atoll	463
0454e392-ba65-4f2d-90e3-dfd21e71d004	Aton	466
20507268-0de2-44b7-af9d-feca7067b18a	Atto	469
71c43385-d104-47f6-a5ff-512f3a4cd843	Atyme	472
77799a7a-391b-4e67-8dbd-b11f6bf016e2	Audia	475
5754e512-003b-43f5-81f3-ae83e1c0abd7	Audio Analogue	478
3c44d92c-516c-44c0-9d91-bb6c68865a18	Audio Authority	481
8b0f0a53-dda7-48b4-ac6e-57977d6d007a	Audio Control	484
6ba5a8f9-ecbb-4ce2-b1b8-1cb46cc9a928	Audio Pro	487
4f19fd64-88b8-4314-8c20-a9072763de80	Audio Refinement	490
4e488413-3aec-46df-94ee-684b1696c4b2	Audio Research	493
2170de36-e38c-4d2a-a354-b847f1df0c33	Audio Solutions	496
edf1c17a-ac8a-46c0-b66c-3c0d317f130b	AudioSonic	499
87c5677d-7cad-4e72-a751-e2ec2fefc356	AudioSource	502
c7ed87fe-d3f3-44b8-89e8-9540d099e22f	Audioengine	505
bb5efdee-a454-4bc1-a7fe-5ac994b1bd86	Audiola	508
5eed023f-8647-4d8c-9e44-54b67caf0666	Audiolab	511
df140469-7ac8-4705-b24f-1d901eee917b	Audiosonic	514
921c8cce-a0ed-4ed7-b062-5d67c8644df0	Audiovox	517
213e7e07-8641-49a6-b7ca-90fdfce71ca1	Audioxperts	520
49761096-06e6-45a2-b9f0-9247f9d5b5ad	August	523
aa42be57-6c43-4ada-a101-3689f47aa012	Auna	526
0a61ff66-819a-4663-a4dd-9bce49c8f6a6	Auraglow	529
d0811655-7389-4278-b328-7be7ad79a23a	Auralic	532
1813f0e6-3ae2-4f88-b8a2-adb0b0fc6b7d	Auria	535
592c8e7c-1abc-4041-809a-b57786b3c5b1	Austar	538
842021d0-0009-4a44-8403-083b6cd55bea	Auton	541
b1d12b4f-2059-4120-a370-46291e4f20b4	Auvio	544
bca47e33-8505-4c16-9ee4-b75f183b733e	Auvisio	547
42a75c9c-631c-46d2-9126-76bca919d725	Avanit	550
f8ac4011-5161-47fc-b468-b27a00f9839e	Avatec	553
1b8ebbd0-6a2e-40cf-8efe-23e95ebbc8ae	Aver	556
6e5a5641-4b50-4a43-be82-013bb760e70a	AverMedia	559
a9251298-3f55-44fd-bbf0-678c59853a4a	Avera	562
4613b04f-2998-4e92-896e-96c3871a9b90	Avov	565
b6dfad67-07f5-49c3-9023-0cd26c606a57	Axas	568
072b071b-8ad6-4940-8261-e4b1178212fa	Axil	571
8556f8ca-3f78-4c51-abea-85536b71f165	Axion	574
49193e0a-0283-448c-9f91-ec886584b5b4	Axxess	577
d390351e-1e05-4dac-ba75-2eb83d0d1ad1	Aya	580
186a14f7-65c5-4e17-949b-ac9b907b0a9f	Ayon	583
458b2cbc-d35d-4a85-a10d-2aa50a4ebeed	Ayre	586
20bbeae8-de0b-4b6b-bf2a-e038bf809455	AzureWave	589
5be21064-040c-41ee-9681-24635b66de6b	B Tech	592
aae2e8b1-75a6-4835-8e12-a4f38cdcf69d	B&K	595
afee40e4-2228-4d65-af2b-28165bcdf003	B&W	598
1b7d5271-359b-4751-9e9d-e5bd19671d73	BBK Electroinics	601
5a6f4bc8-20de-4eb5-a62c-9b63030439a8	BBK Electronics	604
6519f8fd-1322-46ad-a31f-89c80a54379f	BC Acoustique	607
8278b96e-a13b-4573-adbe-520fc4ae93d6	BGH	610
7af15d34-a6d5-4cd9-906c-e32f60cb6928	BPL	613
0d9246b2-4239-4e72-a295-1fdde925ce3f	BSkyB	616
e247498b-2329-47c5-b4d9-575cd4b8073b	BT Vision	619
499f73d3-252e-419c-ba57-76ca8bc918b9	BTX	622
734a1a79-99af-4c0d-9301-82ab09296f27	BWare	625
5aaa3166-fc1a-493e-83e8-33b225f3fb36	Baff	628
1e908482-3fd0-45a8-99bc-988ccf1cd249	Balanced Audio Technology	631
2ac4b9e4-e833-4861-b72e-eaaf46365059	Balmet	634
25b8a66e-fcea-4183-8340-0253ce5c55cb	Bandridge	637
65c9d3d5-9ad1-4b5c-8e85-7fc38bf490ae	Bang & Olufsen	640
90b0984c-8c8d-4044-907d-7ccad5d34dd6	Barco	643
39934b32-ef68-4ae7-b180-0d7e51b2f63c	Bauer	646
2be17497-2f82-4a02-8d09-2be8c144ae08	Bauhn	649
981287cf-426e-4fa1-aa15-f763d9a6c0f3	Baumann Meyer	652
93e9f5d5-b3a2-4d14-aaac-51356a93bc45	BeSat	655
e412964c-7254-4295-8ea9-3a79c9b6c771	Beamax	658
aa90daaf-7869-4397-bf58-9b39f9030d1b	Beko	661
b4bb9e5f-b9e0-4025-850f-9d1e23cb5cad	Bel Canto	664
ea65d4ef-c9fa-4ad6-ad1c-374d99f110b0	Bel Digital	667
46750ba3-ec86-4b8a-985b-b53c191cf4e6	Belgacom	670
09c3324b-0ecf-4854-a84d-f097b8e9f23b	Belkin	673
52d0292f-b322-4f40-bae2-67cbfc88ce84	Bell	676
1e64a992-e6e2-4bad-b5c7-65f32a1e4e50	Bellagio	679
4a5283d3-67d7-4f4b-baaf-732e16f745d6	Belson	682
9bda7dfd-a9fa-4f50-be5d-099d1397cd4a	BenQ	685
262b8bf8-91ce-4366-8162-a17a971215c9	Benchmark	688
1e3da3c8-7b98-4111-bb86-b71169937ecd	Bernelli	691
4279100e-3438-4deb-94a8-9c51882b9cdc	Best Buy	694
cdb09a28-7752-483c-a301-f33768d6ef6f	Bexa	697
4891e1c1-d79f-4332-b053-65cceb71c967	Beyonwiz	700
f19c3b83-b6dd-4f38-b44a-21caea5bb021	Binary by Snap AV	703
6e08dfac-295c-4a9d-aa31-bcf96fa49f7f	Bionaire	706
56b441ce-bba0-4d2d-a3da-eca6aa4c8337	Black Widow	709
77f8ad00-2c68-469e-9eff-56470462d397	Blade	712
74c29f4c-26e8-4eba-bd68-e79edf7ac6b4	Blaupunkt	715
024b95b7-6d69-4069-afb7-7ab8abdb162b	Blizoo	718
52b71147-b090-4bcb-a4dc-c53bef4e605d	Block	721
80e0b833-e00d-442f-8a67-f852ee6c7b4a	Blu Sens	724
ce0fde47-3b09-4b78-9d7f-892128406ca9	Bluetech	727
6f6a42a6-eb4d-4c35-ae49-36dca87477fd	Blustream	730
a2bc55bf-14e5-41e7-97f5-21c4bb5ee80c	Boca	733
d718397f-340c-43d6-b582-ef7e27140261	Bohm	736
6c9189b5-6740-4e4c-8fdf-b62e52fcd2d8	Bose	739
f421e790-7f69-462b-813e-4b5adabfba52	Boston Acoustics	742
83583554-8bbf-4147-b532-04a9a1a0e2f5	Botech	745
8e66b8c8-576b-4d88-9be6-2b8f4fa0e696	Boulder	748
d50dcd76-40df-4b75-b7a5-8f2a096332a9	Bouygues Telecom	751
969ebc06-8574-40d1-bfab-b8e770efa35e	Bqueel	754
e36fb838-7e66-4166-b866-c7cae052357b	Brandt	757
d60f0087-2b9e-4463-8364-9d6bdad1f253	Brauhn	760
abbb24b0-93a7-47bc-8f81-b57a9d8a89bd	Braun	763
66241408-2f07-4f8b-82be-304ab1d67710	Brennan	766
3e7046f5-d73a-4ec3-adf3-788ee582e855	Brite-View	769
d416f580-11ae-4da6-9514-f47908f066a5	Broksonic	772
d4575a1b-5dbf-4c56-ad50-c3f43663e438	Bryston	775
adcac9b4-134d-4af6-8765-2c38900c53a6	Buffalo	778
4eaac542-6108-4008-aa08-de2a25e41af8	Bulsatcom	781
29a25e33-8cc6-478f-9512-2f4f509f267e	Burmester	784
eb1afe74-92c2-48c1-8e89-42ad26e542f7	Busch-Jaeger	787
f308bac0-4228-4e1a-bb33-7ab966181079	Bush	790
7dcc0ebd-9b18-4b43-9233-56c0c7edbf42	Buttkicker	793
88317143-1c79-447a-bacd-89a7e382ffe2	BuzzTV	796
e92c900b-fdef-4637-82d9-1062e73382dd	Bytecc	799
cb51d292-d249-4bfa-abc2-20836c114e78	CAT	802
cbd2a821-3667-4b8f-9ae2-149e4880650e	CAVS	805
b04f8e87-c29c-4215-b0a2-5fb5f249f959	CCE	808
14b89399-6656-429e-8f00-45b0decc6b5b	CE Labs	811
852fa57b-066c-4706-a930-1fa799a423e2	CE-Link	814
f4972d57-7a7c-4466-ae2e-54e0b3e21dcf	CGV	817
4622d1b4-6f87-45bf-9e1e-7798069c74d9	CLO Systems	820
cd0ed4e0-3c90-4804-b972-b0ee2da1bdfd	CMI	823
c63e3eb6-bed6-4e6c-ba0e-9fb7d7d7b701	CMX	826
e3498301-f461-4240-92f4-135578f66ed6	CNS	829
6aa0877c-b5c5-4439-a732-140c4c6ea2f2	CSL	832
0e79a766-3428-45c8-9cf6-e1d71d721ae8	Cabasse	835
4cf6af2d-75e1-4dd6-b465-3b21d64d5f68	Cable Matters	838
c32cd95f-92af-461c-a046-20996fa64788	Cable One	841
1cd7d592-f99b-4638-a762-14c7884e93c6	Cablemas	844
fabf4875-0166-4bca-bbb7-2a2e5207262a	Cables To Go	847
a5e6a942-9df0-4756-ab93-63361ec59ba3	Cabletech	850
c56e7db3-7c4b-4c0b-90d9-eac2b9876fa3	California Audio Labs	853
f3f8850d-a7f9-4cc0-81a8-fdb393e5c86c	Calrad	856
e16d5b49-cd07-44cd-96ea-b1917adbec7a	Calypso	859
b05f2d2c-593a-4964-be47-b9f27c18adf6	Cambridge Audio	862
0375eedb-4013-4857-b2b9-e460dd9271d3	Cambridge Soundworks	865
4ec53865-7d82-4609-a789-bbb9a87248be	Canal Digital	868
07014f8d-2b4f-4f25-9683-c6d5f07b5ba7	Canal Plus	871
062e3ba7-4cfc-44c5-90e6-1563d509eacd	Canon	874
569b4c22-7ad6-4bb7-b1a0-eb863b766d27	Canton	877
77770b55-4137-4670-987d-b95609ea4df1	Capello	880
bbbb19e5-7b6f-4f54-8ad8-0b53065ff92f	Captive Works	883
ebe46de5-9e26-4a51-8f6d-6d6e05ea77a9	Carrier	886
92f5dded-31b7-4a13-814b-4f1e312b07aa	Carver	889
6b3aa0d4-a4ef-4066-9c98-0192e8b51cc4	Cary Audio	892
81be50c5-1b5a-46e3-9577-e70099346fb7	Casio	895
63c60eed-0f2b-4487-9e6f-3453029ce46a	Cat	898
f1409fa1-315c-4278-8739-66fa3b19c3f8	Cayin	901
dea0f37b-be11-42d3-8f59-1734dc3ec069	Celadon	904
850c0a38-451b-4da8-ad75-cfe8370469e2	Celcus	907
60c5266b-7d3d-4e5c-bc05-b49a12d96f85	Celera	910
2cdc5602-de14-4ea5-9b0b-272bd6c6b5a7	Celexon	913
830905e1-8e30-4d8f-a876-9156d54499c6	Cello	916
4b88c70a-6692-437c-a49b-f296ce9a5776	Centronics	919
73a9955c-044c-4855-abef-b041e1c79359	Ceton	922
2bfb2411-e7ed-40de-8f5a-2e07de009a8a	Challenger	925
5403dab8-bd1f-45da-a9ec-49934c64af5d	Changhong	928
80c4a3fb-cfb7-4824-88ae-422f94a3effc	Channel Master	931
fa4bdef0-5f11-4acc-899f-e64cb055870d	Channel Vision	934
2c8d257d-d3c0-47a3-9de2-ba2e2f95ca7b	Chauvet DJ	937
fbbb8e4a-19d6-47fb-80b8-5a0fe992fdfa	Cheerlink	940
683e026e-7768-49cb-8769-f22ff4eb2c9c	Chief	943
bbd4155e-ccff-47e3-8796-13388ddb19a8	ChinaVision	946
66942e6c-16c0-4ef9-b26c-6fec2253a48a	Chord Electronics	949
3e274486-6ead-4c26-b286-2beb078ee5bc	Christie	952
05979dca-f722-484d-8f79-c9a984e3b53f	Cielo	955
a698119a-54ab-4c14-b542-8e353e0c9e87	Cinemateq	958
d2d896f0-726e-42bb-8a4f-bee33b260114	Cineslide	961
9c96eee9-4cef-4803-9fe9-31a8aacc159c	Cinevision	964
58ad756d-18aa-47fd-8f18-8a3c79ab64d7	Cisco	967
abb66a38-5f01-4d50-825e-2fc7e6c8d20e	Citizen	970
1c7b5f28-6103-437a-9c94-e19a16dacf2f	CityCom	973
37f48e14-3cc0-482b-9a0b-7b2251990bd4	Clare Controls	976
95d6db42-211f-44e8-9895-24493640a65f	Clarke-Tech	979
dca1d7ef-5ec0-43b2-8651-6d725295760e	Classe	982
2aaf6fb3-5744-43d5-8143-a04df8eeb58b	Clatronic	985
94938d64-2d4b-4813-8392-beb9f28cddf0	Clevertouch	988
862825cf-17e6-4793-9eb4-527bba70c860	ClickTronic	991
d550432e-8125-4095-bbdb-5977e54e452a	Clint	994
631fc306-66be-4f11-b8f4-5064de5ddb54	Cloud Ibox	997
1830ab6a-57ee-4a67-a4bf-3fde1b69120c	Cloud Media	1000
eee8a417-21d1-4000-9bca-aa0bb73eeff7	Coboc	1003
bfb92541-6c49-4beb-bd71-a6652be82691	Cobra	1006
55d78f34-fd33-4a11-824b-50667054e870	Coby	1009
ae1763e4-0764-4237-b931-3bf15ac7ab05	Cocktail Audio	1012
a3ac7337-bb32-40cd-8e7b-275da55c4c47	Comag	1015
d5e7a08a-0d24-47a5-a6db-f0ed496bd1fe	Comcast	1018
d41e37ad-968c-40b7-b464-5dc198b00df1	Comigo	1021
fe992bb1-3ad9-4bef-9fba-887b0d3a2273	Commax	1024
1b76af10-34e9-4999-8c32-eab0c846df59	Como Audio	1027
c9388112-fb23-4bf7-9deb-f2e828ac4f00	Compro	1030
1ad748fc-78d6-425b-83f8-55eb05d17907	Conaxsat	1033
60a80db6-14df-4ccb-a177-34a77474383c	Conceptronic	1036
8d19ede8-2474-476f-a372-af622b76f8ff	Concertone	1039
047de32e-edf6-4e4b-b7e4-f273f6e21531	Condor	1042
4273de2f-d1c0-43a8-8cbc-382c80c57c7d	Conia	1045
654a27a2-041d-45d3-aaa8-30ec0bcd939f	ConnectGear	1048
5fff052c-2b5d-416d-8b4a-f24a96b71a4c	Conrac	1051
abf62016-030c-45f7-bef4-3e5c9a6456eb	Contex	1054
a9d16b95-3160-40ec-a757-e278fed4de8d	Continent	1057
ec8c2244-77e6-40d2-8ad6-7e68ec1b3a91	Continental Edison	1060
fdd19cf5-851d-4411-b2c1-0d617f77b2c4	Coocaa	1063
fd674723-a81c-4b17-a925-6c7b8d1d345e	Coolsat	1066
91053a26-3e37-4a5c-8137-b0f09fc737e1	Coolstream	1069
a04f5119-50f4-4871-8193-27f9891cd6de	Copland	1072
4678121f-ae55-4150-a2b2-0876ad1c2fc7	Coship	1075
cd7c0b0a-3708-40a8-9e53-686417b70b8b	Cosmosat	1078
31ab63ed-affa-4515-98f6-04638cd8f752	Cox	1081
12ed737e-c571-45ec-bc1f-2c5df0603246	Craig	1084
dc9e2dfa-c6cd-4535-acf6-d83ed5a96669	Creative Labs	1087
25c3ba69-84ef-49ca-a2e5-bd218b783f49	Creek	1090
bfc6869f-ce63-48fe-ab92-4b5e34e17802	Cristor	1093
01179c0d-b532-4261-8157-d719548eab5f	Crypto	1096
fbfa569d-41d0-479e-a2ad-000847d455ae	Crypton	1099
69704503-0242-410c-b128-36fcb3c4dd35	Crystal Audio	1102
aebce809-f63f-46ac-bbe1-33ed5b17b553	Cube Revo	1105
0bb4cbb7-c5b3-410e-8129-ccb807633fc3	Current	1108
b2e42c25-ad99-464c-9f90-d249e7613868	Current Audio	1111
79ae5b1f-b14e-4b76-9033-1ca0c69378fb	Curtis	1114
7aa38a9f-962e-4f2d-8226-9eb3ddfe6e21	Curtis International	1117
ea93d0b9-ac10-4132-bf53-9cf2a1d1bd05	Curtis Mathes	1120
ec75c391-7b58-4e9d-b8cc-cd03c6b64a03	Cyberhome	1123
10e7d2bd-e9fa-4779-babd-7f0a82189c26	Cyfrowy Polsat	1126
f6f991c6-9dbf-4bb8-8803-1b9d15ce3fe1	Cyp	1129
5c1bc864-cbc2-4e01-97b7-6e382b8a3aef	Cyron	1132
95ce9466-7e61-453f-af94-f0c71d57061e	Cyrus	1135
b3323890-ec9a-433b-a73d-0790897fc2c0	D Smart	1138
0ade5d78-4f67-4dbf-b173-a3a92650ebf7	D-Link	1141
1d80cfb3-11df-42a1-b49a-b1bf8ac72d96	DBox	1144
8117b639-f99f-4202-a03f-d02d4e2df77f	DCS	1147
a2b93989-ea36-486d-970e-2cc1fdce8e35	DEQX	1150
c2546f82-5299-467f-a529-0ea2d7d604e7	DGStation	1153
e6efb927-a5bd-4084-a6f2-f42f31b2a48a	DGTec	1156
feeb79cc-daad-49af-a0fe-3c0343bce64d	DI-Way	1159
954ed06e-dafc-44fe-a66c-de707bd5abf6	DK Digital	1162
d962381c-196c-4cc9-8ed0-295a8653f812	DLO	1165
5a6f9bea-3dca-4597-980a-f853651f060c	DMTech	1168
96bd3559-b374-4fea-b1dc-adb8bd68098c	DMX	1171
e10c89f5-1905-4fa3-a8dc-f882d174f088	DNT	1174
ce9d4d08-15b2-4ff4-9c92-5b6c02ed45d8	DRH	1177
807ebb2f-ea60-4f75-97c5-d058fef9de97	DSE	1180
1ace92ed-a438-421e-8426-6b79bd101699	DSPeaker	1183
ebf86737-f1f3-4aaf-98b3-7f0dd2d9e792	DStv	1186
9498f02f-e9c8-4065-8ef9-d8702877abd5	DTVS	1189
f8df0c4a-6e16-4bc9-a84b-84a2271d6638	DVDO	1192
748b6ebe-b21a-4a6b-b634-762c73a3c510	DVI Gear	1195
55dbb197-86ba-4e66-8fc0-e8b56702e7c1	DWIN	1198
40ec3511-82b3-4dd7-8fe1-7155c4857b52	DXtreme	1201
b43d08cd-9941-42a1-8c62-f132f461a6a0	Da-Lite	1204
23d6553c-d82d-4a37-96ac-f6f58ce3a5cb	Daenyx	1207
1818a697-d2bb-4538-bf95-a83be5261b1b	Daewoo	1210
5beed27b-7c53-48c4-a20d-98a78dc03e72	Daijitsu	1213
7a93d577-feae-46ae-8185-ce4c0bd18cee	Daikin	1216
c8ae097c-f099-4d12-8821-6546ddac2fb2	Dali	1219
4637289f-1e6c-4f52-96cd-df87a576d49f	Danby	1222
24540ae1-c425-4d16-89e8-0153a33e5c38	Dane-Elec	1225
b011b422-2e57-4e90-b29c-c15ea8b33943	Dantax	1228
8ac2fe49-1bb8-411a-a1e9-3dbb17b36fa8	Darbee	1231
4079f295-0828-4364-ad13-45082448ec54	Dark	1234
fc512bec-1c3e-4a2c-a521-9971d6351ad2	Datasat	1237
42112830-cea7-4e1c-b15a-53434cb8088e	Daytek	1240
80a85530-1772-4693-9da6-c657acdbe528	Dayton	1243
ba74790c-38c0-4b2e-9363-f58c573bde6a	Definitive Technology	1246
48d636ef-1bc3-4f47-b078-acebae145d9b	Dell	1249
e698d2d4-8888-40f3-a68e-f17c0540359b	Delonghi	1252
3c1e5266-9857-4f1f-83fd-7d59d57eddf1	Delphi	1255
0928f720-cd93-411f-a16a-545f58e7108f	Delsat	1258
8e20a551-101f-4aca-b89d-b8fdde7dea79	Deltaco	1261
eb4956b5-da4a-40bb-802d-1b690e056aba	Denon	1264
41487328-a68f-455f-aed2-ddc406cb03af	Denson	1267
0a32b532-9420-42d0-809a-52d7e39f692b	Denver	1270
8cafdb3f-92a5-41a3-b0a9-58ac8210db39	Deutsche Telekom	1273
d8b0fede-7859-48db-8c8f-0868c4260961	Devant	1276
5ca4dff6-2373-47c1-aaf9-891d9281ad93	Devialet	1279
66244b98-8a4c-44bb-909f-297d772021e3	Devolo	1282
b7060f49-366c-4162-bba8-c74db66a9d3f	Di-Way	1285
b8ec0e72-4076-4c9d-a6e3-22e200ca070a	DiVinci	1288
a91b59e7-4e70-41c6-a9bc-4a82001db891	Diamond	1291
c5890a29-2400-4137-a002-150c5e724243	Dick Smith	1294
90330ff8-17dd-4699-8fe6-0de4cdc124c4	Dicon	1297
2292b83a-8ed3-4a41-acc4-7ab9f22919ff	Digi-Media	1300
e2784447-dd5c-4f24-9381-dd3d5bc692d8	DigiCrystal	1303
9b290a0d-66e5-4c5c-9355-46049f71e133	Digifusion	1306
7173b2f5-7419-4879-82fb-995ecb6a2089	Digihome	1309
f42e9402-11e7-406a-8459-6da00a1181e3	Digiline	1312
4f5edede-6b1a-4cbd-a3ae-3f81b963ab45	Digimate	1315
46c6a069-30f2-41a8-ae2b-dfa1b013023b	Digimerge	1318
95e820b6-4a21-4e31-9c5c-ba001cd15960	Digiquest	1321
baa1f7da-2dca-45af-b5a6-c1e2ce872e80	Digital Galaxy	1324
aa4fc040-69f4-4b85-8d2a-ae4c9e0d9409	Digital Lifestyles	1327
738aa062-83ad-488d-ae5f-100785742085	Digital Projection	1330
3df7d38d-841d-4b27-ad96-dd61fd5596ac	Digital View	1333
373a7c5f-3167-4246-a68a-500188bd4e80	Digital Watchdog	1336
596d2133-d743-43c9-b5d8-2fcd53d3fa4a	DigitalView	1339
eb5058a8-4849-4153-86e4-27342ef44821	Digitalb	1342
e606f3b9-a42b-482d-8255-449955922eef	Digitalbox	1345
08160eb7-7887-4648-a6be-6c5e4dd980eb	Digitalstream	1348
cc088605-35ec-4832-a1d7-dc2c99420f58	Digitech	1351
9311e8db-91a2-44e8-bf4d-340743236869	Digitel	1354
879a7869-52bf-4744-9691-7b12eb1d1565	Digitex	1357
7800f049-1192-4fff-bcdb-294898fd1806	Digitrex	1360
b0966b7e-3b33-45ac-8210-1cb34ae1d41c	Digitron	1363
bf9053e0-dac5-48a9-9d83-fe1661fbaadc	Digitronic	1366
06736afc-2069-4158-95c9-3cc7ed2f8322	Digiturk	1369
16de716c-690e-4b8e-beed-52294ffd3c5e	Digiview	1372
04c3163b-797e-4220-942d-81fab063c7ce	Digix	1375
e5a5fc3d-e4c1-4ed0-9812-8535e81f1847	Dikom	1378
3b42fc60-be35-442b-80bd-147f56cdac9d	Dilog	1381
96c004c8-c3d6-48ad-a40e-9869a4952871	Dinger	1384
c6f60809-0912-4415-b19c-0f61568ceb4f	Diora	1387
924770e6-43f1-40eb-a7a6-65a27afe1d77	DirecTV	1390
c1d4b1f3-c783-4d26-b080-2b2cc23cc0d0	Directed Electronics	1393
42987400-de5d-410b-86f2-d80ccf01ec82	Dish	1396
761bb4bd-023b-4368-a6bb-486f5b82305d	Dish TV	1399
14d1dc74-4667-4dea-aa6b-33b99756c643	Disney	1402
b2a0fb5e-3296-436d-8830-001f270c5f0f	Dizipia	1405
704b9916-20ec-41a7-9809-e7d9ca84b34e	Docooler	1408
ff62ac3f-4028-4330-8529-85c53352a819	Dolamee	1411
7a1b7f9d-d063-4943-bba4-55848334b600	Dr. HD	1414
f061515b-e328-46ad-910b-921dea17d7d6	Draco	1417
66664f2c-67cc-415f-a4c6-c259e1d47d36	Dragon	1420
be0207dd-554d-49f7-aa50-09464a593dae	Dragonbox	1423
8bab2886-b130-46db-95cc-38a4a9531ffc	Dragonfly	1426
9b19ed1f-c6be-4d95-8289-1fedcf30ec7a	Draper	1429
3c8c7486-72c8-4144-8c7a-d3498928f0c5	Dream Multimedia	1432
078c24ff-4f49-4722-8311-499caba15305	Dream Vision	1435
77433bbd-8105-47ef-b77c-391a66142550	DreamLink	1438
b1538f26-293e-4ce5-8c3e-349f45982b9c	Dreamsky	1441
6c445e28-8a6f-4887-a2f3-02254dad5cd1	DroidBOX	1444
89cc8831-d1ab-41aa-9602-31af7b58b3d9	Dual	1447
020ff0ac-f846-459f-ba35-e1d8f7923d7a	Dune	1450
a7e78ad4-9499-4a16-844f-569bcc6a16e1	Duosat	1453
15e41d1a-f16d-404e-ac2f-5ee37e8f3cfa	Duosat Troy	1456
0d08b2e6-d075-4ebe-825f-128ff26717da	Durabrand	1459
026311ee-8943-46e9-91b0-dcfd39db7f9d	Duronic	1462
b5eb8765-1ff1-400d-a35e-19d199c5d734	Dussun	1465
e9551dcb-b52c-4cb1-af70-1fb1b716a1de	Dvico	1468
d653acb6-ddf7-4d3e-9e4b-52b14eecd34b	Dynamic Mounting	1471
a5968d37-2627-4447-a579-539d605d122b	Dynasat	1474
49e90b9c-cda8-44f0-baed-8b24530f0644	Dynaudio	1477
147b07d9-1662-43ce-b6b3-50673965665f	Dynavox	1480
1d81c6f3-915b-4d97-86ff-f4fc79e8519d	Dynex	1483
c8103f6f-df45-4a00-8413-37c6f65b06a7	Dyon	1486
dcfa71c2-18a2-4685-8e45-cab77dad4230	Dyson	1489
065426bf-e588-4b1d-90b0-c93c7182fbc0	E-SDS	1492
21f3b07d-cb41-4155-900b-bf3c293f81ed	E:Max	1495
26698a17-0a22-43a2-91dc-465bd23b2ac5	EAD	1498
989f225b-399d-466c-87af-4900ff1764a8	ECG	1501
68fffe38-f2ff-4294-b945-e995d6af225d	EKT	1504
27f5b508-ff18-404c-b376-9c6163e90468	ESA	1507
c17c314b-7f4b-4032-997a-564cc37684d5	ESI	1510
5b1a98b1-cd54-4b44-98b9-ceb143cc7ca5	ET PRO	1513
1539eba8-9398-45a0-b2ca-db91ca6e986d	Earthquake	1516
a14a6ee6-a4c7-42d1-868c-9ec3591a29ea	Easy Living	1519
9a1c7794-81ce-4720-9163-cf86e53b0c7e	Easy Touch	1522
cc54d28b-2e4b-4dfc-9a22-50a17f90c3f3	Ecco	1525
8a13a8e5-e7b9-49af-b547-7f291ab32dae	Echosat	1528
5ef4ca12-861d-4453-87f0-e3f5381c0cf6	Echostar	1531
ded23148-9fe8-4ddd-aa2b-7de7e153edfb	Echostar Europe	1534
f0996e16-5a32-46f6-a5c9-013be475e58d	Edgestar	1537
6abe4adc-fdb8-454a-ac64-b8bfbf23c1c6	Edifier	1540
cafe5f0c-324a-4401-b3ca-082675eb21c7	Edision	1543
c57d1efb-003a-4ba4-b24e-9665ef874ebc	EgoIggo	1546
005600f5-e860-447b-b2bb-5793d897853f	Egreat	1549
92eb959e-a4e0-435b-874d-4c87831cb014	Eiki	1552
62bcf0b4-1545-4d10-98ca-cdaed578c076	Eizo	1555
5e058cd4-bdf8-47bb-a2a7-2b8aac1ee920	Elac	1558
b3b1496b-1bd3-49af-9d62-a33b3950a54b	Elan	1561
d5ae723d-649f-4c26-ad26-6dc7a6d37974	Electric Mirror	1564
f685afd6-eeae-4204-aca8-0eca8740ae6b	Electrocompaniet	1567
b38668c8-c102-45d1-900a-22ff54cc4e81	Electrohome	1570
0a3e6614-d667-4dbe-8ba3-74b72ce16241	Electrolux	1573
b679639e-8545-440d-9463-a7216f16ccef	Elekta	1576
ea4d0de8-07c0-4f35-99c9-3bb0896d7c81	Element	1579
d1cc09b2-cfc1-4f76-8eef-94462df20a70	Elemental LED	1582
adb62a02-2296-4730-9e83-97512a3526d7	Elenberg	1585
1b90e617-b305-469f-9283-0662b57826e7	Elgato	1588
2d1cd85f-e41a-4649-ac3f-b1c16a7aa04c	Elite Screens	1591
9facec29-8abd-487e-acbe-78b151798872	Ellion	1594
1f3841c3-9c7e-4235-842d-bad771092edc	Elonex	1597
63e1603e-78bc-429f-acc5-12d261546874	Elsys	1600
7ccd3205-7cfe-429d-a65f-7cb1d56f0746	Elta	1603
2adfba95-56a6-4b36-aea4-e4f5b69121c8	Eltax	1606
9bfab639-8f41-4752-95a2-2fa61e59ea5b	Eltex	1609
a4d272cf-170c-4ac9-819d-0816f9990f72	Ematic	1612
89d309ca-cec3-4aec-b916-1fb440e1e13e	Emerson	1615
94990346-2d4f-4c75-923a-034b57ae255d	Eminent	1618
90881f15-47f8-4cb8-8fdc-8a384cda1167	Emish	1621
55e76c51-81a0-4c17-a40e-b0971bb87674	Emm	1624
8b6ada01-2747-4d94-98b9-65ec73b6ac6c	Emotiva	1627
044c4442-32d1-410a-b1e9-54a70d1ce1d2	Emprex	1630
ddd626b6-605d-4c10-ad2a-20de887abf9d	Emtec	1633
8a12023b-6c89-4c3c-a49d-575fd1e25a02	Energy	1636
ca9993f8-9e7d-40ba-bf19-b8549243c585	Energy Sistem	1639
d1a6f6fe-93fc-40fa-8b96-b63fffe6dda3	Engel	1642
6daac962-31a9-448c-b678-bf8a531781bc	Engel Axil	1645
aa078d1a-66be-4397-9847-e55614595360	Englaon	1648
53bf0e66-babc-4fbc-b51f-dba3185c9665	Enox	1651
b252033d-4bb3-44f9-a272-94eb121a945f	Enseo	1654
54aae06f-f14d-4630-8573-800007acc5e1	Entiveo	1657
f57b81ab-4241-4a8d-9583-112325e7bb63	Entone	1660
33078773-aaec-4d12-aaf8-1bdbae341fb0	Envision	1663
020d2fb8-7972-4ee0-b1aa-bf02fd6b1e6c	Envizen	1666
b5b3807a-434d-4fa9-baaa-d8b158fb62b2	Episode	1669
bc78fee1-cdde-46c3-a10a-fa2433c21b19	Epson	1672
46bd1f3e-32dd-4470-a9da-6d2673079051	Erod	1675
d73836c4-c800-4c57-b4d3-ee0a1e8ca92f	Escient	1678
0f2d7007-4b82-4740-9660-e77a03082f28	EsoSat	1681
79a8648c-d3e3-4e35-8baa-6636e9690068	Esoteric	1684
af5181b0-000d-4280-a173-884ed11053f7	Essential B	1687
8c31e1e9-0f9c-417b-91c4-1c66a7772d7a	Estone	1690
fbc7bee1-3abc-4efd-92cd-2372686ba62e	Estro	1693
4264908a-d013-43fc-8548-bc1adea736dd	Etekcity	1696
d3807f75-16f8-468e-811a-18f5b14a8419	Euroscreen	1699
43cd34e3-8384-4676-929a-940d441f9ac7	Eurostar	1702
2316c73c-7ced-4a48-9f3f-e4e513ccc6eb	Euston	1705
3abec34c-2604-481a-ab60-e2168b788aec	Eutra	1708
da27135b-292e-4efb-8296-5285d8ccbac1	EverFocus	1711
036e2ab4-f79d-40a1-8e6f-e2ce9109500b	Evesham	1714
a2c2947d-6c75-48a0-90c5-694f9bf6a9c9	Eviant	1717
5e731398-06df-4c49-aa2f-52e84938af56	Evolution	1720
289fe71d-d584-4ac3-aead-6373f09232a4	Evolve	1723
de2b7864-d991-42b0-805e-0f79f258cbc6	Evolveo	1726
223bea33-1036-4fb7-a8b0-bd7c53aaa049	Evotel	1729
458b121d-eac5-4690-9e94-9b6df70ae868	Exposure	1732
ff463774-8fda-48d5-8e14-998cc73a1bd5	Exterity	1735
2b737007-f633-4571-987b-d8a6199116fa	Extron	1738
eadf2801-db62-4f90-860c-43333e1c6a92	Eycos	1741
cec72536-3c6c-4291-9a45-53d8956011e7	F&D	1744
7205057e-e480-44ac-89da-419696333e34	F&U	1747
69c15446-5216-4d9a-8089-31a09bdac6c2	FTE Maximal	1750
ec579a8d-5487-4cff-ac6f-34ac8ce865c1	FX Audio	1753
40ea12e5-53bf-4157-82f5-9eaf048d30d6	Fair Mate	1756
a897d7e4-513a-440b-be6e-53693ee7c420	Fantec	1759
c847b21b-55c7-4855-89d5-f69006765201	Faval	1762
d17fc466-5b06-44ab-a2e4-94c197976417	Favi Entertainment	1765
4fd7d7f2-ad13-43e3-bf05-a98797dfe9db	Feller	1768
3a475cc9-daef-4e1d-901f-7a8fcd1506a7	Felston	1771
a17ad3bc-ec3d-47f3-9c8a-334676c65ba4	Ferguson	1774
7ff6bab8-45db-49e0-a184-814ccb017d96	Fetch TV	1777
bd1f551a-439f-40f4-8227-8d532296fff9	Finlux	1780
d2d1a83c-eec6-44f1-adb4-6d782cf74a36	Finnsat	1783
b45b242f-3dd9-40d1-8477-c3e0e6a770c4	Firgelli	1786
37d7a19a-b57e-4253-8ffc-94055bf278a5	Firstline	1789
45f37344-4e8b-4f6d-b8d8-1b4d36a2b866	Fisher	1792
421e952e-c3d5-4dd4-99c4-d815e2e85989	Fluid	1795
ee2582ff-5222-4958-9d10-cbdb44b629bd	Focal	1798
c11504d9-7894-49fa-a16e-af5d6d73d943	FoneStar	1801
86617339-e72a-4a65-8b76-a73d6d0cf7ac	Fonestar	1804
769911ac-0f0d-457d-a061-3a9b4e375e28	Force	1807
8a47e44b-ebd0-4914-a5a8-fb9607070e1e	Formuler	1810
49222b25-8748-4cd4-80e3-018067ad823c	Fortec Star	1813
08760e84-7a9f-43fa-abfd-f31771aa332f	FosPower	1816
03abb62f-715b-4031-9013-64768a5fffb1	Foscam	1819
a9225225-f451-4de0-b8ac-155d460f3479	Fosgate	1822
bb9f8cc4-4e3f-468d-8b2b-a71968fbc24b	Fosmon	1825
c9e9d114-2290-4fdd-8d6b-a97cb6cc9938	Fostex	1828
1e6d8d95-80be-4169-8770-b1872f380c88	Foxtel	1831
cbd178c2-275f-4553-b9aa-6d9081d1e218	Free	1834
e1bbf517-c8b0-4708-bf50-6fbe1c386f99	Freecom	1837
3c546dc9-be12-4369-8340-495f5d8e4131	Freei	1840
0303d622-2484-412e-a5c1-741479fdfca2	Freesat	1843
ce844a66-2873-4bbf-98de-0be07df2c47c	Freesky	1846
26dff8eb-56b1-4f9c-8f03-80fc99dc446a	Friedrich	1849
df70b675-d52a-4c91-b37f-955593cc3d25	Frigidaire	1852
82af7db7-4bf0-48e8-a03e-4bffce288213	Fuba	1855
f2ed2610-cfd3-4396-8374-70d9266a8493	Fujicom	1858
06665a8a-4cb7-4127-9295-0226e662fbd3	Fujionkyo	1861
043278be-24c3-4dc2-af8d-60fb82807dae	Fujitsu	1864
a6cea89c-9bf1-4249-81e4-0abeee44a9be	Funai	1867
382dd230-1cef-4d33-b069-0af2a192ed0e	Furrion	1870
fcdc330d-f514-4217-bd1b-e624b65bdb67	Fusion	1873
511cd217-e9e6-49f6-81a2-b13cd494fad6	Fusion Research	1876
b2b118e8-4c0c-49e5-baff-c2c708d80127	Futronix	1879
acaa7b21-5f6a-4896-9517-c5d902b2aa1d	Future Automation	1882
c4fbf9d5-8287-4d90-bd5d-ddf22fdcac8d	Fyre TV	1885
66dc089c-cf88-428a-82b7-7ecff50fa989	G-Box	1888
ace1f5a8-457b-4be6-957d-d05353d046a1	GFM	1891
08abc820-bf5c-4b8e-9480-26a04fbbc9a6	GPX	1894
a16ba31b-9a54-4989-9dab-3a05df84820d	GVA	1897
194848a6-337c-4267-bdc1-cb02136f44a2	Galaxy Innovations	1900
f3707ae4-30e9-480e-a346-fa04d80e561a	Gateway	1903
9ff9707a-5406-4c5a-92bb-8e07e809aacd	Geant Electronics	1906
d35d1955-6130-4991-8b70-2391b7f91225	Gefen	1909
bc7439f3-4aba-4e6f-bcc6-d77658654034	Geha	1912
370ba786-5857-447f-92d6-476f51014158	General Electric	1915
bc672b5a-d708-4b06-9828-a765596f27d7	General Instrument	1918
17a6de24-fd63-4346-a7af-efef69682bc9	General Satellite	1921
0e96caed-20cd-4796-966f-c8cf8cb47393	Generic	1924
a9dd3826-1fd1-4b44-a424-e3aa64e18825	Genesis	1927
f9c1c7d9-1deb-4c43-ad5a-5a73869cea72	Geneva	1930
6c1a9e27-5460-4980-9479-6b053e5069a7	Geneva Sound	1933
0c43ac50-35d7-410a-902e-0d5d94aa719e	Geniatech	1936
31596282-5861-4a13-9582-c7d2c06cd222	Genius	1939
050f6c9d-c4cd-4446-a88a-7f03779b5d2f	Gericom	1942
f55c2323-b6b4-471b-9068-0b45851f0e23	Gigablue	1945
7ea8005b-e880-4799-97c7-c1fa9ee8ed54	GlobalSat	1948
81726176-36bd-43fd-b07e-c3792aff7674	Globo	1951
da39708e-c497-4f7f-bfbf-0f136a570a8d	Go Video	1954
14fe00f4-af54-4417-844e-700535825a16	GoSat	1957
aa8f5c01-f7d0-418c-b26f-2b1933de863f	GoldMaster	1960
554ec13e-3f70-4645-adc7-1de1a7169549	GoldStar	1963
89d7dfd1-a0dd-4d16-8baa-330a1abb186c	Golden Interstar	1966
709d4b13-da63-46b9-942d-8a45d5b9bb16	Golden Media	1969
cffdc95e-22c9-4992-8637-75ea94cf429d	Goldenote	1972
edfe2d06-a235-4822-ae26-cd3d35b412ec	GooBang Doo	1975
141f4567-e581-4eab-a09e-70a8b3202aae	Goodmans	1978
64a576eb-71ab-4b5c-b17b-fec30034f0f5	Google	1981
6a6a0cf3-e7fa-4912-9c39-12df17666efb	Goronya	1984
fbe433c4-81d6-4519-b4ba-2addcec5b628	Gosonic	1987
7dd02e10-8990-4a65-beb2-a2830897b026	Gospell	1990
aad944ce-641d-4232-8826-3e0c1d8bb993	Grace Digital	1993
dfd8d1ff-9362-4072-b834-cc28fd11584b	Grace Digital Audio	1996
ca4c9264-f257-47ab-ba4f-e96d52710776	Gradiente	1999
c38be451-481c-4614-969b-01c17f184ee1	Gran Prix	2002
718bbcde-f68f-4bcb-8812-1667888da027	Grandbeing	2005
43767b87-4fbb-447f-9f7c-6382f175ec6d	Grandin	2008
2f063040-7458-4e83-9d42-742bca8c5f96	Grandview	2011
07862e4f-8654-4ddb-9cb7-28d6322be2c7	Greatever	2014
838fd1df-52ec-4c9f-b9a9-c688ef869d25	Gree	2017
e2c79e49-fdd7-4073-926c-d17228d0e0e5	Grundig	2020
ab05a026-f3d3-4787-9715-9fb9793f0aba	GuestTek	2023
5a17dad0-2514-4004-8aaa-fc56ceb1d710	HAI	2026
4fd3602d-fdbb-4112-8395-f5e2f69f308b	HB Multimedia	2029
ef921fd8-32d9-4b85-9710-2addda724904	HCT	2032
4372ff92-52ae-473c-b5f1-49fa4521fe6a	HD Juicebox	2035
620fb132-f27e-476b-bfc4-000b1be0848d	HD-Box	2038
8576047a-15ea-41c8-8cf0-66fb82872b15	HDAnywhere	2041
92fd3bc6-e8ba-439c-875f-e863a3f95744	HDT Terrestrial	2044
c18193e0-eeae-41d6-8f29-7c5df54a2690	HDTV Supply	2047
9c3f695a-3378-4f45-825a-19334ba85d96	HDVio	2050
5e07c8b1-0e33-4e53-a95b-ccc5e6c7d5f9	HKC	2053
e4573991-d0c2-4ee7-ade3-436c80255f13	HP	2056
462f1724-43b7-427a-b66d-5e60adb4f4f8	HQ	2059
c2eb9c86-5e7a-4c94-b273-f2e6894e9a95	HTD	2062
f450b27b-3672-4370-806b-1f2693e5c2bf	HTS	2065
72a3e14e-db17-47f9-9d35-fc95a4010bb7	Haier	2068
30cad599-1841-44c2-90d6-67b1c05c2935	Haivision	2071
62221994-6e38-4b58-a08b-b7ae38e133ae	Halcro	2074
0ac65802-dad8-4388-999d-69057fe3cc86	Hama	2077
52b558a1-3039-490f-a03a-463c0e72ca00	Handan	2080
5e8205e1-9677-4026-82b5-c91c88d16eb4	Hannspree	2083
170e764f-164c-4144-84e5-ba16a29b93eb	Hanseatic	2086
747fcf1b-de6f-442e-859e-e9124b6785dc	Hansun	2089
47b7ce32-417b-4e5e-9f31-7a27529b1121	Hantech	2092
ceed0e58-b838-4dcc-adbb-5663f1bb0120	Harbour Breeze	2095
9ff96f54-bdd8-42f6-8def-2a089a3fc51d	Harman Kardon	2098
e6a670fc-a661-4809-9e6e-7479bea37ce7	Hathway	2101
e2af2bb1-4e10-498a-a438-761d481b4cac	Hauppauge	2104
28bc0a5e-d544-4c92-a097-8dcaef760a7f	Haute	2107
4ee5b60a-d463-49ca-a03e-a22776b24c7c	He@d	2110
1707e7d6-e812-4655-9b50-31a36944235d	Healing Digital	2113
4eba538d-8ed8-4130-b198-8dfb3dc06d5d	Hegel	2116
6ef27de6-82a7-4340-8b35-6b5d9fa25b63	Hhusali	2119
9a0d43c8-3db7-4283-b9e3-af6693894589	HiSilikon	2122
c16a2fb3-2891-441b-9218-3804087e07f7	Hicon	2125
ca4493d6-2f41-46df-b480-6dd3d9f164fc	Hikvision	2128
ee8fce8f-048b-438a-8953-d0d637bbf5a9	Hills	2131
de43c45d-0499-409c-b9e6-b3f645f7eb37	Himedia	2134
cf878ae8-bb98-4ca9-8eba-f19c0ad0c8ac	Hiremco	2137
02536387-24e8-41f8-8a68-a05a9a052f6f	Hirschmann	2140
233b1778-f9b7-48cf-8aa8-e5d7c334fec9	Hisense	2143
e9010056-4377-4893-8651-c864986a9b2c	Hitachi	2146
4699f354-3d77-436b-a48f-02fd167bc6d4	Hiteker	2149
e311bd78-6e62-4b8d-92c0-ea982d1f4692	Holmes	2152
f0f0bac8-6f4a-49e4-84f1-d084bd855c9e	Homecast	2155
fe3b1f5f-3b08-4481-ade5-b147b23fe216	Homeworx	2158
efbf5057-e0ff-4d26-8337-d265845f921c	Honeywell	2161
aef7e77c-f81c-4634-b874-8b8e6a897337	Horizon	2164
d57c5465-dec1-4203-b72a-446c224bfebc	Hot	2167
69a5260a-94d1-4364-bece-92118dd99fc9	Hualu	2170
a157312f-a088-4636-b4bb-e89476c46ed1	Huawei	2173
219345cd-a82e-4099-b1de-b8e22c967c64	Humax	2176
69d64bfc-1694-4d7c-9aeb-b7f0ee97c3ec	Hunter Douglas/ESI	2179
a958fca5-bfc7-482a-889e-70d4957c4a9a	Hybroad	2182
a5a96b95-42bb-4437-86c6-4a19c11d701f	Hyperion	2185
c7f67f4c-4fe0-46a6-a90f-8305838f8fc6	Hypex	2188
2912f6bb-c121-42c2-bc8f-c44767bda6a8	Hyundai	2191
552819d7-a691-40c1-855e-8411fca77629	I Ball	2194
0f9b9206-fbbc-4a3c-bd08-8d22917060d7	I Cable	2197
1966ae06-3d3e-4c5e-8b11-ca0b0d4d4b70	I Com	2200
00aac242-7679-404c-966d-945ecbfd3e2f	I Home	2203
267aa373-9674-4c9d-8326-c9f0ef0abfcc	I Live	2206
50cbc12a-74d7-4e77-be51-52eeaeeac305	I Luv	2209
6a4f44d7-ba57-4386-aa08-e7e4a99f3e64	I Net	2212
50cb2a23-809c-494b-a1ce-47805939a407	I O Data	2215
0298b869-5f56-49cb-aa26-3c713e862aa2	I Set	2218
c431a99d-b512-4a76-85d3-54fee09c9671	I Symphony	2221
e9dcc7cd-e332-48c5-8bc2-93dfd63041df	IBahn	2224
41577a6b-4fae-4ce6-aa9b-8bf762b4c53b	IDdigital	2227
ec348462-bcb2-4c85-946e-1fa3aaefd076	ILIFE	2230
c3966a2f-4810-46b6-a6e4-3c14f6e3220a	INNO-HIT	2233
a6a97729-c8d2-48b4-b96f-2783f653e142	IOGear	2236
81a8066e-c0e3-40fd-bfa6-dabe8aa3c419	IR2BT	2239
c1f370be-7e31-4716-8c6c-d3664c4fef94	IRRadio	2242
d3cd67c8-f0a4-4863-a0f8-c2cbb3b4c00a	IRobot	2245
e14d04af-6ab9-44c8-8fc7-b626e1a4642f	IView	2248
6bd3f9dc-42b6-431a-a09a-3cf9b9472347	Icecrypt	2251
547fc4d6-46f3-4580-ac7d-06ec4b9b93cd	Idroidnation	2254
b56d80d2-4643-4b9b-a1ad-711f6511cf7c	Igloo	2257
8e05b280-c148-416c-8957-e9d6eeb62d42	Ikea	2260
8fb171f3-fdff-4aa5-9350-522382f358e5	Ikon	2263
7836f0bc-52e5-44f9-8e23-1813a882a2f4	Ilo	2266
ae62fa19-686d-4164-9143-0782d675dd0a	Imerge	2269
7a38e1c2-e5df-47fe-a219-019056898208	Impact Acoustics	2272
635f6a32-5622-4e17-9d51-9e3d45fba200	InFocus	2275
fdded312-e83e-4c41-849a-336a224701e1	InLine	2278
d4e421df-8bb7-4a80-8dcf-2407b22dbcdf	IncrediSonic	2281
212499d6-59b8-4883-bd2c-4c1a701e698d	Inday	2284
0fe72ee0-271d-4e19-b30d-d0483928c4b8	Infinity	2287
55be4cc4-94a8-4200-a2dd-bd0a85b4c0c9	Infomir	2290
9c2fdeaa-4d60-48f9-b697-782025f5a74e	Initial	2293
e48638f2-782b-4125-8455-4db13955f441	Innovative Technology	2296
5ba9ffdb-53f2-44bb-ab03-c6924210b3f1	Insight	2299
da1a8b9a-9afb-4d07-b620-95c5d1bb7612	Insignia	2302
c416101f-d216-4aff-8b70-947566d62b74	Insteon	2305
e7e68f02-d094-4a64-92e6-b749bc8ade7b	Intec	2308
770a8ccc-0a46-4025-8d8f-09dfbf8d6ddd	Integra	2311
7d2201a1-419f-4f2f-a412-5bd1cc6baf44	Intek	2314
5e69bb3f-68dc-431a-a43c-20e761a123c7	Intelix	2317
29505126-9e66-44f2-aa1e-729287921e23	Inverto	2320
5d13f72a-54e5-41e7-9a8a-bb5660b40385	Invex	2323
bd320c86-959a-4d41-8e1f-4829b84d5d86	Iomega	2326
dfc81e08-28c7-47c7-9424-af7a71d01d96	Iris	2329
b88516e4-3692-4800-915a-8852683195b8	Istar	2332
bd8ff26c-268d-4328-8fa8-b78c2131437d	J-Tech Digital	2335
3ff89d93-b2cd-45da-b3e7-d8e7166e5ddb	JBL	2338
ef4dc4f7-3ec4-4d34-8c53-6d315770121e	JBL Synthesis	2341
66a0942b-f2ae-4a07-9abe-14e74ea7576d	JBL Systhesis	2344
6d0875d2-3e0b-419c-a9ad-221e100a72a5	JVC	2347
49ece59e-bc5c-4ca6-9fa0-652c90587bcc	JWin	2350
567dc9ac-8271-47b5-a5f8-1cde4beb2ad6	Jadoo	2353
d749b28c-97ce-4132-8921-cf7d9f396dc5	Jaeger	2356
9b622ef0-e17c-46c5-be58-b8327bb6a82d	James Loudspeaker	2359
e5c7c02a-6807-41c5-ac2d-f661fba6732f	Jamo	2362
d404dd2c-d7be-4167-9514-3bfc0026bfc3	Jarre	2365
eed06ca1-9a82-4017-8768-8a243e65fb63	Jay-tech	2368
cdbc8d61-1a99-496f-9fae-11bb92389288	Jeff Rowland	2371
a3d12eb5-51ec-4211-b50c-ff948bec0b31	Jensen	2374
035afe2a-32d2-488c-b572-faa51ed206ea	Jepssen	2377
c5e4c49d-352f-4fac-9e37-fcbbf329ac88	JetStream	2380
0a1720d5-6db2-4f45-ac3c-ecfa92ad56a5	Jiuzhoutech	2383
9ab0c286-b181-4bba-857f-4e6205b463d7	Jolida	2386
ce7551de-31a1-43dc-b9b7-b4209b6e38b0	Joytech	2389
72bc42b4-2faa-4a4c-914f-6907c6dfc8d2	Jynxbox	2392
c4747da0-e264-46fb-9bd6-79ec78cf2182	KCPI	2395
bc732270-9919-4568-9d25-80fdd214ebae	KDLinks	2398
eb3ae3f9-f83c-4963-9e4f-d6c9accd8753	KEF	2401
dbd2de98-77d6-486e-9900-c745d020ae86	KLH	2404
2c95cad4-e580-4e18-afaf-b1746eb6b966	KPN	2407
66f60221-9b5f-4c37-a49d-adc5fb8592f4	KWorld	2410
8e8fa51c-0818-499f-94a9-21ea2c8d0e66	Kaiser Baas	2413
861fa94a-b5d2-4853-9b49-011e1e5dfd07	Kaleidescape	2416
8c06fbbb-c23e-4214-87e2-eb9c0b7c82f2	KanexPro	2419
f4ceef01-be46-476b-9757-779b3d50e3bb	Kanto	2422
fd5e682d-621c-46e3-95f5-4774ed4c509e	Kaon	2425
1206474c-d044-43a0-b38e-b7fd2272ed77	Kathrein	2428
d5db9716-f42f-4718-8637-12b59353afb1	Kawasaki	2431
6a6e4fad-492d-4797-b15a-b603c41bae7d	Ken Brown	2434
3e441b7d-101f-475e-bc74-377656787c17	Kendo	2437
aef569ce-c4a3-4998-bb6c-9279ed819a8a	Kenmore	2440
2bacf874-4767-4a46-93c9-867cc6e04ad4	Kennex	2443
b68824d0-037c-49f6-9ac7-f46f21de6158	Kenwood	2446
3bbd3899-5e90-4616-8da3-d7fd6a0c2ad7	Kenwood Sovereign	2449
c58926ed-8ee2-4762-94ed-9564d0a8d1e3	Key Digital	2452
c8dc0469-0d8a-463f-9aab-384e4d1b5f91	Kicker	2455
3e780bb8-2980-424f-b51b-33fb2348cde7	Kids Station	2458
093f74a9-3b3a-4216-88da-28fb58cb2a6e	Kinivo	2461
5c8b2021-5685-4dd2-aced-0c1152f57387	Kinyo	2464
2809649c-380d-41af-a614-4cc09b2ec8a7	Kirsch	2467
cd8d1267-85e1-42c6-8be1-a7c03e79f399	Klipsch	2470
eebf6122-1224-4dff-ae1d-48a6f758623e	Knoll	2473
60e87b2b-d253-40a0-ba5f-5e10791ae5bf	Koda	2476
b8340232-1012-4c80-b9e7-c264f76803d2	Kodi	2479
0c31d9a8-5efa-40f6-9b76-3599605cc513	Kogan	2482
1aac9eb0-64fe-48f5-8ba5-1116ba276da4	Kolin	2485
2cda7fbe-1ad6-4a8e-8ebb-357a512743ca	Konig	2488
7e8a3f0b-5add-4357-b60b-efd940a29a8b	Konka	2491
98135db9-dd88-4f90-b0bf-4fbd8369cf92	Kora	2494
57fbb306-6762-4cdd-8e30-3694cdea950c	Koramzi	2497
e6051267-62f8-447c-bac5-2df4291a33a2	Korax	2500
66a5a0eb-9ef8-4502-9f65-338c75faf8aa	Korsun	2503
ec44a6e3-d196-4bdd-83d9-d46fe181a337	Koscom	2506
9ab72348-0a41-466e-b2da-158ed7a8b8b2	Koss	2509
42060752-c83f-4c52-970d-7953383ba3bf	Kreiling	2512
a2039829-350e-4353-8356-6a28b9666aa4	Krell	2515
6b9ecb88-26ef-4812-b304-999a5cd77c9b	Kyocera	2518
eb2ab218-23fc-41ed-8856-34e49df91146	Kyostar	2521
686b2f9c-0792-4ab0-91b0-36a58e631521	LG	2524
616702e3-8269-479f-8209-4dffb66f977e	La Digital	2527
6a0acfaa-b4e8-4722-a889-db971fdf802d	LaCie	2530
5114ef36-3269-45a8-b0f1-a1c5c3cb86fc	Laser	2533
7b982f7a-044e-4bf0-8ac7-d36225dc6287	Lasko	2536
1640676a-b245-4305-94ee-b4b23fa04f13	Layer3	2539
7b0ea0ba-8d2b-4676-9ddd-97d469d79ff8	LeEco	2542
14141a96-23bf-4b61-8efd-49f9c58964c5	LeTV	2545
dc7d4b83-d12a-4aa4-9135-7a57aa1c1fff	Leelbox	2548
6378e910-18e5-436a-97b1-ff38ec3fb882	Legend	2551
f6300dfd-1f53-4e6d-91df-5e5feec0e013	Lemon	2554
37c9bdcc-9cd6-44a2-86bc-142a9ebf1e03	Lenco	2557
c0495d69-f073-430b-9fa5-c8373ad917c9	Lennox	2560
cdb17c9d-7ddf-47a9-8720-694ac67c201a	Lenovo	2563
35321e6d-54ca-43c6-afa1-aa8f733b9d25	Lenoxx	2566
94d94c3b-464e-4a7f-aba4-fe8990c7750b	Lenuss	2569
413fbbf1-626c-4f07-afd3-f26be096abc3	Leon Speakers	2572
d8100556-c1b5-4fae-bc33-bc6c19e3537f	Level	2575
7a7fa027-9b5e-498c-a45c-663b3d5d2f1c	Leviton	2578
80a7a2cc-3be5-4479-b8be-4f145478afc3	Lexicon	2581
82b3fc6f-c05a-4f2b-8375-405ad8102e8c	Lexium	2584
dca06da5-1565-4882-a2c1-2cab6b487c5b	Lexsor	2587
d936dc9c-793e-4a0e-aec7-5b97b23b1388	Lextron	2590
7ab9f91d-19f5-4630-bbe2-b05a8b8ab67c	Lexuzbox	2593
8bff1487-ef83-410f-92b2-54c5902c4c5e	Liesegang	2596
8e5b6813-71c6-488b-908e-745fdc6d774c	LifeSize	2599
075015e1-6b0e-4adf-a0e8-edaff8ac1975	Lifetec	2602
02557f9c-4c0c-4533-9bc4-b4d4eab72af6	Lift It	2605
022cbfeb-a7cb-40be-8dea-7859ebbad21f	Ligawo	2608
7fbb1e9c-a6fe-4184-870d-2577a5b44303	Lilliput	2611
8ac04a91-1e5d-4fff-a703-551591905d5c	Limesat	2614
006559bc-6e49-456c-a06a-8b8800fafb4a	Lindy	2617
6321bf69-1353-4204-99df-0f247f66c46a	Linkbox	2620
2cb9081a-2168-46c3-9488-b4ab1e5c183e	Linksys	2623
03b395fc-a56c-4338-8d4d-7c43688d2d37	Linn	2626
51ffb967-4b8e-4562-a2a5-081f7140cf0a	Lite Touch	2629
6ca50a8b-5fb7-4b82-874d-5c45ab9cef71	LiteOn	2632
477a2be4-a76d-45d5-a566-73d14a829c1f	Locatel	2635
2fc6127e-56dc-4199-a538-a8c88ad618c3	Loewe	2638
2d29a28f-f23b-4dc4-b042-bc4e94165463	LogiSat	2641
bf941df7-3e45-44e1-9ffa-84c78e4f2bda	Logic3	2644
ac483819-6912-4a62-bf14-921fcfecc0bb	Logik	2647
8bc7f642-fa0f-4e6a-b7bd-12375a8eb79b	Logisat	2650
dfdbc0e2-a4c3-4e48-80fd-d31369943f52	Logitech	2653
bad70845-1a02-45e4-b98f-85127fc1ce71	Loranz	2656
21768e6e-00d9-4902-a4b3-92ed3ba185d5	Lugulake	2659
a9b3ee96-4b29-4366-ba67-ae707b14a209	Luma	2662
f4a10435-2730-4ef4-bf6d-d790b33580f9	Lumagen	2665
65f7cdb1-8fc8-4091-9dcc-df1717bbf3ba	Lumax	2668
8e5e6939-bf08-4b3a-9530-5ce4adcd4249	Lumene	2671
28360b2e-4cc8-4a84-a337-b7b761a681b9	Luminara	2674
d3e7b3ee-b635-4e24-9deb-a56eadb10d82	Lutron	2677
1d3c29d4-429a-4970-9558-dcc66af4db16	Lux Technologies	2680
89f6d351-6b1a-42e7-ab2d-36a3866b0374	Luxman	2683
ef22ad04-3703-46d5-bdfd-0ef38f638298	Luxor	2686
75787098-66d0-4e2f-a086-674da333f82e	Lyngdorf Audio	2689
4be4b440-4df0-4d78-ab6c-42cfead2aa21	M7	2692
33b33d3b-204e-4dab-82e1-2ad0e7dd2b5d	MAG	2695
3f24996e-de52-4b7d-8011-c5b55cf96fee	MBL	2698
9b7ef853-4c86-4139-b72c-c9a39dd3a843	MBX	2701
e31b06e0-24f9-4ca7-a833-66b084d85400	MBox	2704
2d2e9be9-c3c7-4564-bb39-a589222c08ed	MECOOL	2707
eee4a583-26d3-4711-becb-8ad1e9b67371	MGTS	2710
dcbbdeaa-db5f-497b-a6e6-611fd33d8a26	MJ Acoustics	2713
4e9a4ac0-d299-45d3-870e-6a49f7e42553	MK-Digital	2716
480b6faa-c440-4bc8-a45d-534af8b2479a	MPMan	2719
c33493a6-d8ed-4a2d-be18-cc7e2242ca09	MS-Tech	2722
640a9e84-c48d-48a9-bbaa-9d4c948252a8	MT Logic	2725
5e1dd1ce-1af4-48ad-a217-12ca3a647cd6	MTS	2728
0330832a-4a55-44a3-94b4-e3783c88c643	MVision	2731
f1cb7d47-35dd-465f-af63-5f94731d5100	MXQ	2734
0871565c-8629-4502-b509-23fd5f0bcb48	Maax TV	2737
0fbc39cc-182c-4230-a344-a70324c38a5e	Macrosystem	2740
5a079a69-f539-4bae-ba28-f9304e10656d	MagiNet	2743
0439ccea-7cfd-465f-b33c-b74a954b8021	Magnasonic	2746
bc13d537-9b27-414a-a02a-9891afac25fa	Magnat	2749
31f329ee-4407-47c0-902c-a16b2feec449	Magnavox	2752
b2a896ae-635e-468c-a5d4-ee6865f166a6	Majestic	2755
81eae7ce-2696-4cdc-a892-7bde7a454c17	Makita	2758
19ea2933-b403-4496-8f67-6d5df94d1921	Manhattan	2761
8f24275a-1d52-45d4-9482-a0c481f35aef	Manta	2764
46a6bb78-8fbf-4913-b365-d98799efa75b	MantelMount	2767
6ebf0e72-66d5-41b9-b8fe-8394e23b5672	Marantz	2770
a1dfc3d8-9aec-4a07-8eb3-0c5acc190d1a	Mark Levinson	2773
2260884f-e762-490f-bbfe-8d8a7d295a35	Marmitek	2776
2b2e8b3f-9b48-4614-b860-ca0785350043	Marquant	2779
4db85044-612e-4195-b16e-996c03d6885c	Martin Logan	2782
b67ecf27-40eb-4c45-af0a-ea01888bd646	Marusys	2785
a5b91675-fcdf-4c60-9955-018d855866c9	Mascom	2788
97e669ae-b254-4795-8d8a-8d8576af54b5	Masscool	2791
03707ee2-bd96-4db5-9367-4e4a1808f099	Masterpiece Communications	2794
81d039bd-2165-4f2c-a931-3a1b937147b5	Matchmaster	2797
cdd87f54-51ef-4428-aee8-0c39f05f0f78	Matricom	2800
e58c2c36-d237-45c4-a69d-ca3769c29481	Matsui	2803
69bcecd7-5589-4cd5-a2a8-d46af42581bc	Maxdigital	2806
a0175a4a-9223-4751-9a53-2f24af998127	Maxell	2809
2ccd0ea2-34df-4964-8f5b-9615d278b420	Maxent	2812
aadabf9f-d604-4a84-8e05-f6dad5845cee	Maxim	2815
97a0b121-6512-427b-9033-e7ea2063aeb0	Maximum	2818
2a15643e-cd0a-4e06-90b2-c0096af70333	Maxx	2821
d7b5a7ec-7a69-4643-a46d-d771866a30dc	Maytag	2824
3b256d85-0ad4-41e7-87c5-b5825a2fcae3	Mbeat	2827
d01c14f3-3e33-4b6a-a667-a8503b96d617	McIntosh	2830
1800324d-8652-45b1-a38b-8bbe07389868	McQuay	2833
a5d9def1-a05a-475d-84b0-e62b50e21ebc	Mede8er	2836
60a9db65-ac99-47a6-b6a0-807674692c04	MediaCom	2839
c8d44dea-b417-4cc5-89a8-f090da319a58	MediaLink	2842
f54c1167-56b3-47e0-be09-02fc3977c0b6	Mediabridge	2845
1fc36ada-d469-4f2c-ba6e-5741d04de0d7	Mediasonic	2848
d88342c9-e2d9-4a2b-bffc-a50d1047acf5	Medion	2851
c1ab1b3b-d8b4-419c-907d-192b397afdf2	Megabox	2854
16e45e47-c02e-479b-8316-6199f2712006	Megasat	2857
e38b6420-aad1-45e2-a425-3530b5d876d9	Memorex	2860
44be6d33-a443-457c-8cc9-25dc68fbb278	Memup	2863
f5e19788-c068-4635-b9cd-c0e74e265510	Meo Box	2866
53c8b379-14c9-433a-ab0e-9bb6d1b338eb	Meridian	2869
e74f0311-3072-4e6e-a746-7305229d4d23	Metra Home Theater	2872
c045c0f4-ea99-48f2-92dd-acc464d38fbf	Metronic	2875
af5ab847-ea7b-45f8-9c98-21624a50bea0	Metz	2878
c9a062bf-725e-4e7d-8445-b5475261300a	Micca	2881
d96a9434-721d-43a4-b443-a0b65575ef86	Micomsoft	2884
8c3038ae-8ac3-4c25-86a0-2026d8f73de1	Micro	2887
72719b29-163a-4e99-a31a-bda124e1811d	Microlab	2890
df05d00c-ffc9-4a45-9ac5-0e0967d8d4cb	Micromax	2893
88b4abb8-937a-41af-b56b-3a62e0cbed7f	Micromega	2896
c4997424-e30b-4fa5-b732-e6649a59f26c	Microsoft	2899
25f3c865-bb86-4e8b-ba44-1c51fbf4e5c4	Midte	2902
48837957-8076-43df-b41d-1971e5c86d3e	Ministry of Sound	2905
2bcf8397-5f37-4bee-8c90-e82c0d015263	Minix	2908
e7c3adc4-a77b-4856-b779-b703afb3ca5f	Mint	2911
c5f3f1d8-01ac-47dd-9892-785b70ec0f5d	Mintek	2914
f89ad234-8803-4bf7-a109-eb8163b80ce9	Miraclebox	2917
ff6233ab-0e6c-45de-ba0f-ba62c33879ac	Mirai	2920
abef83e6-b2e8-445f-b6aa-9fafbc756a8d	MirrorMedia	2923
3967b657-6f75-4f6d-acb8-4439630ed945	Mistral	2926
1e9a2fd0-a289-484c-97ca-f7db8c0279b4	Mitsai	2929
264acdb2-bf8b-4c97-834f-e60fe03d3825	Mitsubishi	2932
93b9dc04-fd47-44d4-a659-cc9e5e7215a9	Mitsui	2935
5a9faf25-24b8-4fed-8473-de2f08365bc9	Mivar	2938
26c5b19e-f18a-4eac-b648-296f456b0d2d	Monitor Audio	2941
4234cc31-8b39-4b3b-aa96-a4c910a551b6	Monoprice	2944
90892828-442f-4fcc-90f2-8a23a2f52e26	Monueal	2947
96ba1b71-9058-450b-8248-b6cbbcaa6657	Moome	2950
41ed45cc-b5ff-4a45-b6cf-2885552791da	Moteck	2953
e2251dfb-7028-4d09-bfe6-92c019521f26	Motorola	2956
8cbe04f2-1e4c-4807-b638-72306ee2d0cc	Movielink	2959
d73d1f33-2ba3-4965-96bb-3e2b1005c5ff	Moxi	2962
cdbb8b16-cddc-4b63-ae4e-193d603650b5	Mozaex	2965
9d28ee09-3afb-43d0-9a1a-e91070e5b76e	Multichoice	2968
cdee13c3-f8da-4a8b-99ee-4da70ee9f600	Multimedia Polska	2971
b45e925e-ad34-4b5e-a080-03bba0cbd352	Musical Fidelity	2974
10a063a6-7239-48be-90a8-5ccc45e822c6	Mustang	2977
9603c4e8-19b8-4450-8a02-216ac3ff6103	Mustek	2980
f33add9a-11f5-48c5-ba17-09bae2ffc362	Mutant	2983
b46587a1-ee3a-49b1-8dba-19e592f52823	Muvid	2986
c3b0f8c3-f088-48cb-9534-793c667175dc	Mvix	2989
80f3ea43-de6c-4759-9d89-b33db035dc53	My Sky	2992
9d8b4814-95c5-4463-9988-8d2cecb36b60	MyGica	2995
0346d33b-fa23-403d-aca8-5febab1a7d94	Myryad	2998
300e4486-5e79-4ccc-b782-acb6cad4d6b7	Mystery	3001
6fc8c3a9-007b-4492-8f24-9868ca571890	NAD	3004
f870244c-055b-4902-a57e-4e5f8af736ff	NEC	3007
7dc42a0b-8ca6-40c3-8945-619db9d5f1d8	NFusion	3010
2b6e5110-c4f0-4a3e-bd7f-6c09659ed0fb	NHT	3013
493be6d9-d233-4dc8-a4ee-812d33eb5238	NPG	3016
91798927-1649-4406-9181-f7dc62963de0	NTV-Plus	3019
d9f9f751-d381-420a-9912-d0363c6f9dfc	Nabo	3022
851c3955-7130-4540-930d-875c8aacbaf7	Nagra	3025
7fdf8486-ef67-4ac0-b4f4-bf8908339149	Nagravision	3028
0134e5e7-2904-4e7d-8ca6-c8d5f9a2510e	Naim	3031
d74ea745-4042-474b-b58e-a1b8f91deed3	Nakamichi	3034
aae8044d-1786-43fa-a1b0-041b20fefcc0	NanoXX	3037
efe71cb5-2be3-4df8-87f3-3d0ad8aa9010	Nanosat	3040
a8d1280c-1259-4946-987e-b4d3602178bb	Naxa	3043
88135b2e-893a-4e1e-82ad-99a637f77da6	Naxoo	3046
4916d0b0-f7a5-43a1-a803-e2d9167a018b	Nazabox	3049
e2d17691-0062-4abe-a74a-2768200fb003	Neet	3052
2bd61e21-9fb3-429b-9709-63179dadcd19	Neli	3055
e51cc014-b598-44a1-8ddb-b602af3c31bc	NeoPro	3058
da8ad805-c57f-4d8d-a220-2c7d6474e060	Neon	3061
02e241e3-be9f-4d9a-9ecd-abffa30e3447	Neotion	3064
9bf9118b-85e4-4fcd-9950-dc097063f65b	Net Digital	3067
e3b6e609-946c-4951-a771-f52dc6a00419	Net Line	3070
906156d9-19a8-480b-a5ca-bac898f868e5	Neta	3073
e8612862-d191-4679-a72d-47b888c3532b	Netbox	3076
e7a8ba77-e9dc-475a-bad2-12078be47d21	Netgear	3079
aaaf7923-fc11-47ab-b440-a3495cc2ef87	Netgem	3082
943da796-5689-4809-9fb7-f03a0a4550bc	Netstreams	3085
5093c5e1-ae96-4e88-829a-7624fb1846c0	Neusat	3088
1c073a1f-7c38-4437-8e63-4f5a5413edf4	New Line	3091
b7742952-b158-4e64-904e-1b44960e8e44	Newland	3094
0edd3e2e-159b-4d7a-9c6d-6450d08d2f5b	Nexbox	3097
51c8dba6-99f4-43d5-b994-2af2e0e3689e	Next	3100
e7353f43-cafe-4193-b492-afaf031b51e7	NextWave	3103
e900c27f-38eb-4107-b92c-64808f452a31	Nextstar	3106
0567ef59-25d5-49f3-bf68-2e93f7b98279	Nexus	3109
00644a5a-ae39-48cc-bc2d-cad5e6cb9179	NexxTech	3112
86538afa-1263-450e-b372-941e0761957f	Nikai	3115
64fa51b9-e5e6-4bfd-ae3c-9002632a6ee9	Niko	3118
869ea6de-3d81-445b-85cd-4d8cf05ce550	Niles	3121
2cc7352c-4ea5-4001-ac11-c23b10711a63	Nilox	3124
74f18365-b862-4f25-9cb5-72d5bdcc2e4d	Nirotek	3127
fc4049d8-accf-47ce-a2ca-d1a72374d6e7	Noblex	3130
1ec91db8-94b7-4666-a26b-e3e8d20003d7	Nokia	3133
09380eb8-582f-46f8-b56d-059ca0716b70	Noontec	3136
9fbe15a4-e607-4526-82e1-b87c94e86dd2	Norcent	3139
0ac90d0f-f04c-4b6a-9b75-27f4960decc3	Nordmende	3142
931e12b7-8226-4d5d-bb44-b002860c41fb	Nortek	3145
9c813598-43df-4d81-a656-327a0f932220	Nova	3148
eda22e2f-678d-4444-acea-45b68e647a12	Novatron	3151
e7337f50-fd8b-4472-acd7-3ae26b6ebc1b	Now TV	3154
a47bad3e-2c07-43e4-8689-786764f4d529	Nu-Tech	3157
9a77c4fb-392b-49f4-9636-4f2969d81da5	NuForce	3160
d4c9695a-3500-4036-b616-b9395b38feb9	NuVision	3163
b4201b60-5d11-4a4d-94d5-730a215e3d83	NuVo	3166
a49c3e50-8251-41e7-b673-e83767676356	Nubert	3169
d425647f-1c0a-4c30-9cfc-a1f781498b4d	Numericable	3172
9dfe2a7b-57a8-4603-a6a3-28e1656c1660	Nvidia	3175
2a109692-6919-4c03-8d1f-e8c914954c1b	Nyko	3178
2a49b200-1643-4759-9810-7d3c049246f1	Nyrius	3181
6f88571a-1dd0-447a-99c7-1b564f9961b3	O2	3184
f40e446c-1f29-46d2-a757-d3772ef67fa5	O2Media	3187
5aacd0df-cb98-4a66-abd2-5cbc77b0ccfa	ODYS	3190
c80e2160-fc83-4eed-b5a1-3afafefdc8bb	OK	3193
ff2e3b5a-9632-43dc-8436-0bd554f211a5	ONN	3196
549921fc-2e64-4b85-9e28-bb0743c8c905	OSN	3199
cb493cd8-a9b5-4022-a995-15df4d7693f3	Ocean Digital	3202
e6aded82-221e-4b10-9d2a-8614d67716cd	Oceanic	3205
6c033a64-3247-4cfa-b568-85d242241449	Ocosmo	3208
468fc77f-58f2-451e-821a-fd796ed24753	Octagon	3211
7fdf82fb-af60-4d24-a0c6-4dc5cdede76a	Octava	3214
496990ac-e33e-4fdb-915a-6cf7e34538c9	Odeon	3217
1a2057de-317c-4702-884f-4a76762839f8	Oki	3220
f2fe9cc2-e075-4dbe-b5e1-3735331b15c0	Olevia	3223
54f1fd7a-c4dc-45c2-8c25-638d991c5168	Olin	3226
4943ab41-bd23-4bf0-bf07-f8820a89f7a5	Olive	3229
f5120459-7219-4d4a-91b7-10f21e9e0f2c	Olympus	3232
72e60aa2-2604-4b28-a614-7d2ca51c438c	OmegaSat	3235
f60c0801-173c-416c-93a9-db5691801bd1	Omnimount	3238
6eae3ba3-ba29-473b-a724-ff9e8460d561	Onida	3241
869594fc-3f62-48f4-b44e-4cdd78acd35e	Onix	3244
5d6398c8-ca43-46de-b5c1-e0ac576c5490	Onkyo	3247
0e887754-9b4a-4e16-ae4a-eb517362fde1	Onkyo/Integra	3250
e7cec4e8-c70f-4b1e-8aab-b9bedc9beaad	OpenBox	3253
ae403a6a-a11d-4e43-b943-6f5346c3beeb	OpenHour	3256
06ac74b8-ea22-4825-ab95-f3039bccff90	Openbox	3259
43bc05ef-b590-4c29-9ebc-f687de4def58	Opensat	3262
6d4d065d-3653-42ba-bae8-98f6cdabb6fb	Opentech	3265
a5a420b2-45c9-4b9a-9350-27942011e170	Opentel	3268
44738b23-984d-4bef-9722-319917cf8017	Oppo Digital	3271
94fe81f0-2f5e-49ac-b738-e71a04266f6b	Optex	3274
f29c24ff-fb24-4dc6-8a35-88316b67bbba	Optibox	3277
ad08f3ff-b63e-4249-bdac-38843b1d3440	Opticum	3280
981b0bad-6bce-471d-8eba-2b4974d545bb	Optimus	3283
3eb45697-311e-4e33-a057-fcb87ad11e8d	Optoma	3286
35952da3-bfcc-4855-a966-cf6c272fc31b	Opus	3289
911359f0-629d-48ee-bb62-48e77c3de9a7	Orange	3292
3159b7b6-b938-43ba-898a-1bac082346aa	Oray	3295
b74f3a4d-1f7d-41fe-98a0-ddbbb6d7475d	Orbitech	3298
e95fa62d-a1cc-4dbf-93be-b138dbfff849	Orbitsound	3301
75092eed-2c74-4d99-a145-255581d43d3c	Orbsmart	3304
adeae892-2365-49bf-aa31-8c6538895fa3	Orei	3307
b23b1334-5c85-4119-ab65-693fd84ce242	Orient	3310
258e256b-e346-4eb7-820d-a8f4dce55639	Original	3313
642f5bac-4b08-4f6c-ba0c-1dc92efab9e7	Orion	3316
34a5ecc0-1657-4adf-a850-7000fc68370b	Oritron	3319
dfe7fddc-66b2-4049-b8c1-48af1850d57b	Orton	3322
6384f007-62bf-4cea-a6d5-f0ab08ec915a	Osram	3325
110da51d-a05a-44d5-92a2-8094a42a82e9	Outlaw Audio	3328
41eced45-e564-48e6-8a65-b3b9a42585ac	Ovivo	3331
436b79e1-9f81-4d67-aa79-d1993ad905b9	PDI	3334
d30e6061-bb22-4e57-a7a4-4d6eb21df617	PMB	3337
fad6a351-75f5-49e4-9620-f6f91f27a191	PS Audio	3340
56fe3231-e246-48c7-9f79-79edf57bc473	PS3 Toothfairy	3343
2af6cce9-ef57-4340-adb3-ff7a77bbb588	PSB Speakers	3346
df36a6d4-4a76-4007-82f4-f315bee06285	Pace	3349
d82899ef-6d8a-4005-9f67-02a7578f5704	Pacific Digital Media	3352
573c8c72-c65a-4187-b89f-89abd20acc9e	Packard Bell	3355
e6c93536-e0e6-43cd-9384-72bef98a9d20	Palcom	3358
5ee5e32b-c052-4615-bcab-3b32a3cb549f	Palsonic	3361
3bac0a5b-319a-43c6-90d8-b6fe255ae29f	Panamorph	3364
acf315b8-b38e-4992-8026-ccaf11dd52ff	Panarex	3367
ac75b4ca-7ec4-4c4b-bc25-6a055b279f89	Panasonic	3370
a2b06beb-2989-470d-8a5f-a5c430a86c94	Panlong	3373
d9b5a038-782c-4350-b410-89fbf9f4fad3	Pansat	3376
763be89d-6c95-4bbd-bc6d-b6dbc15ddac2	Pantalla	3379
7c8876e9-fee4-4999-a6dc-9fcbc2ba614d	Pantel	3382
fca517e1-0d7a-459a-98ab-8d91ef75ea6b	Paradigm	3385
d8cf9126-9dd7-49b2-8d18-add221d5c248	Paramax	3388
5ce59964-031c-42a0-b8f1-2fa16e2d7243	Parasound	3391
079c09da-32c9-4260-a7b0-2212117667d0	Parker	3394
cdd9192c-246b-4afe-a9e2-68bab0363154	Parts Express	3397
c7903177-b4ef-4e01-ae37-6ebd80046561	Patriot	3400
7a8ba0d1-ac3a-48df-ac1c-73023d6ba086	Patriot Lighting	3403
753e5eae-fb20-48ae-b19e-81580dca459c	Paulmann	3406
412d7b4d-7a38-46bf-8bec-9d6f4677906d	Peachtree Audio	3409
1aec588f-b470-4e1a-b0ce-948313a04b3c	Peaq	3412
0e2b82a9-60a8-462c-8c07-de40401d053d	Peekton	3415
ea409f1b-8745-4551-8546-bac3d3a6dbb8	Peerless	3418
5ed98d15-3b93-49bf-a6dc-c2c652c05d28	Pega	3421
1cd86543-c4ab-41fd-a039-b0dd2e969078	Pelican	3424
bd773c92-f18d-4949-844b-63cc92d5b433	Pelonis	3427
4222f977-7ee8-461e-a2a7-746f1234949b	Pendo	3430
b53e07aa-5d41-49da-983f-a81f27d90588	Pensonic	3433
f8be061c-90a3-473a-8d4f-e9fc24d10b49	Perreaux	3436
7dfd8be9-cb6f-4bac-a16f-03a695755beb	PhD Solutions	3439
08b72e04-76a3-42d7-8528-dbab606032cc	Philco	3442
bc09b6da-954b-48fa-a002-308f67745f90	Philips	3445
6fd4a440-8424-4ed9-8cb8-bccc42d4ae5f	Phocus	3448
9f2b5744-e2b7-4620-afbe-8f034e184166	Pico Digital	3451
ed99f227-c1fd-419d-8ba7-a5e93ffb2b1f	Pilot	3454
17e1b588-1627-4225-a793-5aeb2015249c	Pinell	3457
c9035d91-6f00-4aec-a340-30a8be666d6e	Pinnacle	3460
a05dddcf-d599-48a7-9a28-93d3a1fceff4	Pinwheel	3463
94ef5b8e-7ab7-4a71-81b5-6e5c4bb4d010	Pioneer	3466
93925b2a-c416-4e0d-9c8b-62797c4f34c4	Pioneer Elite	3469
2486830f-3f98-4b59-bc1e-e32c236d8a7e	Pirelli	3472
b6d31fb5-c220-4045-9678-daef3585d411	Pivos	3475
3d7a756b-6a01-48b2-9677-e8dd0317583b	Pixel Magic	3478
51ff06d5-04f7-4e1f-b734-d5367431a5de	Planar	3481
ebf6c1a0-bc57-46cb-93db-88b9c5ce17a1	Platinum	3484
d79decdc-6ef8-4c8b-af84-962637fb4556	Plex	3487
63f45cbb-dee3-4630-958c-bc9e831dc9a7	Plinius	3490
c5738ee6-6600-4afc-b35c-8aae30c89156	Plus	3493
c80eeca2-b095-4781-821f-2c3ad6569f7d	Point Source Acoustics	3496
f148b6f3-2e64-4c31-8c71-d4875254ba3c	Polaroid	3499
17f19855-3d24-4952-a9af-ad6b99efb5c0	Polk	3502
352e0bc4-834b-4755-bc62-5657ecb9198f	Pollin	3505
c4e371d5-d50c-48b6-93be-3bb01170a3b1	PolyVision	3508
1b28f496-8545-49c2-a43b-05c7d7df9c6c	Polycom	3511
20dd7b51-daae-488a-a6ee-8722fe0c1a08	Popcorn Hour	3514
e5fc2efb-ef0d-419b-96d1-823199e71ae3	Poppstar	3517
ca6421e5-e0fb-4c90-9659-d3f949940c79	Portland	3520
f1e65b6c-8f15-4d1f-b278-b230b5563304	Portta	3523
3b390f6f-2881-4c19-9c33-c4bdbfa4a9de	Power Acoustik	3526
2fabd621-810f-48d9-a3ef-a818a05a7726	PowerNet	3529
54edbf05-53ad-4c57-90dd-6985e1db4e39	Powerpack	3532
fe6fd4c3-e482-4785-b980-15c0c20c1b25	Pozitek	3535
a06e5182-9d90-4815-a345-a5401dff056c	Precision	3538
c227d06c-1e01-437f-a4f0-fd84a58a4374	Precision Acoustics	3541
ae0a918b-88e8-4e76-8d3d-f4b749061c12	Premiere	3544
1c41719e-eafe-4b5b-bf81-7e39c8cd202f	Presidian	3547
93faa19d-fc99-4a26-a6f0-bd5a89d94ce5	Prestigio	3550
c01ceb39-382a-48c0-bf6b-8a8886d92834	Prima	3553
68379403-0f51-47bc-85e5-dec66c170a71	Primare	3556
7199fce3-7f53-40cd-8564-2d8d341e52fa	PrimeDTV	3559
1e2de95e-b1bc-441b-a04f-5741981d6217	Pro Connect	3562
80fae9fd-2c1f-4cd3-910a-06101a0b6369	Pro Control	3565
8877e955-931f-429a-8c17-ee2dbdf606f1	Pro-Ject Audio Systems	3568
14896265-9d75-469b-93fa-734d1cfa0865	ProBox2	3571
37702589-bfdc-4b07-b8b5-be106cf21005	Procaster	3574
a66240a8-e694-4502-9d06-467dd0ffbdde	Proficient	3577
28428bac-8bef-4ce6-a26f-e35dca966257	Projection Design	3580
05449ee8-7cb0-4817-a78b-2de06db17bb3	Proline	3583
2be80e94-1a33-456f-ae5f-5085314165e6	Promac	3586
fabd59ed-1ecf-4595-95d3-f4952823c084	Promethean	3589
81936a49-895c-494c-9645-481cb0e87a12	Proof Vision	3592
99e0c970-4557-4db6-809f-957dd3c449fe	Proscan	3595
b54ea621-dfcf-4225-b796-1ce47383e1cc	Prosound	3598
475f7bc5-7c62-499f-8f45-04d461785ec7	Protek	3601
1c79f66d-5450-4096-9f7e-32a088249c1e	Proton	3604
a8577b3f-6337-4381-9bdc-d7bd25fc722f	Proview	3607
73ea3f82-06a8-42b5-bb3f-d2afdc8293c2	Provision	3610
58ea2f9a-f445-4ca5-80c3-86e101e06291	Proxima	3613
0a4844fa-9cca-4982-801d-7964d6424efd	Psyclone	3616
a9e46a91-2950-44f9-a441-d99b81e3b919	Pulse Audio	3619
18a48afe-93ea-4c20-8967-f9262f027e70	Pure	3622
d765a76d-6364-4f81-83f4-bffc15fe4f43	Pure Theatre	3625
fa415d7e-67b0-4b6b-8cf9-ecacb59be793	PureLink	3628
57d3a991-1137-433f-8725-3847fe4eeaaa	Pyle	3631
e4c694cd-ceec-4b97-babb-3361151f4988	Q Acoustics	3634
e5401bc1-a70a-48fe-a152-e81c5cb3c06c	Q.Bell	3637
0b77b9f6-1c5c-42ad-b7c3-964235300a3b	QNAP	3640
2a697f78-0393-4580-ac0b-b820fab7d145	QRS	3643
d2a84c29-aa51-4da3-bd9b-9f3e7c1b2104	QSat	3646
baf23090-f233-45fc-afc2-dc9ba0ac10a1	QcoQce	3649
be958441-60cb-4e9a-8c55-767adb5ab2b3	Qmax	3652
71ea3df6-7720-498b-9ecd-3b63df0d7b84	Quad	3655
d09f9ddb-8a16-4648-a4f9-fa75d7f95d08	Quantum View	3658
a2124b90-1993-4a37-8220-8d0b8274f811	Quasar	3661
9f8f2692-f13e-4844-b4fb-beb37d559cac	Quatech	3664
003039fd-4ebe-40e4-a425-0854e0340cc9	Quest	3667
a0740ee5-ec7a-4b6c-82f7-2cf3cea1597b	Quickline	3670
5af9b803-c1ec-4557-9fc2-4d54bdb8d715	Qviart	3673
bc259cbb-c750-4502-880f-f172ae2cb362	RCA	3676
af7ebdc7-93dd-4b9a-a187-179c4ae45630	RCN	3679
eb7ae886-c143-4992-9727-de92ff4fde30	RF Link	3682
a28be144-5460-40fe-8ee1-d3e37a6c90c9	RJ Technology	3685
21070491-7e02-4c84-901c-1d8b30137140	RSL Speakers	3688
78886a0f-2ceb-4ca2-855d-56a65c1f8465	RTI	3691
2b68105f-f118-42bf-87a1-fd534d2651f3	Radio Shack	3694
6a6eaa3a-a5ac-4e0d-8cff-b1142d139bbc	Radiotone	3697
1df6951e-a4ba-4a25-81a7-643cbc5d32a7	RaidSonic	3700
702da654-4336-4bc9-8ab8-fd44f2ea0375	Rako	3703
df3febc8-3833-42e8-9d52-0da54b88dd2b	Rebox	3706
3de133fe-5a38-4bcc-9192-9bc6e7d048cb	Red Eagle	3709
eab22b80-16cf-4bea-a59e-d6aa2b10c9c8	Redline	3712
6b8a10c9-9477-474d-b882-7f8b1a5b97e3	Reel Multimedia	3715
9e1e6b68-0aff-4b58-b6f7-89c7c9b58849	Reflexion	3718
173dc043-72fd-4835-aa4b-eacf9fe51a43	Rega	3721
5d5ca885-1deb-41ba-88b4-5b5a3405b334	Regent	3724
aca95470-a1bf-4a16-a9d2-c668d519dc99	Rel	3727
2225de6c-c821-497a-9898-fd0e47811452	Reliance	3730
05f52955-6bdc-4101-ac20-724a09a6df24	Relisys	3733
f8c5c9ea-8e87-4e6b-bfbe-b943b4e29f04	Relook	3736
19693731-1f8e-48aa-a960-93d5d7b1904d	Replay	3739
a1d898f6-b4d8-42fe-b49f-554dafb00949	Request Mulitmedia	3742
ef067f54-fd89-4d93-85d2-567e1c3c3630	Revo	3745
0e84d566-a1d5-433e-a596-76e6a586e52a	RevolutionHD	3748
a83363dd-a3a6-4e4e-a8e3-3db3252df679	Revox	3751
f4635bcc-a1bc-4e5c-8cfb-0a8fa87fa15f	Rikomagic	3754
a2f1d6cc-e4c7-4f48-97d4-b329efcd43aa	Rinnai	3757
749e62cb-430e-4053-9c66-0c71ede9663f	Roberts	3760
56af2ae2-1a3c-4d6c-b1ca-821d6f159608	Rocketfish	3763
5d6ed36f-3d9a-4f6c-b351-d84ebafd2cd6	Rockford Fosgate	3766
1be193f4-406a-4d77-98b1-a29abc914db6	Rogers	3769
bd218a3a-4b48-4239-8c69-6a5a471b5f66	Rogue Audio	3772
c78ae415-fcbf-499b-975b-6c5085b0d1c7	Roksan	3775
b62b20f6-e3d9-41af-829e-2e2f6fdb1108	Roku	3778
3253ee2a-25a8-4ead-b0ca-942e5f27bd6c	Rollei	3781
70a9e142-cd2f-4539-a4f9-2f984207a03b	Rolsen	3784
49f7ef76-f579-4f30-b232-f9ecf8ae8e9b	Rominetak	3787
40484cf0-c727-463b-bc62-d46b85e42b12	Ronin	3790
51e745e2-dec2-4f47-852b-f422a4f44004	Rotel	3793
90b1e89f-1390-47e7-9f46-d6b28c2b0fff	Roth	3796
53a706ea-2c86-46e7-80cd-0c9f16d5395a	Ruark Audio	3799
6b505b60-51fa-41d2-8b41-2447abdbefd4	Rubin	3802
d69b66ae-e15d-48f4-bda2-227c513ede64	Runco	3805
1e8e4ea0-c40e-4fd0-a2e5-7c20b9f47dd3	Russound	3808
51686620-85d3-4b12-8efc-4925e82a83ac	SAB	3811
2c8b09cf-7ee9-4bd3-b75a-56d94bf8f53d	SEG	3814
1575d4f9-7354-4c35-bfe1-167c821b2107	SFR	3817
162a776c-93f4-46d3-b098-a3c9dfd1a59d	SIIG	3820
785197f1-9e75-4515-a433-84f4a0cc8467	SMC	3823
33b972a8-d591-4f79-8587-c0fa9da28b75	SVA	3826
47a2d254-851f-4fbb-8ddc-ac6bf49ee55e	Saaria	3829
1f2daae2-60c3-46f9-877f-143db49415a6	Saba	3832
89bed248-b862-450e-ae7f-834eb1cfba70	Sabre	3835
4eaf0904-32c9-46da-80cf-a0c159d63c95	Sabrent	3838
a8eb4e2a-0469-4620-8e5f-ac97ec3493e0	SageTV	3841
83c86568-2b08-49f6-a0fe-e0ac16284913	Sagem	3844
04e35af3-3149-4251-8c9b-7804fa9d7ae8	Sagemcom	3847
209d99e4-5e89-4553-b206-ee04de7aeb6d	Salora	3850
985ca539-19a4-441f-b3f3-c406ed5c8139	Sampo	3853
ba4f280c-b067-419c-b187-85924b392779	Samsat	3856
f26bb44c-57d4-4986-8369-bff0ed9126d6	Samsung	3859
aebea823-6541-4b59-b48a-e829a1cbdd11	Sandisk	3862
5d96474a-2a44-4e92-b9bf-4ab19fe5fff9	Sandstrom	3865
d37e2935-044b-4e47-ace2-3f0d8b788051	Sangean	3868
b7b9c405-9bb9-43d5-a690-bb872f0b35e6	Sansonic	3871
dbbc04f5-b6dc-40dc-9c1c-3d1b3e9c7ac4	Sansui	3874
ec1fef2e-2b7b-4ebc-ab7c-03cd460c1efc	Sanus Systems	3877
3cb037d5-d11e-4d88-87d6-9ed5c43acaa6	Sanyo	3880
b0424e56-69cb-40e0-8b85-476a0ee9d2b9	Sapphire	3883
7ae9f071-62e4-4831-9664-b57b9dcb7d56	SatKing	3886
50e3d2c5-b549-4879-b048-c8d72b328a96	Savant	3889
f5fb848e-1ee4-4609-9d80-2ce829e94ec0	Scansonic	3892
132143e7-f77f-4354-b6e8-d9cfdc4a2230	Scenario Automation	3895
fb3ea3ec-5bca-454c-81d6-3530953cc8b9	Scene Style	3898
64131839-f65f-41a8-b340-a3156bc1f7a4	Sceptre	3901
c6e3edc6-4fa3-4fd3-8cce-cd02d7e6f43f	Schaub Lorenz	3904
146f506a-6105-48c8-abe5-ba5e1ce6edc0	Schmartz	3907
56806fac-a94f-4c76-a582-0d0502c2f07d	Schneider	3910
9322b3a8-6c5a-4cdd-8a53-4d8268f806e6	Schwaiger	3913
f888917a-4403-403a-86a7-10a16e57b853	Scientific Atlanta	3916
dcae6036-edd2-41c0-9719-8987f60501f3	Scott	3919
f2ab6319-d0c2-42d9-9a24-8045bc987818	Screen Innovations	3922
84b9ed55-3de6-4121-9e39-10e2d177da08	Screen Research	3925
c5a17f61-13fa-4114-b1f8-77f954063549	Screentechnics	3928
947bf7ba-dfc0-4b4e-855f-71c79dfa6006	Seagate	3931
58f61739-ae7c-4723-91f6-1b6971e98b45	Sedea	3934
d775e14a-7efb-4c98-9e86-8d742d453467	Seiki	3937
fd65206c-7b3b-44fa-89fa-49f20905902f	Seltronic	3940
c507f786-6441-4831-bb4b-639553c81dad	Sencor	3943
3ea5dde5-8b40-48e1-88ab-20b60fac947a	Sennheiser	3946
acc1dfbf-aa81-429f-adac-484de9f9885d	Sensei	3949
9d659fe0-7b89-43ad-80ba-10c01bcd0651	Sentry	3952
0724cd5e-91df-4cee-a652-e6586dfccf85	Servimat	3955
49ec91e6-0d99-4c64-8204-fe8424b3ef73	SetOne	3958
5eb2f734-9d3d-442f-9b08-02ea8566d074	Seura	3961
ac230a19-9587-4f94-9c41-01f8f81c9585	Sewell Direct	3964
10e45f60-f41e-4a40-acfa-9c90051a556d	Seymour Screen Excellence	3967
3c2fc5c8-2280-42cd-8cdb-f30647f09b2b	Sezam	3970
c11c6ede-da4b-4b49-80e1-635f64e348f7	Sharp	3973
534b75be-4e46-47e6-ad46-7d7f1c7bbc5c	Sharper Image	3976
db92467b-8137-4044-9290-9cea2316bc10	Shaw	3979
ca999491-4240-4266-ab97-f19c2b22dae4	Shaw Direct	3982
269cb452-78e5-49d1-94a9-5abcb33c0181	Sherbourn	3985
7643d3d3-bd3b-4a4e-8112-f527e0e74320	Sherwood	3988
16ef974c-ec27-404d-af7e-264ac5605094	Shin Kin	3991
508754a7-ec18-4f5f-8ba7-9ca9aec0b757	Shinco	3994
8c611203-3f29-4322-8bc1-184cbd4daf13	Shinelco	3997
4c848ead-dd05-40b5-aa9d-c4dec318246a	Shinybow	4000
73a406fb-df61-42ed-bee2-055a4ee9f5cd	Shivaki	4003
c665b786-76af-4353-876c-52930d5c57a8	Showbox	4006
33412fd8-fcd4-4950-9c37-07ff59143ce0	Siemens	4009
f75d274a-47c0-40f9-968e-f527aaaf7a95	Sigmac	4012
3e5cfacd-9625-4622-beeb-ec709ee22826	Sigmatek	4015
e7422624-2552-4d21-b9e4-4483025b7566	Silicondust	4018
4c1d5c56-4f01-4292-94bb-e0634805c6cf	Silo	4021
a89dcfa6-caa9-4478-b562-8acb930507d9	Silva Schneider	4024
6fb3972b-da53-45d9-8254-ff6bbd643326	SilverCrest	4027
4f219f44-fb29-4d7c-9880-814e0abb8607	Sim2	4030
5b030216-f50e-43db-b2a6-44e1600b983b	SimAudio	4033
b3ac9685-cc8b-427b-9006-2e354619d197	Singer	4036
f45fe59e-7bf2-46a8-aa08-91390df0b5f8	Singtel	4039
60fcad1f-1d24-4543-9d93-6c884921cdc5	Sinotec	4042
5eea8ac4-9ca2-4420-b643-919ea3b4a0dc	Sinudyne	4045
8f33108b-9216-46d6-9e38-08ea98d5aa8b	Siragon	4048
57e6b4b1-28e6-4274-a812-88821e55acfc	Sirius	4051
4cebc5f6-cf24-48cb-bc2c-6fb969a6bace	SiriusXM	4054
39960229-2b26-4213-a203-865f7429b287	Sitecom	4057
e947534a-d99e-4468-8623-0a3117cc9622	Sky	4060
362f99aa-6a9c-4d82-bac0-0df5f85e7507	Sky Vision	4063
98372935-8dc5-44c9-a863-cc3196e5f891	SkyStream	4066
57bea70a-9de3-4224-bbe0-a540ede1b06b	SkyVue	4069
a1754e6a-4f44-48a4-912e-1b8d8dc76cb5	Skybox	4072
1aa694e3-009a-4f83-8a16-332863629d01	Skymaster	4075
5564ee45-1047-4ea8-913f-de9ffa402aa6	Skyworth	4078
dbd37908-419b-4f70-a8a7-481650e5478f	Slim Devices	4081
b967e5b4-bd3b-428d-a8ad-1393b040729a	Sling Media	4084
9247f4d2-28ea-4cb6-975e-5919ffbbb587	Smart	4087
1bc68f44-2804-4def-b266-9f3dc8a82bb7	Smart Tech	4090
d3527b6e-6835-45a2-a6f2-9df173dc4dde	SmartParts	4093
ec91a226-82e2-48fd-9f74-583b219ea147	Smartlabs	4096
4db3e6f3-6a55-4da3-a28b-abf46648cffa	Snap AV	4099
6d10f0a9-de20-41a4-bf9a-551da3cb9f36	SnapStream	4102
ad7ec48d-f5fd-447b-857b-f563682eabff	Sogno	4105
476b3484-e869-41cc-9ce1-a7827df0f7f6	Sogo	4108
1cf6002b-6f5c-407a-9270-730053b2e6f1	Sole	4111
748fb85b-313a-425e-9af0-bd1cd7644e39	SoleusAir	4114
09bfcb47-bf7c-4de9-af24-001dadc616fd	Somfy	4117
b69fd8eb-90c6-40eb-9957-db585ff2a9f2	Sommet	4120
16743ab5-d59c-4a77-98c5-f83bd951c879	Sonance	4123
b605c591-0fdd-4883-9095-f2ffcdf7a9b0	Sonicview	4126
b0918d17-9949-46d3-bf90-da636b56905a	Soniq	4129
45998413-fab4-4688-9b31-e4d1a5a1675b	Sonos	4132
7bed5d80-ebe6-4539-9da9-cb7e3ccc1dce	Sony	4135
37fdabf1-e039-45a1-8301-b9ad08447d83	Soundfreaq	4138
c27c2313-142d-4ee8-9237-a6c9e4c03e11	Soundgraph	4141
5373316a-f55e-44d9-b5b4-90eef438b70d	Soundmatters	4144
e411663b-ec66-40b2-b32e-809ab5b35015	Soundstream	4147
d01c19b9-97c9-4635-9829-fac0817b6bf0	Soyo	4150
b0c19ca7-6a08-4312-80fe-d2cc4a8254f4	Sparklight	4153
0b0ea80d-347f-4409-916d-0b00d44df3b5	Spatz	4156
d5b12970-fc58-41a3-908f-d87610422222	Speakercraft	4159
01a80ebb-5d68-47b4-be99-5b383a29631c	Specktroniq	4162
cc8d93cb-a3c9-48c0-b152-446fc07c863f	Spectrum	4165
7952fe0a-b75b-408f-83e5-dc5b4e54445a	Speedlink	4168
4c78ebbe-5b69-49f6-b59c-84b9bd515bc6	Spherex	4171
c6434e62-bb83-444c-9c95-e5609dc00c01	Spiderbox	4174
53ab03de-5148-43c4-9213-c54fdab85f07	Star	4177
6d92bf7b-f728-4a7e-b671-da2bfe44ce38	Star Track	4180
8ac9d574-4f89-4701-a5ec-85b5cffdc3ad	StarSat	4183
6663a87a-b3fd-44ec-b745-0d4f5c0a4a14	StarTech	4186
c75fd7a8-bae9-4fdc-b559-c99e037b67e5	Starbox	4189
e8d20fc0-ca71-4506-8525-a31c35ffb978	Starcom	4192
17da0c0c-f6e2-4e83-a4f1-27bf05226377	Starhub	4195
e6ce3a26-53e5-4315-a47d-cf2a959e7119	Starport	4198
e3dafc7f-126e-437d-b044-37b8acc261b9	Starview	4201
0b6e4675-3d4a-4c4f-aa73-e867c91ffe94	Stewart Filmscreen	4204
cc384965-2e51-46d0-9ad1-cc4a86ae3ac3	Storex	4207
d19c7c7d-c9b3-4b5a-bf3e-93a69432b931	StormAudio	4210
84dcba6e-e6cd-46c6-84be-83bea78776a0	Streambox	4213
bd44dc7b-7fee-49cd-a9fc-abee25b6eef3	Stromberg Carlson	4216
82e8763c-41c9-41ba-960f-a92ce8b830c0	Strong	4219
aa5e5527-db16-4327-8d9f-4b5d38b08f72	Suddenlink	4222
16a6f665-d7e4-4260-bc3c-81c659a68388	Sugden	4225
4e54ff44-eef0-4895-a7bc-8e7275345356	Sumitomo	4228
8d6fb34f-939e-4d57-bc9e-a52d8bbd97a7	Sumvision	4231
eaea470a-8e4f-4c99-8640-78a27f0d5636	Sun Direct	4234
367282b2-e895-4a40-9c15-2ac323eda61e	SunBright	4237
95c5ede2-a5f2-482e-ad8a-3d76632e44ff	SunBrite	4240
12cfb828-6668-4550-954f-3701429afeb7	Sunbeam	4243
9eab9a71-8515-446e-bb97-825efd5de868	Sunfire	4246
aea7f383-24b1-4c04-904e-0cd5854ee18c	Sunia	4249
58678284-fa8f-4024-95f7-2d961af4fa7b	Sunkey	4252
a5d3ed9e-c5f2-4a5b-a5b1-f36ac7e2d331	Sunny	4255
e3bcd4e6-45d5-460b-8a00-128862cd5478	Sunpentown	4258
65cc7554-0087-41cc-863f-4717f3bb594d	Sunstech	4261
aff25b86-afe2-49fb-9083-066354a55e53	Sunvell	4264
dd07fd46-ba2c-438b-863e-66cd19bb8836	SuperSonic	4267
dba391ae-0289-42c8-9c20-6d41360c26c0	Superscan	4270
4fff04b2-9146-4b24-ad9d-224f3525cd2f	Supersonic	4273
aa017b18-d540-47e9-9585-3f0edd3a0054	Supra	4276
866037bb-d788-49f6-8d73-ecbc7dd293fd	Sven	4279
1e32ea71-c41f-4e7e-baa6-b0598a67ba45	Sveon	4282
5449407b-45c7-4e90-a456-7ee848c0ad63	Swann Security	4285
a018e678-83c5-4d18-b3a5-e9ef125e6099	Swedx	4288
624da195-422d-4bc7-9aab-4d6065629a2f	Swisscom	4291
6d6db8f7-c270-4d3d-acd0-ab2e600933d1	Sylvania	4294
7c500dce-48a4-4f7c-8b11-93fc476c64ba	Symphonic	4297
5812083e-908b-4e23-bb41-6b60575ee773	Synaps	4300
bd38bb27-4dbd-4665-bdd9-b42606c9b43e	T+A	4303
a0917b3d-d275-440a-acc1-4e9415e3c6c6	T-Home	4306
de6ce97f-ecfe-404b-b1c7-a7bda2894b07	TAG McLaren	4309
df1f3d96-0523-4720-af57-21e75e403ac3	TCL	4312
c56d0433-204d-49f8-8b19-d93abab34109	TCM	4315
84abcc08-81e6-4926-9161-e0b62bf0da64	TEAC	4318
21cedec3-6838-4824-8a3f-eaeffc72edca	TELE System	4321
ea61d88a-422b-4e12-83ec-28e9410d3b74	TESmart	4324
b234b8da-7c99-4052-a126-688f9905f715	TICTID	4327
aa28468e-ad4d-405c-bd84-8d37db70e2e2	TV One	4330
2736b002-e3f4-43b5-8493-d730a632a1db	TV Star	4333
e0764a79-c4a0-4469-8fea-0ac9e96fa490	TVonics	4336
5cf48b01-bef6-4b41-895a-1969b1ccbae1	TVpad	4339
f1fe6035-f429-470e-b8d1-68cc5974d2de	TaTa Sky	4342
a15c2872-1627-4ef9-87ad-33b455207666	Tact Audio	4345
52126866-df93-4626-92df-7d62fcb5e724	Takara	4348
ba2a70c8-74b4-421f-9690-26b170a36b0f	Talcom	4351
6c807fd0-83a8-47c4-84ae-3304c3cb96ec	TalkTalk	4354
e38aaeff-b877-40f6-920f-81c418a0b04a	Tandberg	4357
5cefb0b2-2b13-4efd-9878-84eb5d67de25	Tangent	4360
c47dfbc9-b28f-47a8-b1bd-76ee17991b55	Tanix	4363
7a5f05e6-927c-424e-ac8b-80c17bf59d5b	Tao	4366
075c8588-ff88-485b-8127-823aef0243a7	Taotronics	4369
9d25cb18-e14a-4416-b4e7-fadebd69053a	Tascam	4372
511c0f4e-ce66-428c-934d-746b61647ab6	Tatung	4375
8b749084-59e6-47b5-833a-7b218c0394b8	Teac	4378
cf510014-5710-44d8-bd9c-ce25e93ecc7e	Tech2o	4381
0ed4fbfb-ed24-4786-8bf1-463f424cd031	TechLine	4384
4db3145d-4ded-4e3b-8e66-42ac0dec414e	TechniSat	4387
260cb5ac-f322-4382-bbe0-90e68207a545	Technical Pro	4390
f3d2b338-6a25-4137-88e0-f489c624a9be	Technicolor	4393
09250f30-be68-4f84-8c6e-93e23dd5e607	Technics	4396
e7f2b3ca-899f-41e9-beac-53cbeb1745ad	Technika	4399
d1522ef6-a2d3-457a-884d-2cf370c3f896	Techno Trend	4402
c367c0f3-27c9-4e18-8f0e-a08c83f438be	Technomate	4405
a68e9596-0877-48a5-a6ad-a6664d328fba	Technosat	4408
e73bd5c6-2e9d-4cbf-a04b-38627c80c88f	Techsolo	4411
6fe384d8-8b9a-4862-ad46-fe449db66034	Techwood	4414
511f3da6-505b-45c0-ba86-54552fc2cad3	TeckNet	4417
e7c350af-9956-4875-bcb0-1d143e955f3b	Tectoy	4420
8c75af91-6721-45f8-ada0-d1a73089f308	Tedelex	4423
5ee4c643-c4cf-4736-aa6f-344fe3f2d943	Teinuro	4426
2803957a-6ffe-4356-960c-1f711ed2a8bb	Tekcomm	4429
144b6acc-9b78-4e96-a61a-6f0d6a7f67fb	TelSKY	4432
29573708-b81e-4c98-acff-e70858075a9d	Telecaption	4435
e3a61230-8568-4acb-a098-173e53f32123	Telecommunication Technologies	4438
fdac9235-bc27-4060-b06a-36751552f2f5	Teledevice	4441
c8b4cf2d-0b0f-4fd0-b614-0231528fd803	Telefunken	4444
59d5fe50-e2d6-47a5-a965-36a019b7b5b1	Telekom	4447
41ba1de4-af10-493a-87c8-e22a30829130	Telelynx	4450
f747de6e-a54a-4eea-aebc-b9ef9294a048	Telenet	4453
45267615-68bc-439d-af1c-1dc2bd54eb37	Telergy	4456
b1e3e75c-be7f-48cc-8ed6-a2ab74f311e3	Telestar	4459
a0bcc2bd-d68e-4c81-ad8b-e7b712a5cdfd	Telsey	4462
ef3deb3b-cc0d-4fa8-a82c-b65e752719f6	Telstra	4465
3d96b74f-ce44-4c8c-afe9-29ed70fbd97f	Telus	4468
e2211338-a4ac-41d9-b5d0-26a3fd139ef1	Tendak	4471
cbd9e961-9c35-4cb8-9e25-215c47c991a7	Terk	4474
aafee7bd-845f-4df3-951d-1eb68e57c2ce	Terratec	4477
1ddca2da-44d9-4fb0-8843-49fd5cff174b	Terris	4480
97251df1-fe7e-4cab-908d-3e4108e819d4	Teufel	4483
091758f6-4254-48ce-9f4c-55c0c475e11d	Tevion	4486
755c2791-b1bc-4ddb-8302-4d91676af43e	Theta	4489
0d20c84a-c8d1-44c1-8512-d4472e013c76	Thomson	4492
bf3ccd6b-7f32-4dde-a122-ab9e327055ca	Thorn	4495
cef6dc40-9ad2-4d4e-a49e-fa63bbede3ad	Thule	4498
5dca1c72-d109-42ed-978b-e94def5d9fc1	TiVo	4501
5fe984d7-e76e-43ae-af86-8d9dab394966	Tiger	4504
727b7cb2-ceeb-4a59-ac0b-7d73862241d1	TileVision	4507
239d0c62-68c5-4a4d-a1ab-bbc4ef5dbd4e	Timeguard	4510
d193bfc0-fb39-47be-a875-42fa9d3f32c5	Titan	4513
fdc59a29-4189-4a1c-bc5b-4e97dca6dec2	Tivax	4516
20fb063a-3b78-489c-9ab6-1d5b0461a36d	Tivoli Audio	4519
f7b7b513-b40f-4468-9040-604137060ad6	Tocom	4522
e826d972-c81a-4e56-9bc1-5269acb41a0b	Tom	4525
17917465-21d3-48ef-8eca-d1dcab95e754	Tonbux	4528
392e8669-c6f4-47b3-add9-07e13a9b6c09	ToneWinner	4531
db5f1312-7085-4891-8473-8477389d5102	Topfield	4534
5f51f440-657c-4cb8-abac-12f573383841	Topison	4537
cfa489e9-e3cb-4195-ade8-3ce130b06dc8	Tornado	4540
b9e63e63-9d24-4531-b59e-c1d97fe6b532	Toshiba	4543
b310a8b4-ebd2-4c70-83e5-7b9509965c89	Totaldac	4546
ba6d5d26-2bb1-469d-b980-75e1fc6cf0fc	Touchstone	4549
755f59ce-99e6-42ee-aca7-98c48f9175e5	Trans Electric	4552
7119eec1-7743-427b-b8d9-af0dcefee535	Transcend	4555
ed879974-4052-4a51-b5a5-f17b193604e7	Transformative Engineering	4558
ea14b509-5605-4d47-aa4d-674d2e2961b0	Transonic	4561
9d303e19-1c02-47f5-877c-e68f3f182c9c	Traxdata	4564
a429af64-66c5-4599-ae1a-6152f63ad250	Traxis	4567
1f28eca6-80ce-49c1-ae39-8437fad32a99	Trekstor	4570
8b98b27c-6c3b-455c-90da-404b87491db3	Trevi	4573
1be95a70-eeb9-46d5-b6cb-6350e563ef5e	Triad	4576
fa42fca5-e983-4a27-a0dc-899914f50f38	Triad Speakers	4579
3c476ba6-23b5-4669-be43-40d105722b2f	Triax	4582
5affa887-091d-4577-aaba-80f81fdea363	Trinity	4585
5442cc5e-9396-4f05-a348-b0510cefe05d	Trinnov	4588
3fa5d402-e078-4161-a690-3e2d91e2b388	Triones	4591
ff9b8097-88ee-46d5-9df8-fc2788ad5ca2	Tripp Lite	4594
22af2da4-60e4-4960-b2bf-41b2306f000d	Tronsmart	4597
bc4388ce-66df-4cc1-92c9-086f78b30bfe	Tru Audio	4600
c77513b9-064b-4b50-b673-ec30e4dae68a	Truman	4603
8372df1e-1219-467e-8a2b-51acb960ba8e	Trutech	4606
e9cad427-acfa-4948-b774-5a82242d209d	Turtle Beach	4609
615d1a23-e403-4fff-945e-a0ba16f3ca1d	U2C	4612
d4f267e0-55eb-46c2-a8d3-55d5058eebce	UGREEN	4615
e27f1f17-216e-4c92-b4ea-8e1b8293c15c	UMC	4618
59bc9d3c-368d-4b28-bca1-958168a173f6	UPC	4621
783d093e-82d6-4952-aab0-73a52c19ac96	Uebo	4624
127189eb-80f2-4529-b265-74b49fa60e9f	UltraPlus	4627
11095c78-6393-4c0b-9bdc-6d907e098d45	Ultraluxx	4630
b757ffaa-01d3-4589-8734-03852f4583ab	Unblock Tech	4633
c9b85dfd-2881-436a-9016-c6c5ab809539	Uniden	4636
1d6bee12-954c-413a-9e4a-0044a601c10e	United	4639
cf634631-23f7-4385-8d20-4e1a87910d4a	Unitymedia	4642
d1ea5842-e55e-4800-a223-adc44adff822	Univers	4645
2c5c4031-50be-4fc1-8280-dbbf8208344d	Universum	4648
9272179d-c12e-4745-b96e-0c861fe8c029	Upstar	4651
4566fc8d-6971-4376-852a-ad89ab34da73	Usher	4654
9adeef3e-08ea-43b9-8969-98ef75f8d80f	V7	4657
efa8eaed-9390-4c9d-be4a-cd8974dda0dd	VDA	4660
d8448272-7cfd-4b51-b216-5a7af48a33ac	VDiGi	4663
bae01e49-ae86-4a60-b4a3-5447eb942041	VIGICA	4666
fbbc6247-bd26-4144-b254-a286e2c72796	VLSystem	4669
1f07f9d6-8e89-4d26-8769-0760b51e2826	VR Radio	4672
fa2079f5-1975-40a1-81a1-5699741b0864	VST	4675
ebe5e3a8-7b99-4b6b-b699-254c21a15078	VTech	4678
21c907dd-7b0e-4468-9162-654cffa7e851	Vanco	4681
1097d009-49c1-4bf9-b10e-7e6d6612a34b	Vantage	4684
96c082c0-83f4-4444-a086-3ca93b65e724	Vantage Digital	4687
44b151c5-b765-40e5-9962-1b930493734e	Varilight	4690
b75f43ca-7145-4256-b0c0-ba73e80af10c	Vaux	4693
550858da-3894-4b58-b742-047a29a2d2f3	Vefx	4696
41e4d422-548c-4217-9a32-150207faf124	Velleman	4699
cf6d7dcb-74ca-4ef9-8100-c7283c6295af	Velodyne	4702
d1cb34a5-16f8-4d94-80f8-f3af55e8f9fe	Venton	4705
0a894321-b80f-4e62-be17-6854f75ff1bb	Venturer	4708
ca8b7c84-21b3-4156-9339-ecfbefead8ed	Veon	4711
9c33ae3a-a42f-4fc4-af43-26f0d561532f	Verbatim	4714
a7625daf-b1c9-4ecb-9108-878d0eb5a1f8	Vestel	4717
104c6422-bf3a-467a-a819-e8c0985945f7	Viano	4720
1f8bf004-36d7-40b4-aba2-72248fdf2b9a	Viasat	4723
f71d695b-a1a1-4e79-a66d-a0a2701b4595	Video Storm	4726
c6b4ae5a-5f02-46bb-a149-e78712375366	VideoLogic	4729
97e68f37-85a9-463c-9ec3-79d8653430ab	VideoWeb	4732
e1337138-c9cc-4413-883f-2b497331eee7	Videocon	4735
cb7dcea9-51f9-43f1-957f-ea9972b343dc	Videostrong	4738
5c3c1f27-1d7a-47c3-801d-18a9b313d214	Videotron	4741
68c3b956-ad4f-42f4-867f-67a3c7acd7db	Vidikron	4744
ec764344-0292-4934-97a3-68f10ad92d87	ViewHD	4747
e8fbdec4-0a09-4205-8949-33c0642d5cf6	ViewSonic	4750
d6f0444d-46b0-4e8d-a048-97346a94e7db	ViewTV	4753
dfa2bc76-101d-4cf9-afb4-185d1bb216e5	Viewpia	4756
9c665065-2bdc-435f-95df-42ff2c9dc1d4	Viewsat	4759
cf519b90-682a-49c4-8740-32eba832196c	Vimax	4762
17052c22-0748-446b-9592-fb866e71a33e	Vincent Audio	4765
31fa9add-c057-49cf-9c6f-6cb6944d6f42	Viore	4768
e0f46766-01d2-42fe-92da-32de3ddd7a4f	Virgin Media	4771
afdb69af-b5cf-47f6-9bd6-1e4603d8a77f	Virgin Multimedia	4774
f533b05c-9c21-4fc2-bad4-5d275c7916ed	Vision Technology	4777
abf4f385-edf4-4df4-9861-76806ac6d1eb	VisionNet	4780
d6eb0a33-53bf-43d3-b8ef-793c871fe35b	VisionQuest	4783
41509276-28cb-40c3-a10e-6c9203cb5880	Visionic	4786
48f4c73e-fde0-4e30-8184-3cbe1897d258	Visiosat	4789
c6735516-f9be-4521-970a-fea3266800cc	Vita Audio	4792
e7e43025-47a6-49f3-95e2-c691fc1cf4f4	Vitek	4795
1fa7b6f7-c50c-4cad-b600-694497db5b7b	Vivanco	4798
56e1ba2d-9cf4-418e-9cbc-48ebf194b610	Vivax	4801
f2aaf813-c877-45b2-a3b5-a93e9faedd38	Vivitek	4804
63f9dcf0-92c6-4494-86ad-1dc46e0e4dbe	Vivo	4807
8bafbf61-033e-4bbd-b6d2-d441aebffb1e	Vizio	4810
801e7e47-85dc-40c1-a631-6731a0efdc99	Vizyon	4813
211d754d-5dcd-4d43-a5b9-2f288bac1e5e	Vmedia	4816
946182f3-70ce-495f-8d33-824287f55f8b	Vocopro	4819
66a5ca62-ea32-4ff9-b773-20403424126a	Vodafone	4822
8f01ab6c-2481-468a-a38e-17b583c92e3e	VonHaus	4825
2379d79b-1f60-4a8d-8af1-545bc0e99109	Voo	4828
dd348a69-8be6-46ae-b999-a297b27a2213	Vornado	4831
076db46a-9d22-42e5-a435-9a250f3627d2	Voxson	4834
9855d488-407e-4fd9-a6a1-d1e4259ce1c9	Voygon	4837
e4525398-5f61-4f95-a419-3d53dc79ead7	Vu	4840
c23a6336-4609-4493-90bb-a224873df0f7	Vu+	4843
54d3b778-3484-4ebd-a93a-cd8f0da6799d	VuDu	4846
07d12968-e05c-475e-90a9-d4f42d09e83d	Vutec	4849
c6f32555-55a0-4599-a205-d1c182354a4e	WB Electronics	4852
6c2211a2-2695-4a3a-8d47-1fe985de8bf2	WBOX	4855
71f7c1af-9598-4f1f-86eb-c6b90e610beb	WBOX Technologies	4858
21dfd20d-1651-44dc-8243-0f84a0bbba7f	WOW!	4861
2bdb52dc-5fc5-4cd1-834e-ebb3b78077c1	Wadia	4864
2160581b-51df-4235-868e-86c6428ef0ed	Walker	4867
7d8b3295-dd74-4802-826a-007d61786606	Wall Wizard	4870
de3dfede-7a6a-4828-86c5-ba7c54b1b5b6	Wansa	4873
5891bfc3-c204-4a61-ae70-6018867f1a4a	Waoo	4876
e7c99954-8ed8-41fb-96c7-351e98eac933	WaytecQ	4879
7c451157-07a4-46b5-8d8e-8ddcc4daea8c	We Digital	4882
c49a989e-8b1e-4199-a42f-48f4412cffad	WellAV	4885
10fd13bb-9d22-47fb-8717-ddcf56fe4213	Western Digital	4888
6f180ac0-ce62-4383-8191-b5bbf97dc18f	Westinghouse	4891
7206c67b-a08f-4c1e-a6a5-8b536c041fea	Westpointe	4894
fbf3f8d9-7e60-4b6a-adf5-4cbfb9ac54dd	Wetek	4897
64799cbb-432e-44fb-8783-55e9ae276f19	Wharfedale	4900
75555e66-7d30-43fc-b783-b294da60fd1a	Winbook	4903
d7b74737-a91c-486b-90ef-369259da9dd6	Wintal	4906
bc189124-f24b-4771-ad5a-5781d4a7db16	Wirepath Surveillance	4909
9f333b32-be57-4057-9b24-2df518872bf6	Wisi	4912
a0c95c5b-3334-4cdb-a4e3-3cb72a309ba7	Wolf Cinema	4915
94f4c138-e489-4f8e-af00-da497c7fa764	Woxter	4918
1558cf6a-11da-4a9a-bed5-bad7ac9780ba	Wyred4sound	4921
8c50f2a0-0861-4db4-9c3f-4c58c23efa90	Wyrestorm	4924
00c8cb29-2980-428d-bb94-70a896327f66	X10	4927
cee7865c-5ae6-4124-889a-a3c63ea62733	Xantech	4930
1234cf22-a799-4345-a0d8-5331a164dc42	Xavi	4933
fdb54eb0-6fcf-4af6-a650-6fa562fd26bd	Xcruiser	4936
56059db0-dc1d-41e3-99c8-0cb543db58f0	Xdome	4939
5a9fce8e-a280-42bf-805c-33b535eb7681	Xenta	4942
d14e8a55-556f-4b16-8c36-e971ffc87b6d	Xeobox	4945
66818a99-ed55-4107-bd28-c94016625310	Xfinity	4948
fe31d05e-7b8d-471b-9554-cff1054d2bdd	Xgody	4951
0e077434-9dfa-4a01-ab3d-447610f91b84	Xiaomi	4954
dc0604e8-555b-41a4-8f75-ab1f1e2671e1	Xion	4957
a21532bc-4d86-4246-ae36-a934d91eea26	Xomax	4960
2d9a5e64-ae7a-4399-9b39-c7538f094d1b	Xoro	4963
75370eb2-16d5-491c-9727-1611a5c8b499	Xtreamer	4966
4987a2cd-951d-418e-894c-c028e2a20f09	XtremeMac	4969
19a05c68-fac1-4f28-b6ee-75eea5afdeb1	Xtrend	4972
6e6c8e34-3de8-485a-b1c3-d5b68bf15ebd	YBA	4975
86e812ac-f829-461f-bd4c-93e9f08507ec	Yamaha	4978
95f26705-ed0d-46a6-b555-518032f17817	Yes	4981
3c6398fc-9e15-4ea3-b352-d6608edf515a	YouSee	4984
2796adf7-3b04-4d37-a57b-abe78499d15c	Yukai	4987
8afe5c0e-8a05-4b0e-8ec7-21a10e7e116f	Yumatu	4990
25863d2f-a898-4c63-847e-c79eeb434891	ZTE	4993
9809e05b-aa95-41aa-9852-fe6f80ec7386	ZUUM Media	4996
494b6b23-e75a-4178-a7a1-3010bc41ba73	ZaapTV	4999
be8de527-f61f-4651-84ff-0fccaa15dd3e	Zaapa	5002
feda095c-55e1-4e76-a56b-3ded770d7138	Zanden	5005
64134167-5f98-47b7-b78a-8991d63485b2	ZapMaster	5008
aa5be62c-9715-4d6b-a386-2d10871dc625	Zappiti	5011
b0b1451a-8e78-4b65-934b-7d1885ca77f3	ZeeVee	5014
ff0e1fef-4b04-49cb-9335-c4c18405de32	Zehnder	5017
cf12fc2f-0a78-441f-b80b-a3b6a1e31cc2	Zektor	5020
5559b8d2-a1e5-4bd1-af71-61e915d1bb16	Zenega	5023
87308008-aa1b-4bcd-a641-5a94de55b91d	Zenith	5026
be6db887-8df9-480c-9869-2c5fd0ca8251	Zepto	5029
f0b4dfe9-922a-4c33-a3d6-732e89c22128	Zettaguard	5032
30ab2e39-f532-4d8b-ba63-fa542d4dd33d	Zgemma	5035
f4a73cb6-b4e3-421b-b51f-ded5bad6391c	Zidoo	5038
a23a3d31-f180-4467-bc35-92f7335c5f5f	Zigen	5041
5a56cf3c-4807-491c-bf2d-c6ce9bdb4ce2	Ziggo	5044
116dd9be-ab8b-4af1-a6a6-4a3b4aacf146	Zinwell	5047
1a99f975-47ac-4801-b409-a54c9190dc8f	Zon Multimedia	5050
c19fccac-fc04-45bf-ba8c-b05b99d03431	Zoomtak	5053
d4988e48-09cb-41e1-8dcf-09c1bc8659d8	Zotac	5056
1542da96-a9e6-46ff-9adc-8023b45dfb7b	Zulex	5059
f01b8a3c-4704-42aa-a7a8-42e797bae061	Zune	5062
5bef5bde-8da6-49fc-848c-0a7b7ebf94b7	Zvox	5065
d0d867cb-7258-4490-8145-db96b9c285e5	ZyXEL	5068
1a378fe2-b293-4c39-93f9-f72a499aba50	chiliGREEN	5071
36d5424f-85ed-436b-8f70-7909a5b60173	deleyCON	5074
d00ce4b2-954d-44fb-b290-1dad95501ad0	i Luv	5077
527b4c9e-06b4-408e-bac0-fc47e66828c9	iLink	5080
5f406aec-5f35-438f-8a83-e8861a918643	iNneXT	5083
a3062ac7-782b-43db-80fc-aff38dcdece5	iconBIT	5086
8971c88f-4e38-4d7e-8bda-c79a28244186	it Magic	5089
4ebbca76-43b4-4ad5-a278-bfd6085a5566	mMotion	5092
7506cc67-3d81-4e36-af1b-235367fb3d81	miniDSP	5095
888533c8-77ed-4045-a937-cbba5b67bff8	oCosmo	5098
\.


--
-- TOC entry 3483 (class 0 OID 410505)
-- Dependencies: 209
-- Data for Name: mm_hardware_specifications; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_hardware_specifications (mm_hardware_spec_guid, mm_hardware_spec_model, mm_hardware_spec_json, mm_hardware_spec_ir_json, mm_hardware_spec_manufacturer_guid) FROM stdin;
\.


--
-- TOC entry 3520 (class 0 OID 417812)
-- Dependencies: 246
-- Data for Name: mm_hardware_type; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_hardware_type (mm_hardware_type_guid, mm_hardware_type_name) FROM stdin;
\.


--
-- TOC entry 3486 (class 0 OID 410526)
-- Dependencies: 212
-- Data for Name: mm_library_dir; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_library_dir (mm_media_dir_guid, mm_media_dir_path, mm_media_dir_last_scanned, mm_media_dir_status, mm_media_dir_class_enum, mm_media_dir_username, mm_media_dir_password) FROM stdin;
\.


--
-- TOC entry 3484 (class 0 OID 410511)
-- Dependencies: 210
-- Data for Name: mm_library_link; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_library_link (mm_link_guid, mm_link_name, mm_link_json, mm_link_username, mm_link_password) FROM stdin;
\.


--
-- TOC entry 3485 (class 0 OID 410520)
-- Dependencies: 211
-- Data for Name: mm_media; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_media (mm_media_guid, mm_media_metadata_guid, mm_media_path, mm_media_ffprobe_json, mm_media_json, mm_media_class_enum) FROM stdin;
\.


--
-- TOC entry 3477 (class 0 OID 410469)
-- Dependencies: 203
-- Data for Name: mm_media_channel; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_media_channel (mm_channel_guid, mm_channel_name, mm_channel_media_id, mm_channel_country_guid, mm_channel_logo_guid) FROM stdin;
\.


--
-- TOC entry 3487 (class 0 OID 410532)
-- Dependencies: 213
-- Data for Name: mm_media_remote; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_media_remote (mmr_media_guid, mmr_media_link_guid, mmr_media_uuid, mmr_media_metadata_guid, mmr_media_ffprobe_json, mmr_media_json, mmr_media_class_enum) FROM stdin;
\.


--
-- TOC entry 3506 (class 0 OID 410646)
-- Dependencies: 232
-- Data for Name: mm_media_sync; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_media_sync (mm_sync_guid, mm_sync_path, mm_sync_path_to, mm_sync_options_json) FROM stdin;
\.


--
-- TOC entry 3488 (class 0 OID 410538)
-- Dependencies: 214
-- Data for Name: mm_metadata_album; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_album (mm_metadata_album_guid, mm_metadata_album_name, mm_metadata_album_id, mm_metadata_album_json, mm_metadata_album_musician_guid, mm_metadata_album_user_json, mm_metadata_album_localimage) FROM stdin;
\.


--
-- TOC entry 3489 (class 0 OID 410544)
-- Dependencies: 215
-- Data for Name: mm_metadata_anime; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_anime (mm_metadata_anime_guid, mm_metadata_anime_name, mm_metadata_anime_json, mm_metadata_anime_mapping, mm_metadata_anime_mapping_before, mm_metadata_anime_localimage, mm_metadata_anime_user_json, mm_metadata_anime_media_id) FROM stdin;
\.


--
-- TOC entry 3490 (class 0 OID 410550)
-- Dependencies: 216
-- Data for Name: mm_metadata_book; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_book (mm_metadata_book_guid, mm_metadata_book_isbn, mm_metadata_book_isbn13, mm_metadata_book_name, mm_metadata_book_json, mm_metadata_book_user_json, mm_metadata_book_localimage) FROM stdin;
\.


--
-- TOC entry 3491 (class 0 OID 410556)
-- Dependencies: 217
-- Data for Name: mm_metadata_collection; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_collection (mm_metadata_collection_guid, mm_metadata_collection_name, mm_metadata_collection_media_ids, mm_metadata_collection_json, mm_metadata_collection_imagelocal) FROM stdin;
\.


--
-- TOC entry 3480 (class 0 OID 410487)
-- Dependencies: 206
-- Data for Name: mm_metadata_download_que; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_download_que (mm_download_guid, mm_download_provider, mm_download_que_type, mm_download_new_uuid, mm_download_provider_id, mm_download_status, mm_download_path) FROM stdin;
\.


--
-- TOC entry 3492 (class 0 OID 410562)
-- Dependencies: 218
-- Data for Name: mm_metadata_game_software_info; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_game_software_info (gi_id, gi_system_id, gi_game_info_short_name, gi_game_info_name, gi_game_info_json, gi_game_info_localimage, gi_game_info_sha1, gi_game_info_blake3) FROM stdin;
\.


--
-- TOC entry 3493 (class 0 OID 410568)
-- Dependencies: 219
-- Data for Name: mm_metadata_game_systems_info; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_game_systems_info (gs_id, gs_game_system_name, gs_game_system_alias, gs_game_system_json, gs_game_system_localimage) FROM stdin;
\.


--
-- TOC entry 3494 (class 0 OID 410574)
-- Dependencies: 220
-- Data for Name: mm_metadata_logo; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_logo (mm_metadata_logo_guid, mm_metadata_logo_media_guid, mm_metadata_logo_image_path) FROM stdin;
\.


--
-- TOC entry 3495 (class 0 OID 410580)
-- Dependencies: 221
-- Data for Name: mm_metadata_movie; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_movie (mm_metadata_movie_guid, mm_metadata_movie_media_id, mm_metadata_movie_name, mm_metadata_movie_json, mm_metadata_movie_localimage_json, mm_metadata_movie_user_json) FROM stdin;
\.


--
-- TOC entry 3496 (class 0 OID 410586)
-- Dependencies: 222
-- Data for Name: mm_metadata_music; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_music (mm_metadata_music_guid, mm_metadata_media_music_id, mm_metadata_music_name, mm_metadata_music_json, mm_metadata_music_album_guid, mm_metadata_music_user_json) FROM stdin;
\.


--
-- TOC entry 3497 (class 0 OID 410592)
-- Dependencies: 223
-- Data for Name: mm_metadata_music_video; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_music_video (mm_metadata_music_video_guid, mm_metadata_music_video_media_id, mm_metadata_music_video_band, mm_metadata_music_video_song, mm_metadata_music_video_json, mm_metadata_music_video_localimage_json, mm_metadata_music_video_user_json) FROM stdin;
\.


--
-- TOC entry 3498 (class 0 OID 410598)
-- Dependencies: 224
-- Data for Name: mm_metadata_musician; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_musician (mm_metadata_musician_guid, mm_metadata_musician_name, mm_metadata_musician_id, mm_metadata_musician_json, mm_metadata_musician_localimage) FROM stdin;
\.


--
-- TOC entry 3499 (class 0 OID 410604)
-- Dependencies: 225
-- Data for Name: mm_metadata_person; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_person (mm_metadata_person_guid, mm_metadata_person_media_id, mm_metadata_person_meta_json, mm_metadata_person_image, mm_metadata_person_name) FROM stdin;
\.


--
-- TOC entry 3505 (class 0 OID 410640)
-- Dependencies: 231
-- Data for Name: mm_metadata_review; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_review (mm_review_guid, mm_review_metadata_guid, mm_review_json) FROM stdin;
\.


--
-- TOC entry 3500 (class 0 OID 410610)
-- Dependencies: 226
-- Data for Name: mm_metadata_sports; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_sports (mm_metadata_sports_guid, mm_metadata_media_sports_id, mm_metadata_sports_name, mm_metadata_sports_json, mm_metadata_sports_user_json, mm_metadata_sports_image_json) FROM stdin;
\.


--
-- TOC entry 3501 (class 0 OID 410616)
-- Dependencies: 227
-- Data for Name: mm_metadata_tvshow; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_metadata_tvshow (mm_metadata_tvshow_guid, mm_metadata_tvshow_name, mm_metadata_tvshow_json, mm_metadata_tvshow_localimage_json, mm_metadata_tvshow_user_json, mm_metadata_media_tvshow_id) FROM stdin;
\.


--
-- TOC entry 3502 (class 0 OID 410622)
-- Dependencies: 228
-- Data for Name: mm_notification; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_notification (mm_notification_guid, mm_notification_text, mm_notification_time, mm_notification_dismissable) FROM stdin;
\.


--
-- TOC entry 3503 (class 0 OID 410628)
-- Dependencies: 229
-- Data for Name: mm_options_and_status; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_options_and_status (mm_options_and_status_guid, mm_options_json, mm_status_json) FROM stdin;
df641592-2c6a-4ffa-816d-5f24dcea1ddd	{"API": {"anidb": null, "imvdb": null, "google": "AIzaSyCwMkNYp8E4H19BDzlM7-IDkNCQtw0R9lY", "isbndb": "25C8IT4I", "tvmaze": "mknotneeded", "thetvdb": "147CB43DCA8B61B7", "shoutcast": null, "thelogodb": null, "soundcloud": null, "themoviedb": "f72118d1e84b8a1438935972a9c37cac", "globalcache": null, "musicbrainz": null, "thesportsdb": "4352761817344", "opensubtitles": null, "openweathermap": "575b4ae4615e4e2a4c34fb9defa17ceb", "rottentomatoes": "f4tnu5dn9r7f28gjth3ftqaj"}, "MAME": {"Version": 230}, "User": {"Password Lock": null, "Activity Purge": null}, "Cloud": {}, "Trakt": {"ApiKey": null, "ClientID": null, "SecretKey": null}, "Backup": {"Interval": 0, "BackupType": "local"}, "Docker": {"Nodes": 0, "SwarmID": null, "Instances": 0}, "LastFM": {"api_key": null, "password": null, "username": null, "api_secret": null}, "Twitch": {"OAuth": null, "ClientID": null}, "Account": {"ScheduleDirect": {"User": null, "Password": null}}, "Metadata": {"Trailer": {"Clip": false, "Behind": false, "Carpool": false, "Trailer": false, "Featurette": false}, "DL Subtitle": false, "MusicBrainz": {"Host": null, "Port": 5000, "User": null, "Password": null}, "MetadataImageLocal": false}, "Transmission": {"Host": null, "Port": 9091, "Password": "metaman", "Username": "spootdev"}, "Docker Instances": {"elk": false, "smtp": false, "mumble": false, "pgadmin": false, "portainer": false, "teamspeak": false, "wireshark": false, "musicbrainz": false, "transmission": false}, "MediaKrakenServer": {"MOTD": null, "Maintenance": null, "Server Name": "MediaKraken", "MaxResumePct": 5}}	{"thetvdb_Updated_Epoc": 0}
\.


--
-- TOC entry 3504 (class 0 OID 410634)
-- Dependencies: 230
-- Data for Name: mm_radio; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_radio (mm_radio_guid, mm_radio_name, mm_radio_description, mm_radio_address, mm_radio_active) FROM stdin;
\.


--
-- TOC entry 3481 (class 0 OID 410493)
-- Dependencies: 207
-- Data for Name: mm_software_category; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_software_category (mm_software_category_guid, mm_software_category_category) FROM stdin;
\.


--
-- TOC entry 3517 (class 0 OID 410882)
-- Dependencies: 243
-- Data for Name: mm_software_developer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_software_developer (mm_developer_guid, mm_developer_name, mm_developer_json) FROM stdin;
\.


--
-- TOC entry 3518 (class 0 OID 410891)
-- Dependencies: 244
-- Data for Name: mm_software_publisher; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_software_publisher (mm_publisher_guid, mm_publisher_name, mm_publisher_json) FROM stdin;
\.


--
-- TOC entry 3507 (class 0 OID 410652)
-- Dependencies: 233
-- Data for Name: mm_tv_schedule; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_tv_schedule (mm_tv_schedule_id, mm_tv_schedule_station_id, mm_tv_schedule_date, mm_tv_schedule_json) FROM stdin;
\.


--
-- TOC entry 3508 (class 0 OID 410658)
-- Dependencies: 234
-- Data for Name: mm_tv_schedule_program; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_tv_schedule_program (mm_tv_schedule_program_guid, mm_tv_schedule_program_id, mm_tv_schedule_program_json) FROM stdin;
\.


--
-- TOC entry 3509 (class 0 OID 410664)
-- Dependencies: 235
-- Data for Name: mm_tv_stations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_tv_stations (mm_tv_stations_id, mm_tv_station_name, mm_tv_station_id, mm_tv_station_channel, mm_tv_station_json, mm_tv_station_image) FROM stdin;
\.


--
-- TOC entry 3510 (class 0 OID 410670)
-- Dependencies: 236
-- Data for Name: mm_user; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_user (id, username, email, password, created_at, active, is_admin, user_json, lang) FROM stdin;
\.


--
-- TOC entry 3511 (class 0 OID 410676)
-- Dependencies: 237
-- Data for Name: mm_user_activity; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_user_activity (mm_activity_guid, mm_activity_name, mm_activity_overview, mm_activity_short_overview, mm_activity_type, mm_activity_item_guid, mm_activity_user_guid, mm_activity_datecreated, mm_activity_log_severity) FROM stdin;
\.


--
-- TOC entry 3512 (class 0 OID 410682)
-- Dependencies: 238
-- Data for Name: mm_user_group; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_user_group (mm_user_group_guid, mm_user_group_name, mm_user_group_description, mm_user_group_rights_json) FROM stdin;
38117775-93b4-47d6-9a42-fc886ab6580c	Administrator	Server administrator	{"Admin": true, "PreviewOnly": false}
6666956c-a4d8-45bc-9c56-deaa1c2b68d3	User	General user	{"Admin": false, "PreviewOnly": false}
bea39ac2-505e-4cdd-9a3b-9c7da2cb28b2	Guest	Guest (Preview only)	{"Admin": false, "PreviewOnly": true}
\.


--
-- TOC entry 3514 (class 0 OID 410690)
-- Dependencies: 240
-- Data for Name: mm_user_profile; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_user_profile (mm_user_profile_guid, mm_user_profile_name, mm_user_profile_json) FROM stdin;
0863c596-3d62-4409-83c2-0515d3465adc	Adult	{"3D": true, "TV": true, "Home": true, "Lang": "en", "Sync": true, "Adult": true, "Books": true, "Games": true, "MaxBR": 100, "Movie": true, "Music": true, "IRadio": true, "Images": true, "LiveTV": true, "Sports": true, "Internet": true, "MaxRating": 5}
2bf39eb6-8192-4c2d-88cf-6341bbec1cd8	Teen	{"3D": true, "TV": true, "Home": true, "Lang": "en", "Sync": false, "Adult": false, "Books": true, "Games": true, "MaxBR": 50, "Movie": true, "Music": true, "IRadio": true, "Images": true, "LiveTV": true, "Sports": true, "Internet": true, "MaxRating": 3}
2bcbbd3e-8e2e-4fdc-92a9-471e5d018539	Child	{"3D": false, "TV": true, "Home": true, "Lang": "en", "Sync": false, "Adult": false, "Books": true, "Games": true, "MaxBR": 20, "Movie": true, "Music": true, "IRadio": false, "Images": true, "LiveTV": false, "Sports": true, "Internet": false, "MaxRating": 0}
\.


--
-- TOC entry 3515 (class 0 OID 410696)
-- Dependencies: 241
-- Data for Name: mm_user_queue; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_user_queue (mm_user_queue_guid, mm_user_queue_name, mm_user_queue_user_id, mm_user_queue_media_type_enum) FROM stdin;
\.


--
-- TOC entry 3516 (class 0 OID 410702)
-- Dependencies: 242
-- Data for Name: mm_version; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.mm_version (mm_version_number) FROM stdin;
43
\.


--
-- TOC entry 3528 (class 0 OID 0)
-- Dependencies: 239
-- Name: mm_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.mm_user_id_seq', 1, false);


--
-- TOC entry 3168 (class 2606 OID 410707)
-- Name: mm_software_category gc_id_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_software_category
    ADD CONSTRAINT gc_id_pk PRIMARY KEY (mm_software_category_guid);


--
-- TOC entry 3224 (class 2606 OID 410709)
-- Name: mm_metadata_game_software_info gi_id_mpk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_game_software_info
    ADD CONSTRAINT gi_id_mpk PRIMARY KEY (gi_id);


--
-- TOC entry 3231 (class 2606 OID 410711)
-- Name: mm_metadata_game_systems_info gs_id_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_game_systems_info
    ADD CONSTRAINT gs_id_pk PRIMARY KEY (gs_id);


--
-- TOC entry 3161 (class 2606 OID 410713)
-- Name: mm_metadata_download_que mdq_id_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_download_que
    ADD CONSTRAINT mdq_id_pk PRIMARY KEY (mm_download_guid);


--
-- TOC entry 3324 (class 2606 OID 410715)
-- Name: mm_user_activity mm_activity_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_user_activity
    ADD CONSTRAINT mm_activity_pk PRIMARY KEY (mm_activity_guid);


--
-- TOC entry 3149 (class 2606 OID 410717)
-- Name: mm_media_channel mm_channel_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_media_channel
    ADD CONSTRAINT mm_channel_guid_pk PRIMARY KEY (mm_channel_guid);


--
-- TOC entry 3155 (class 2606 OID 410719)
-- Name: mm_cron_jobs mm_cron_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_cron_jobs
    ADD CONSTRAINT mm_cron_guid_pk PRIMARY KEY (mm_cron_guid);


--
-- TOC entry 3332 (class 2606 OID 410889)
-- Name: mm_software_developer mm_developer_id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_software_developer
    ADD CONSTRAINT mm_developer_id PRIMARY KEY (mm_developer_guid);


--
-- TOC entry 3157 (class 2606 OID 410721)
-- Name: mm_hardware_device mm_device_id_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_hardware_device
    ADD CONSTRAINT mm_device_id_pk PRIMARY KEY (mm_device_guid);


--
-- TOC entry 3170 (class 2606 OID 410723)
-- Name: mm_game_dedicated_servers mm_game_server_id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_game_dedicated_servers
    ADD CONSTRAINT mm_game_server_id PRIMARY KEY (mm_game_server_guid);


--
-- TOC entry 3173 (class 2606 OID 410725)
-- Name: mm_hardware_specifications mm_hardware_id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_hardware_specifications
    ADD CONSTRAINT mm_hardware_id PRIMARY KEY (mm_hardware_spec_guid);


--
-- TOC entry 3339 (class 2606 OID 417801)
-- Name: mm_hardware_manufacturer mm_hardware_manu_uni_name; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_hardware_manufacturer
    ADD CONSTRAINT mm_hardware_manu_uni_name UNIQUE (mm_hardware_manu_name);


--
-- TOC entry 3341 (class 2606 OID 417819)
-- Name: mm_hardware_manufacturer mm_hardware_manufacturer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_hardware_manufacturer
    ADD CONSTRAINT mm_hardware_manufacturer_pkey PRIMARY KEY (mm_hardware_manu_guid);


--
-- TOC entry 3344 (class 2606 OID 417821)
-- Name: mm_hardware_type mm_hardware_type_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_hardware_type
    ADD CONSTRAINT mm_hardware_type_pkey PRIMARY KEY (mm_hardware_type_guid);


--
-- TOC entry 3346 (class 2606 OID 417824)
-- Name: mm_hardware_type mm_hardware_type_uni_name; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_hardware_type
    ADD CONSTRAINT mm_hardware_type_uni_name UNIQUE (mm_hardware_type_name);


--
-- TOC entry 3177 (class 2606 OID 410727)
-- Name: mm_library_link mm_link_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_library_link
    ADD CONSTRAINT mm_link_guid_pk PRIMARY KEY (mm_link_guid);


--
-- TOC entry 3186 (class 2606 OID 410731)
-- Name: mm_library_dir mm_media_dir_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_library_dir
    ADD CONSTRAINT mm_media_dir_pk PRIMARY KEY (mm_media_dir_guid);


--
-- TOC entry 3184 (class 2606 OID 410733)
-- Name: mm_media mm_media_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_media
    ADD CONSTRAINT mm_media_pk PRIMARY KEY (mm_media_guid);


--
-- TOC entry 3199 (class 2606 OID 410735)
-- Name: mm_metadata_album mm_metadata_album_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_album
    ADD CONSTRAINT mm_metadata_album_pk PRIMARY KEY (mm_metadata_album_guid);


--
-- TOC entry 3207 (class 2606 OID 410737)
-- Name: mm_metadata_anime mm_metadata_anime_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_anime
    ADD CONSTRAINT mm_metadata_anime_pk PRIMARY KEY (mm_metadata_anime_guid);


--
-- TOC entry 3210 (class 2606 OID 410739)
-- Name: mm_metadata_book mm_metadata_book_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_book
    ADD CONSTRAINT mm_metadata_book_pk PRIMARY KEY (mm_metadata_book_guid);


--
-- TOC entry 3216 (class 2606 OID 410741)
-- Name: mm_metadata_collection mm_metadata_collection_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_collection
    ADD CONSTRAINT mm_metadata_collection_guid_pk PRIMARY KEY (mm_metadata_collection_guid);


--
-- TOC entry 3235 (class 2606 OID 410743)
-- Name: mm_metadata_logo mm_metadata_logo_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_logo
    ADD CONSTRAINT mm_metadata_logo_guid_pk PRIMARY KEY (mm_metadata_logo_guid);


--
-- TOC entry 3253 (class 2606 OID 410745)
-- Name: mm_metadata_music mm_metadata_music_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_music
    ADD CONSTRAINT mm_metadata_music_pk PRIMARY KEY (mm_metadata_music_guid);


--
-- TOC entry 3265 (class 2606 OID 410747)
-- Name: mm_metadata_music_video mm_metadata_music_video_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_music_video
    ADD CONSTRAINT mm_metadata_music_video_pk PRIMARY KEY (mm_metadata_music_video_guid);


--
-- TOC entry 3272 (class 2606 OID 410749)
-- Name: mm_metadata_musician mm_metadata_musician_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_musician
    ADD CONSTRAINT mm_metadata_musician_pk PRIMARY KEY (mm_metadata_musician_guid);


--
-- TOC entry 3244 (class 2606 OID 410751)
-- Name: mm_metadata_movie mm_metadata_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_movie
    ADD CONSTRAINT mm_metadata_pk PRIMARY KEY (mm_metadata_movie_guid);


--
-- TOC entry 3291 (class 2606 OID 410753)
-- Name: mm_metadata_sports mm_metadata_sports_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_sports
    ADD CONSTRAINT mm_metadata_sports_pk PRIMARY KEY (mm_metadata_sports_guid);


--
-- TOC entry 3298 (class 2606 OID 410755)
-- Name: mm_metadata_tvshow mm_metadata_tvshow_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_tvshow
    ADD CONSTRAINT mm_metadata_tvshow_pk PRIMARY KEY (mm_metadata_tvshow_guid);


--
-- TOC entry 3301 (class 2606 OID 410757)
-- Name: mm_notification mm_notification_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_notification
    ADD CONSTRAINT mm_notification_pk PRIMARY KEY (mm_notification_guid);


--
-- TOC entry 3303 (class 2606 OID 410759)
-- Name: mm_options_and_status mm_options_and_status_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_options_and_status
    ADD CONSTRAINT mm_options_and_status_guid_pk PRIMARY KEY (mm_options_and_status_guid);


--
-- TOC entry 3335 (class 2606 OID 410898)
-- Name: mm_software_publisher mm_publisher_id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_software_publisher
    ADD CONSTRAINT mm_publisher_id PRIMARY KEY (mm_publisher_guid);


--
-- TOC entry 3305 (class 2606 OID 410761)
-- Name: mm_radio mm_radio_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_radio
    ADD CONSTRAINT mm_radio_guid_pk PRIMARY KEY (mm_radio_guid);


--
-- TOC entry 3310 (class 2606 OID 410763)
-- Name: mm_metadata_review mm_review_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_review
    ADD CONSTRAINT mm_review_pk PRIMARY KEY (mm_review_guid);


--
-- TOC entry 3312 (class 2606 OID 410765)
-- Name: mm_media_sync mm_sync_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_media_sync
    ADD CONSTRAINT mm_sync_guid_pk PRIMARY KEY (mm_sync_guid);


--
-- TOC entry 3314 (class 2606 OID 410767)
-- Name: mm_tv_schedule mm_tv_schedule_id_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_tv_schedule
    ADD CONSTRAINT mm_tv_schedule_id_pk PRIMARY KEY (mm_tv_schedule_id);


--
-- TOC entry 3318 (class 2606 OID 410769)
-- Name: mm_tv_schedule_program mm_tv_schedule_program_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_tv_schedule_program
    ADD CONSTRAINT mm_tv_schedule_program_guid_pk PRIMARY KEY (mm_tv_schedule_program_guid);


--
-- TOC entry 3320 (class 2606 OID 410771)
-- Name: mm_tv_stations mm_tv_stations_id_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_tv_stations
    ADD CONSTRAINT mm_tv_stations_id_pk PRIMARY KEY (mm_tv_stations_id);


--
-- TOC entry 3326 (class 2606 OID 410773)
-- Name: mm_user_group mm_user_group_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_user_group
    ADD CONSTRAINT mm_user_group_guid_pk PRIMARY KEY (mm_user_group_guid);


--
-- TOC entry 3322 (class 2606 OID 410775)
-- Name: mm_user mm_user_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_user
    ADD CONSTRAINT mm_user_pkey PRIMARY KEY (id);


--
-- TOC entry 3328 (class 2606 OID 410777)
-- Name: mm_user_profile mm_user_profile_guid_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_user_profile
    ADD CONSTRAINT mm_user_profile_guid_pk PRIMARY KEY (mm_user_profile_guid);


--
-- TOC entry 3330 (class 2606 OID 410779)
-- Name: mm_user_queue mm_user_queue_id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_user_queue
    ADD CONSTRAINT mm_user_queue_id PRIMARY KEY (mm_user_queue_guid);


--
-- TOC entry 3278 (class 2606 OID 410781)
-- Name: mm_metadata_person mmp_id_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_metadata_person
    ADD CONSTRAINT mmp_id_pk PRIMARY KEY (mm_metadata_person_guid);


--
-- TOC entry 3191 (class 2606 OID 410783)
-- Name: mm_media_remote mmr_media_remote_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.mm_media_remote
    ADD CONSTRAINT mmr_media_remote_pk PRIMARY KEY (mmr_media_guid);


--
-- TOC entry 3166 (class 1259 OID 410784)
-- Name: gc_category_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX gc_category_idx_name ON public.mm_software_category USING btree (mm_software_category_category);


--
-- TOC entry 3220 (class 1259 OID 410785)
-- Name: gi_game_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX gi_game_idx_name ON public.mm_metadata_game_software_info USING btree (gi_game_info_name);


--
-- TOC entry 3221 (class 1259 OID 410786)
-- Name: gi_game_idx_name_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX gi_game_idx_name_trigram_idx ON public.mm_metadata_game_software_info USING gist (gi_game_info_name public.gist_trgm_ops);


--
-- TOC entry 3222 (class 1259 OID 410787)
-- Name: gi_game_idx_short_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX gi_game_idx_short_name ON public.mm_metadata_game_software_info USING btree (gi_game_info_short_name);


--
-- TOC entry 3225 (class 1259 OID 410788)
-- Name: gi_system_id_ndx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX gi_system_id_ndx ON public.mm_metadata_game_software_info USING btree (gi_system_id);


--
-- TOC entry 3162 (class 1259 OID 410789)
-- Name: mdq_que_type_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mdq_que_type_idx_name ON public.mm_metadata_download_que USING btree (mm_download_que_type);


--
-- TOC entry 3150 (class 1259 OID 410790)
-- Name: mm_channel_idx_country; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_channel_idx_country ON public.mm_media_channel USING btree (mm_channel_country_guid);


--
-- TOC entry 3151 (class 1259 OID 410791)
-- Name: mm_channel_idx_logo; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_channel_idx_logo ON public.mm_media_channel USING btree (mm_channel_logo_guid);


--
-- TOC entry 3152 (class 1259 OID 410792)
-- Name: mm_channel_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_channel_idx_name ON public.mm_media_channel USING btree (mm_channel_name);


--
-- TOC entry 3153 (class 1259 OID 410793)
-- Name: mm_channel_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_channel_idxgin_json ON public.mm_media_channel USING gin (mm_channel_media_id);


--
-- TOC entry 3333 (class 1259 OID 410890)
-- Name: mm_developer_name_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_developer_name_idx ON public.mm_software_developer USING btree (mm_developer_name);


--
-- TOC entry 3158 (class 1259 OID 410795)
-- Name: mm_device_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_device_idxgin_json ON public.mm_hardware_device USING gin (mm_device_json);


--
-- TOC entry 3163 (class 1259 OID 410796)
-- Name: mm_download_idx_provider; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_download_idx_provider ON public.mm_metadata_download_que USING btree (mm_download_provider);


--
-- TOC entry 3164 (class 1259 OID 410916)
-- Name: mm_download_que_ndx_provider_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_download_que_ndx_provider_id ON public.mm_metadata_download_que USING btree (mm_download_provider_id);


--
-- TOC entry 3165 (class 1259 OID 410917)
-- Name: mm_download_que_ndx_status; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_download_que_ndx_status ON public.mm_metadata_download_que USING btree (mm_download_status);


--
-- TOC entry 3226 (class 1259 OID 410798)
-- Name: mm_game_info_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_game_info_idxgin_json ON public.mm_metadata_game_software_info USING gin (gi_game_info_json);


--
-- TOC entry 3227 (class 1259 OID 410799)
-- Name: mm_game_info_idxgin_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_game_info_idxgin_name ON public.mm_metadata_game_software_info USING gin (((gi_game_info_json -> '@name'::text)));


--
-- TOC entry 3171 (class 1259 OID 410800)
-- Name: mm_game_server_name_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_game_server_name_idx ON public.mm_game_dedicated_servers USING btree (mm_game_server_name);


--
-- TOC entry 3232 (class 1259 OID 410801)
-- Name: mm_game_systems_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_game_systems_idxgin_json ON public.mm_metadata_game_systems_info USING gin (gs_game_system_json);


--
-- TOC entry 3159 (class 1259 OID 417826)
-- Name: mm_hardware_device_idx_spec_uuid; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_hardware_device_idx_spec_uuid ON public.mm_hardware_device USING btree (mm_device_spec_guid);


--
-- TOC entry 3174 (class 1259 OID 410803)
-- Name: mm_hardware_idx_model; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_hardware_idx_model ON public.mm_hardware_specifications USING btree (mm_hardware_spec_model);


--
-- TOC entry 3337 (class 1259 OID 417799)
-- Name: mm_hardware_manu_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_hardware_manu_idx_name ON public.mm_hardware_manufacturer USING btree (mm_hardware_manu_name);


--
-- TOC entry 3175 (class 1259 OID 417825)
-- Name: mm_hardware_spec_idx_manu_uuid; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_hardware_spec_idx_manu_uuid ON public.mm_hardware_specifications USING btree (mm_hardware_spec_manufacturer_guid);


--
-- TOC entry 3342 (class 1259 OID 417822)
-- Name: mm_hardware_type_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_hardware_type_idx_name ON public.mm_hardware_type USING btree (mm_hardware_type_name);


--
-- TOC entry 3178 (class 1259 OID 410804)
-- Name: mm_link_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_link_idx_name ON public.mm_library_link USING btree (mm_link_name);


--
-- TOC entry 3179 (class 1259 OID 410805)
-- Name: mm_link_json_idxgin; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_link_json_idxgin ON public.mm_library_link USING gin (mm_link_json);


--
-- TOC entry 3200 (class 1259 OID 410806)
-- Name: mm_media_anime_name_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_anime_name_trigram_idx ON public.mm_metadata_anime USING gist (mm_metadata_anime_name public.gist_trgm_ops);


--
-- TOC entry 3180 (class 1259 OID 410807)
-- Name: mm_media_idx_metadata_uuid; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_idx_metadata_uuid ON public.mm_media USING btree (mm_media_metadata_guid);


--
-- TOC entry 3181 (class 1259 OID 410808)
-- Name: mm_media_idx_path; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_idx_path ON public.mm_media USING btree (mm_media_path);


--
-- TOC entry 3182 (class 1259 OID 410809)
-- Name: mm_media_idxgin_ffprobe; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_idxgin_ffprobe ON public.mm_media USING gin (mm_media_ffprobe_json);


--
-- TOC entry 3254 (class 1259 OID 410810)
-- Name: mm_media_music_video_band_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_music_video_band_trigram_idx ON public.mm_metadata_music_video USING gist (mm_metadata_music_video_band public.gist_trgm_ops);


--
-- TOC entry 3255 (class 1259 OID 410811)
-- Name: mm_media_music_video_song_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_music_video_song_trigram_idx ON public.mm_metadata_music_video USING gist (mm_metadata_music_video_song public.gist_trgm_ops);


--
-- TOC entry 3237 (class 1259 OID 410812)
-- Name: mm_media_name_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_name_trigram_idx ON public.mm_metadata_movie USING gist (mm_metadata_movie_name public.gist_trgm_ops);


--
-- TOC entry 3187 (class 1259 OID 410906)
-- Name: mm_media_remote_ndx_link_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_remote_ndx_link_id ON public.mm_media_remote USING btree (mmr_media_link_guid);


--
-- TOC entry 3188 (class 1259 OID 410907)
-- Name: mm_media_remote_ndx_media_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_remote_ndx_media_id ON public.mm_media_remote USING btree (mmr_media_uuid);


--
-- TOC entry 3189 (class 1259 OID 410908)
-- Name: mm_media_remote_ndx_meta_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_media_remote_ndx_meta_id ON public.mm_media_remote USING btree (mmr_media_metadata_guid);


--
-- TOC entry 3192 (class 1259 OID 410813)
-- Name: mm_metadata_album_idx_musician; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_album_idx_musician ON public.mm_metadata_album USING btree (mm_metadata_album_musician_guid);


--
-- TOC entry 3193 (class 1259 OID 410814)
-- Name: mm_metadata_album_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_album_idx_name ON public.mm_metadata_album USING btree (mm_metadata_album_name);


--
-- TOC entry 3194 (class 1259 OID 410815)
-- Name: mm_metadata_album_idx_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_album_idx_name_lower ON public.mm_metadata_album USING btree (lower(mm_metadata_album_name));


--
-- TOC entry 3195 (class 1259 OID 410816)
-- Name: mm_metadata_album_idxgin_id_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_album_idxgin_id_json ON public.mm_metadata_album USING gin (mm_metadata_album_id);


--
-- TOC entry 3196 (class 1259 OID 410817)
-- Name: mm_metadata_album_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_album_idxgin_json ON public.mm_metadata_album USING gin (mm_metadata_album_json);


--
-- TOC entry 3197 (class 1259 OID 410818)
-- Name: mm_metadata_album_name_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_album_name_trigram_idx ON public.mm_metadata_album USING gist (mm_metadata_album_name public.gist_trgm_ops);


--
-- TOC entry 3201 (class 1259 OID 410820)
-- Name: mm_metadata_anime_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_anime_idx_name ON public.mm_metadata_anime USING btree (mm_metadata_anime_name);


--
-- TOC entry 3202 (class 1259 OID 410821)
-- Name: mm_metadata_anime_idx_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_anime_idx_name_lower ON public.mm_metadata_anime USING btree (lower(mm_metadata_anime_name));


--
-- TOC entry 3203 (class 1259 OID 410822)
-- Name: mm_metadata_anime_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_anime_idxgin_json ON public.mm_metadata_anime USING gin (mm_metadata_anime_json);


--
-- TOC entry 3204 (class 1259 OID 410827)
-- Name: mm_metadata_anime_idxgin_user_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_anime_idxgin_user_json ON public.mm_metadata_anime USING gin (mm_metadata_anime_user_json);


--
-- TOC entry 3205 (class 1259 OID 410909)
-- Name: mm_metadata_anime_ndx_media_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_anime_ndx_media_id ON public.mm_metadata_anime USING btree (mm_metadata_anime_media_id);


--
-- TOC entry 3208 (class 1259 OID 410828)
-- Name: mm_metadata_book_name_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_book_name_trigram_idx ON public.mm_metadata_book USING gist (mm_metadata_book_name public.gist_trgm_ops);


--
-- TOC entry 3217 (class 1259 OID 410829)
-- Name: mm_metadata_collection_idxgin_media_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_collection_idxgin_media_json ON public.mm_metadata_collection USING gin (mm_metadata_collection_media_ids);


--
-- TOC entry 3218 (class 1259 OID 410830)
-- Name: mm_metadata_collection_idxgin_meta_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_collection_idxgin_meta_json ON public.mm_metadata_collection USING gin (mm_metadata_collection_json);


--
-- TOC entry 3219 (class 1259 OID 410831)
-- Name: mm_metadata_collection_idxgin_name_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_collection_idxgin_name_json ON public.mm_metadata_collection USING gin (mm_metadata_collection_name);


--
-- TOC entry 3228 (class 1259 OID 410901)
-- Name: mm_metadata_game_software_info_blake3_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_game_software_info_blake3_idx ON public.mm_metadata_game_software_info USING btree (gi_game_info_blake3);


--
-- TOC entry 3229 (class 1259 OID 410900)
-- Name: mm_metadata_game_software_info_sha1_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_game_software_info_sha1_idx ON public.mm_metadata_game_software_info USING btree (gi_game_info_sha1);


--
-- TOC entry 3233 (class 1259 OID 410910)
-- Name: mm_metadata_game_systems_info_ndx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_game_systems_info_ndx_name ON public.mm_metadata_game_systems_info USING btree (gs_game_system_name);


--
-- TOC entry 3256 (class 1259 OID 410832)
-- Name: mm_metadata_idx_band_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idx_band_name ON public.mm_metadata_music_video USING btree (mm_metadata_music_video_band);


--
-- TOC entry 3257 (class 1259 OID 410833)
-- Name: mm_metadata_idx_band_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idx_band_name_lower ON public.mm_metadata_music_video USING btree (lower(mm_metadata_music_video_band));


--
-- TOC entry 3211 (class 1259 OID 410834)
-- Name: mm_metadata_idx_book_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idx_book_name ON public.mm_metadata_book USING btree (mm_metadata_book_name);


--
-- TOC entry 3212 (class 1259 OID 410835)
-- Name: mm_metadata_idx_book_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idx_book_name_lower ON public.mm_metadata_book USING btree (lower(mm_metadata_book_name));


--
-- TOC entry 3238 (class 1259 OID 410836)
-- Name: mm_metadata_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idx_name ON public.mm_metadata_movie USING btree (mm_metadata_movie_name);


--
-- TOC entry 3239 (class 1259 OID 410837)
-- Name: mm_metadata_idx_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idx_name_lower ON public.mm_metadata_movie USING btree (lower(mm_metadata_movie_name));


--
-- TOC entry 3258 (class 1259 OID 410838)
-- Name: mm_metadata_idx_song_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idx_song_name ON public.mm_metadata_music_video USING btree (mm_metadata_music_video_song);


--
-- TOC entry 3259 (class 1259 OID 410839)
-- Name: mm_metadata_idx_song_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idx_song_name_lower ON public.mm_metadata_music_video USING btree (lower(mm_metadata_music_video_song));


--
-- TOC entry 3213 (class 1259 OID 410840)
-- Name: mm_metadata_idxgin_isbn; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idxgin_isbn ON public.mm_metadata_book USING btree (mm_metadata_book_isbn);


--
-- TOC entry 3214 (class 1259 OID 410841)
-- Name: mm_metadata_idxgin_isbn13; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idxgin_isbn13 ON public.mm_metadata_book USING btree (mm_metadata_book_isbn13);


--
-- TOC entry 3240 (class 1259 OID 410842)
-- Name: mm_metadata_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idxgin_json ON public.mm_metadata_movie USING gin (mm_metadata_movie_json);


--
-- TOC entry 3260 (class 1259 OID 410843)
-- Name: mm_metadata_idxgin_music_video_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idxgin_music_video_json ON public.mm_metadata_music_video USING gin (mm_metadata_music_video_json);


--
-- TOC entry 3261 (class 1259 OID 410844)
-- Name: mm_metadata_idxgin_music_video_media_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idxgin_music_video_media_id ON public.mm_metadata_music_video USING gin (mm_metadata_music_video_media_id);


--
-- TOC entry 3262 (class 1259 OID 410845)
-- Name: mm_metadata_idxgin_music_video_media_id_imvdb; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idxgin_music_video_media_id_imvdb ON public.mm_metadata_music_video USING gin (((mm_metadata_music_video_media_id -> 'imvdb'::text)));


--
-- TOC entry 3241 (class 1259 OID 410846)
-- Name: mm_metadata_idxgin_user_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_idxgin_user_json ON public.mm_metadata_movie USING gin (mm_metadata_movie_user_json);


--
-- TOC entry 3236 (class 1259 OID 410847)
-- Name: mm_metadata_logo_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_logo_idxgin_json ON public.mm_metadata_logo USING gin (mm_metadata_logo_media_guid);


--
-- TOC entry 3292 (class 1259 OID 410881)
-- Name: mm_metadata_media_tvshow_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_media_tvshow_id_idx ON public.mm_metadata_tvshow USING btree (mm_metadata_media_tvshow_id);


--
-- TOC entry 3242 (class 1259 OID 410902)
-- Name: mm_metadata_movie_ndx_media_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_movie_ndx_media_id ON public.mm_metadata_movie USING btree (mm_metadata_movie_media_id);


--
-- TOC entry 3245 (class 1259 OID 410848)
-- Name: mm_metadata_music_idx_album; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_music_idx_album ON public.mm_metadata_music USING btree (mm_metadata_music_album_guid);


--
-- TOC entry 3246 (class 1259 OID 410849)
-- Name: mm_metadata_music_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_music_idx_name ON public.mm_metadata_music USING btree (mm_metadata_music_name);


--
-- TOC entry 3247 (class 1259 OID 410850)
-- Name: mm_metadata_music_idx_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_music_idx_name_lower ON public.mm_metadata_music USING btree (lower(mm_metadata_music_name));


--
-- TOC entry 3248 (class 1259 OID 410851)
-- Name: mm_metadata_music_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_music_idxgin_json ON public.mm_metadata_music USING gin (mm_metadata_music_json);


--
-- TOC entry 3249 (class 1259 OID 410852)
-- Name: mm_metadata_music_idxgin_media_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_music_idxgin_media_id ON public.mm_metadata_music USING gin (mm_metadata_media_music_id);


--
-- TOC entry 3250 (class 1259 OID 410853)
-- Name: mm_metadata_music_idxgin_user_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_music_idxgin_user_json ON public.mm_metadata_music USING gin (mm_metadata_music_user_json);


--
-- TOC entry 3251 (class 1259 OID 410854)
-- Name: mm_metadata_music_name_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_music_name_trigram_idx ON public.mm_metadata_music USING gist (mm_metadata_music_name public.gist_trgm_ops);


--
-- TOC entry 3263 (class 1259 OID 410855)
-- Name: mm_metadata_music_video_idxgin_user_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_music_video_idxgin_user_json ON public.mm_metadata_music_video USING gin (mm_metadata_music_video_user_json);


--
-- TOC entry 3266 (class 1259 OID 410856)
-- Name: mm_metadata_musician_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_musician_idx_name ON public.mm_metadata_musician USING btree (mm_metadata_musician_name);


--
-- TOC entry 3267 (class 1259 OID 410857)
-- Name: mm_metadata_musician_idx_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_musician_idx_name_lower ON public.mm_metadata_musician USING btree (lower(mm_metadata_musician_name));


--
-- TOC entry 3268 (class 1259 OID 410858)
-- Name: mm_metadata_musician_idxgin_id_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_musician_idxgin_id_json ON public.mm_metadata_musician USING gin (mm_metadata_musician_id);


--
-- TOC entry 3269 (class 1259 OID 410859)
-- Name: mm_metadata_musician_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_musician_idxgin_json ON public.mm_metadata_musician USING gin (mm_metadata_musician_json);


--
-- TOC entry 3270 (class 1259 OID 410860)
-- Name: mm_metadata_musician_name_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_musician_name_trigram_idx ON public.mm_metadata_musician USING gist (mm_metadata_musician_name public.gist_trgm_ops);


--
-- TOC entry 3273 (class 1259 OID 410861)
-- Name: mm_metadata_person_idx_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_person_idx_id ON public.mm_metadata_person USING btree (mm_metadata_person_media_id);


--
-- TOC entry 3274 (class 1259 OID 410862)
-- Name: mm_metadata_person_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_person_idx_name ON public.mm_metadata_person USING btree (mm_metadata_person_name);


--
-- TOC entry 3275 (class 1259 OID 410918)
-- Name: mm_metadata_person_idx_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_person_idx_name_lower ON public.mm_metadata_person USING btree (lower(mm_metadata_person_name));


--
-- TOC entry 3276 (class 1259 OID 410863)
-- Name: mm_metadata_person_idxgin_meta_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_person_idxgin_meta_json ON public.mm_metadata_person USING gin (mm_metadata_person_meta_json);


--
-- TOC entry 3308 (class 1259 OID 410864)
-- Name: mm_metadata_review_idx_metadata_uuid; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_review_idx_metadata_uuid ON public.mm_metadata_review USING btree (mm_review_metadata_guid);


--
-- TOC entry 3279 (class 1259 OID 410866)
-- Name: mm_metadata_sports_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idx_name ON public.mm_metadata_sports USING btree (mm_metadata_sports_name);


--
-- TOC entry 3280 (class 1259 OID 410867)
-- Name: mm_metadata_sports_idx_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idx_name_lower ON public.mm_metadata_sports USING btree (lower(mm_metadata_sports_name));


--
-- TOC entry 3281 (class 1259 OID 410868)
-- Name: mm_metadata_sports_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idxgin_json ON public.mm_metadata_sports USING gin (mm_metadata_sports_json);


--
-- TOC entry 3282 (class 1259 OID 410869)
-- Name: mm_metadata_sports_idxgin_media_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idxgin_media_id ON public.mm_metadata_sports USING gin (mm_metadata_media_sports_id);


--
-- TOC entry 3283 (class 1259 OID 410870)
-- Name: mm_metadata_sports_idxgin_media_id_imdb; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idxgin_media_id_imdb ON public.mm_metadata_sports USING gin (((mm_metadata_media_sports_id -> 'imdb'::text)));


--
-- TOC entry 3284 (class 1259 OID 410871)
-- Name: mm_metadata_sports_idxgin_media_id_thesportsdb; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idxgin_media_id_thesportsdb ON public.mm_metadata_sports USING gin (((mm_metadata_media_sports_id -> 'thesportsdb'::text)));


--
-- TOC entry 3285 (class 1259 OID 410872)
-- Name: mm_metadata_sports_idxgin_media_id_thetvdb; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idxgin_media_id_thetvdb ON public.mm_metadata_sports USING gin (((mm_metadata_media_sports_id -> 'thetvdb'::text)));


--
-- TOC entry 3286 (class 1259 OID 410873)
-- Name: mm_metadata_sports_idxgin_media_id_thetvdbseries; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idxgin_media_id_thetvdbseries ON public.mm_metadata_sports USING gin (((mm_metadata_media_sports_id -> 'thetvdbSeries'::text)));


--
-- TOC entry 3287 (class 1259 OID 410874)
-- Name: mm_metadata_sports_idxgin_media_id_tmdb; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idxgin_media_id_tmdb ON public.mm_metadata_sports USING gin (((mm_metadata_media_sports_id -> 'tmdb'::text)));


--
-- TOC entry 3288 (class 1259 OID 410875)
-- Name: mm_metadata_sports_idxgin_media_id_tvmaze; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_idxgin_media_id_tvmaze ON public.mm_metadata_sports USING gin (((mm_metadata_media_sports_id -> 'tvmaze'::text)));


--
-- TOC entry 3289 (class 1259 OID 410876)
-- Name: mm_metadata_sports_name_trigram_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_sports_name_trigram_idx ON public.mm_metadata_sports USING gist (mm_metadata_sports_name public.gist_trgm_ops);


--
-- TOC entry 3293 (class 1259 OID 410877)
-- Name: mm_metadata_tvshow_idx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_tvshow_idx_name ON public.mm_metadata_tvshow USING btree (mm_metadata_tvshow_name);


--
-- TOC entry 3294 (class 1259 OID 410878)
-- Name: mm_metadata_tvshow_idx_name_lower; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_tvshow_idx_name_lower ON public.mm_metadata_tvshow USING btree (lower(mm_metadata_tvshow_name));


--
-- TOC entry 3295 (class 1259 OID 410879)
-- Name: mm_metadata_tvshow_idxgin_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_tvshow_idxgin_json ON public.mm_metadata_tvshow USING gin (mm_metadata_tvshow_json);


--
-- TOC entry 3296 (class 1259 OID 410880)
-- Name: mm_metadata_tvshow_idxgin_localimage_json; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_metadata_tvshow_idxgin_localimage_json ON public.mm_metadata_tvshow USING gin (mm_metadata_tvshow_json);


--
-- TOC entry 3299 (class 1259 OID 410947)
-- Name: mm_notification_ndx_time; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_notification_ndx_time ON public.mm_notification USING btree (mm_notification_time);


--
-- TOC entry 3336 (class 1259 OID 410899)
-- Name: mm_publisher_name_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_publisher_name_idx ON public.mm_software_publisher USING btree (mm_publisher_name);


--
-- TOC entry 3306 (class 1259 OID 410913)
-- Name: mm_radio_ndx_active; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_radio_ndx_active ON public.mm_radio USING btree (mm_radio_active);


--
-- TOC entry 3307 (class 1259 OID 410912)
-- Name: mm_radio_ndx_name; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_radio_ndx_name ON public.mm_radio USING btree (mm_radio_name);


--
-- TOC entry 3315 (class 1259 OID 410915)
-- Name: mm_tv_schedule_ndx_schedule_date; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_tv_schedule_ndx_schedule_date ON public.mm_tv_schedule USING btree (mm_tv_schedule_date);


--
-- TOC entry 3316 (class 1259 OID 410914)
-- Name: mm_tv_schedule_ndx_station_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX mm_tv_schedule_ndx_station_id ON public.mm_tv_schedule USING btree (mm_tv_schedule_station_id);


-- Completed on 2021-07-31 15:10:02

--
-- PostgreSQL database dump complete
--


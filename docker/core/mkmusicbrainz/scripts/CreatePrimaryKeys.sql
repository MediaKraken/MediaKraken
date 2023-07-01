ALTER TABLE alternative_medium ADD CONSTRAINT alternative_medium_pkey PRIMARY KEY (id);
ALTER TABLE alternative_medium_track ADD CONSTRAINT alternative_medium_track_pkey PRIMARY KEY (alternative_medium, track);
ALTER TABLE alternative_release ADD CONSTRAINT alternative_release_pkey PRIMARY KEY (id);
ALTER TABLE alternative_release_type ADD CONSTRAINT alternative_release_type_pkey PRIMARY KEY (id);
ALTER TABLE alternative_track ADD CONSTRAINT alternative_track_pkey PRIMARY KEY (id);
ALTER TABLE annotation ADD CONSTRAINT annotation_pkey PRIMARY KEY (id);
ALTER TABLE application ADD CONSTRAINT application_pkey PRIMARY KEY (id);
ALTER TABLE area ADD CONSTRAINT area_pkey PRIMARY KEY (id);
ALTER TABLE area_alias ADD CONSTRAINT area_alias_pkey PRIMARY KEY (id);
ALTER TABLE area_alias_type ADD CONSTRAINT area_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE area_annotation ADD CONSTRAINT area_annotation_pkey PRIMARY KEY (area, annotation);
ALTER TABLE area_attribute ADD CONSTRAINT area_attribute_pkey PRIMARY KEY (id);
ALTER TABLE area_attribute_type ADD CONSTRAINT area_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE area_attribute_type_allowed_value ADD CONSTRAINT area_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE area_containment ADD CONSTRAINT area_containment_pkey PRIMARY KEY (descendant, parent);
ALTER TABLE area_gid_redirect ADD CONSTRAINT area_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE area_tag ADD CONSTRAINT area_tag_pkey PRIMARY KEY (area, tag);
ALTER TABLE area_tag_raw ADD CONSTRAINT area_tag_raw_pkey PRIMARY KEY (area, editor, tag);
ALTER TABLE area_type ADD CONSTRAINT area_type_pkey PRIMARY KEY (id);
ALTER TABLE artist ADD CONSTRAINT artist_pkey PRIMARY KEY (id);
ALTER TABLE artist_alias ADD CONSTRAINT artist_alias_pkey PRIMARY KEY (id);
ALTER TABLE artist_alias_type ADD CONSTRAINT artist_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE artist_annotation ADD CONSTRAINT artist_annotation_pkey PRIMARY KEY (artist, annotation);
ALTER TABLE artist_attribute ADD CONSTRAINT artist_attribute_pkey PRIMARY KEY (id);
ALTER TABLE artist_attribute_type ADD CONSTRAINT artist_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE artist_attribute_type_allowed_value ADD CONSTRAINT artist_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE artist_credit ADD CONSTRAINT artist_credit_pkey PRIMARY KEY (id);
ALTER TABLE artist_credit_gid_redirect ADD CONSTRAINT artist_credit_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE artist_credit_name ADD CONSTRAINT artist_credit_name_pkey PRIMARY KEY (artist_credit, position);
ALTER TABLE artist_gid_redirect ADD CONSTRAINT artist_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE artist_ipi ADD CONSTRAINT artist_ipi_pkey PRIMARY KEY (artist, ipi);
ALTER TABLE artist_isni ADD CONSTRAINT artist_isni_pkey PRIMARY KEY (artist, isni);
ALTER TABLE artist_meta ADD CONSTRAINT artist_meta_pkey PRIMARY KEY (id);
ALTER TABLE artist_rating_raw ADD CONSTRAINT artist_rating_raw_pkey PRIMARY KEY (artist, editor);
ALTER TABLE artist_tag ADD CONSTRAINT artist_tag_pkey PRIMARY KEY (artist, tag);
ALTER TABLE artist_tag_raw ADD CONSTRAINT artist_tag_raw_pkey PRIMARY KEY (artist, editor, tag);
ALTER TABLE artist_type ADD CONSTRAINT artist_type_pkey PRIMARY KEY (id);
ALTER TABLE autoeditor_election ADD CONSTRAINT autoeditor_election_pkey PRIMARY KEY (id);
ALTER TABLE autoeditor_election_vote ADD CONSTRAINT autoeditor_election_vote_pkey PRIMARY KEY (id);
ALTER TABLE cdtoc ADD CONSTRAINT cdtoc_pkey PRIMARY KEY (id);
ALTER TABLE cdtoc_raw ADD CONSTRAINT cdtoc_raw_pkey PRIMARY KEY (id);
ALTER TABLE country_area ADD CONSTRAINT country_area_pkey PRIMARY KEY (area);
ALTER TABLE deleted_entity ADD CONSTRAINT deleted_entity_pkey PRIMARY KEY (gid);
ALTER TABLE edit ADD CONSTRAINT edit_pkey PRIMARY KEY (id);
ALTER TABLE edit_area ADD CONSTRAINT edit_area_pkey PRIMARY KEY (edit, area);
ALTER TABLE edit_artist ADD CONSTRAINT edit_artist_pkey PRIMARY KEY (edit, artist);
ALTER TABLE edit_data ADD CONSTRAINT edit_data_pkey PRIMARY KEY (edit);
ALTER TABLE edit_event ADD CONSTRAINT edit_event_pkey PRIMARY KEY (edit, event);
ALTER TABLE edit_genre ADD CONSTRAINT edit_genre_pkey PRIMARY KEY (edit, genre);
ALTER TABLE edit_instrument ADD CONSTRAINT edit_instrument_pkey PRIMARY KEY (edit, instrument);
ALTER TABLE edit_label ADD CONSTRAINT edit_label_pkey PRIMARY KEY (edit, label);
ALTER TABLE edit_mood ADD CONSTRAINT edit_mood_pkey PRIMARY KEY (edit, mood);
ALTER TABLE edit_note ADD CONSTRAINT edit_note_pkey PRIMARY KEY (id);
ALTER TABLE edit_note_change ADD CONSTRAINT edit_note_change_pkey PRIMARY KEY (id);
ALTER TABLE edit_note_recipient ADD CONSTRAINT edit_note_recipient_pkey PRIMARY KEY (recipient, edit_note);
ALTER TABLE edit_place ADD CONSTRAINT edit_place_pkey PRIMARY KEY (edit, place);
ALTER TABLE edit_recording ADD CONSTRAINT edit_recording_pkey PRIMARY KEY (edit, recording);
ALTER TABLE edit_release ADD CONSTRAINT edit_release_pkey PRIMARY KEY (edit, release);
ALTER TABLE edit_release_group ADD CONSTRAINT edit_release_group_pkey PRIMARY KEY (edit, release_group);
ALTER TABLE edit_series ADD CONSTRAINT edit_series_pkey PRIMARY KEY (edit, series);
ALTER TABLE edit_url ADD CONSTRAINT edit_url_pkey PRIMARY KEY (edit, url);
ALTER TABLE edit_work ADD CONSTRAINT edit_work_pkey PRIMARY KEY (edit, work);
ALTER TABLE editor ADD CONSTRAINT editor_pkey PRIMARY KEY (id);
ALTER TABLE editor_collection ADD CONSTRAINT editor_collection_pkey PRIMARY KEY (id);
ALTER TABLE editor_collection_area ADD CONSTRAINT editor_collection_area_pkey PRIMARY KEY (collection, area);
ALTER TABLE editor_collection_artist ADD CONSTRAINT editor_collection_artist_pkey PRIMARY KEY (collection, artist);
ALTER TABLE editor_collection_collaborator ADD CONSTRAINT editor_collection_collaborator_pkey PRIMARY KEY (collection, editor);
ALTER TABLE editor_collection_deleted_entity ADD CONSTRAINT editor_collection_deleted_entity_pkey PRIMARY KEY (collection, gid);
ALTER TABLE editor_collection_event ADD CONSTRAINT editor_collection_event_pkey PRIMARY KEY (collection, event);
ALTER TABLE editor_collection_gid_redirect ADD CONSTRAINT editor_collection_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE editor_collection_instrument ADD CONSTRAINT editor_collection_instrument_pkey PRIMARY KEY (collection, instrument);
ALTER TABLE editor_collection_label ADD CONSTRAINT editor_collection_label_pkey PRIMARY KEY (collection, label);
ALTER TABLE editor_collection_place ADD CONSTRAINT editor_collection_place_pkey PRIMARY KEY (collection, place);
ALTER TABLE editor_collection_recording ADD CONSTRAINT editor_collection_recording_pkey PRIMARY KEY (collection, recording);
ALTER TABLE editor_collection_release ADD CONSTRAINT editor_collection_release_pkey PRIMARY KEY (collection, release);
ALTER TABLE editor_collection_release_group ADD CONSTRAINT editor_collection_release_group_pkey PRIMARY KEY (collection, release_group);
ALTER TABLE editor_collection_series ADD CONSTRAINT editor_collection_series_pkey PRIMARY KEY (collection, series);
ALTER TABLE editor_collection_type ADD CONSTRAINT editor_collection_type_pkey PRIMARY KEY (id);
ALTER TABLE editor_collection_work ADD CONSTRAINT editor_collection_work_pkey PRIMARY KEY (collection, work);
ALTER TABLE editor_language ADD CONSTRAINT editor_language_pkey PRIMARY KEY (editor, language);
ALTER TABLE editor_oauth_token ADD CONSTRAINT editor_oauth_token_pkey PRIMARY KEY (id);
ALTER TABLE editor_preference ADD CONSTRAINT editor_preference_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_artist ADD CONSTRAINT editor_subscribe_artist_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_artist_deleted ADD CONSTRAINT editor_subscribe_artist_deleted_pkey PRIMARY KEY (editor, gid);
ALTER TABLE editor_subscribe_collection ADD CONSTRAINT editor_subscribe_collection_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_editor ADD CONSTRAINT editor_subscribe_editor_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_label ADD CONSTRAINT editor_subscribe_label_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_label_deleted ADD CONSTRAINT editor_subscribe_label_deleted_pkey PRIMARY KEY (editor, gid);
ALTER TABLE editor_subscribe_series ADD CONSTRAINT editor_subscribe_series_pkey PRIMARY KEY (id);
ALTER TABLE editor_subscribe_series_deleted ADD CONSTRAINT editor_subscribe_series_deleted_pkey PRIMARY KEY (editor, gid);
ALTER TABLE event ADD CONSTRAINT event_pkey PRIMARY KEY (id);
ALTER TABLE event_alias ADD CONSTRAINT event_alias_pkey PRIMARY KEY (id);
ALTER TABLE event_alias_type ADD CONSTRAINT event_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE event_annotation ADD CONSTRAINT event_annotation_pkey PRIMARY KEY (event, annotation);
ALTER TABLE event_attribute ADD CONSTRAINT event_attribute_pkey PRIMARY KEY (id);
ALTER TABLE event_attribute_type ADD CONSTRAINT event_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE event_attribute_type_allowed_value ADD CONSTRAINT event_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE event_gid_redirect ADD CONSTRAINT event_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE event_meta ADD CONSTRAINT event_meta_pkey PRIMARY KEY (id);
ALTER TABLE event_rating_raw ADD CONSTRAINT event_rating_raw_pkey PRIMARY KEY (event, editor);
ALTER TABLE event_tag ADD CONSTRAINT event_tag_pkey PRIMARY KEY (event, tag);
ALTER TABLE event_tag_raw ADD CONSTRAINT event_tag_raw_pkey PRIMARY KEY (event, editor, tag);
ALTER TABLE event_type ADD CONSTRAINT event_type_pkey PRIMARY KEY (id);
ALTER TABLE gender ADD CONSTRAINT gender_pkey PRIMARY KEY (id);
ALTER TABLE genre ADD CONSTRAINT genre_pkey PRIMARY KEY (id);
ALTER TABLE genre_alias ADD CONSTRAINT genre_alias_pkey PRIMARY KEY (id);
ALTER TABLE genre_alias_type ADD CONSTRAINT genre_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE genre_annotation ADD CONSTRAINT genre_annotation_pkey PRIMARY KEY (genre, annotation);
ALTER TABLE instrument ADD CONSTRAINT instrument_pkey PRIMARY KEY (id);
ALTER TABLE instrument_alias ADD CONSTRAINT instrument_alias_pkey PRIMARY KEY (id);
ALTER TABLE instrument_alias_type ADD CONSTRAINT instrument_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE instrument_annotation ADD CONSTRAINT instrument_annotation_pkey PRIMARY KEY (instrument, annotation);
ALTER TABLE instrument_attribute ADD CONSTRAINT instrument_attribute_pkey PRIMARY KEY (id);
ALTER TABLE instrument_attribute_type ADD CONSTRAINT instrument_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE instrument_attribute_type_allowed_value ADD CONSTRAINT instrument_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE instrument_gid_redirect ADD CONSTRAINT instrument_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE instrument_tag ADD CONSTRAINT instrument_tag_pkey PRIMARY KEY (instrument, tag);
ALTER TABLE instrument_tag_raw ADD CONSTRAINT instrument_tag_raw_pkey PRIMARY KEY (instrument, editor, tag);
ALTER TABLE instrument_type ADD CONSTRAINT instrument_type_pkey PRIMARY KEY (id);
ALTER TABLE iso_3166_1 ADD CONSTRAINT iso_3166_1_pkey PRIMARY KEY (code);
ALTER TABLE iso_3166_2 ADD CONSTRAINT iso_3166_2_pkey PRIMARY KEY (code);
ALTER TABLE iso_3166_3 ADD CONSTRAINT iso_3166_3_pkey PRIMARY KEY (code);
ALTER TABLE isrc ADD CONSTRAINT isrc_pkey PRIMARY KEY (id);
ALTER TABLE iswc ADD CONSTRAINT iswc_pkey PRIMARY KEY (id);
ALTER TABLE l_area_area ADD CONSTRAINT l_area_area_pkey PRIMARY KEY (id);
ALTER TABLE l_area_artist ADD CONSTRAINT l_area_artist_pkey PRIMARY KEY (id);
ALTER TABLE l_area_event ADD CONSTRAINT l_area_event_pkey PRIMARY KEY (id);
ALTER TABLE l_area_genre ADD CONSTRAINT l_area_genre_pkey PRIMARY KEY (id);
ALTER TABLE l_area_instrument ADD CONSTRAINT l_area_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_area_label ADD CONSTRAINT l_area_label_pkey PRIMARY KEY (id);
ALTER TABLE l_area_mood ADD CONSTRAINT l_area_mood_pkey PRIMARY KEY (id);
ALTER TABLE l_area_place ADD CONSTRAINT l_area_place_pkey PRIMARY KEY (id);
ALTER TABLE l_area_recording ADD CONSTRAINT l_area_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_area_release ADD CONSTRAINT l_area_release_pkey PRIMARY KEY (id);
ALTER TABLE l_area_release_group ADD CONSTRAINT l_area_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_area_series ADD CONSTRAINT l_area_series_pkey PRIMARY KEY (id);
ALTER TABLE l_area_url ADD CONSTRAINT l_area_url_pkey PRIMARY KEY (id);
ALTER TABLE l_area_work ADD CONSTRAINT l_area_work_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_artist ADD CONSTRAINT l_artist_artist_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_event ADD CONSTRAINT l_artist_event_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_genre ADD CONSTRAINT l_artist_genre_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_instrument ADD CONSTRAINT l_artist_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_label ADD CONSTRAINT l_artist_label_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_mood ADD CONSTRAINT l_artist_mood_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_place ADD CONSTRAINT l_artist_place_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_recording ADD CONSTRAINT l_artist_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_release ADD CONSTRAINT l_artist_release_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_release_group ADD CONSTRAINT l_artist_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_series ADD CONSTRAINT l_artist_series_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_url ADD CONSTRAINT l_artist_url_pkey PRIMARY KEY (id);
ALTER TABLE l_artist_work ADD CONSTRAINT l_artist_work_pkey PRIMARY KEY (id);
ALTER TABLE l_event_event ADD CONSTRAINT l_event_event_pkey PRIMARY KEY (id);
ALTER TABLE l_event_genre ADD CONSTRAINT l_event_genre_pkey PRIMARY KEY (id);
ALTER TABLE l_event_instrument ADD CONSTRAINT l_event_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_event_label ADD CONSTRAINT l_event_label_pkey PRIMARY KEY (id);
ALTER TABLE l_event_mood ADD CONSTRAINT l_event_mood_pkey PRIMARY KEY (id);
ALTER TABLE l_event_place ADD CONSTRAINT l_event_place_pkey PRIMARY KEY (id);
ALTER TABLE l_event_recording ADD CONSTRAINT l_event_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_event_release ADD CONSTRAINT l_event_release_pkey PRIMARY KEY (id);
ALTER TABLE l_event_release_group ADD CONSTRAINT l_event_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_event_series ADD CONSTRAINT l_event_series_pkey PRIMARY KEY (id);
ALTER TABLE l_event_url ADD CONSTRAINT l_event_url_pkey PRIMARY KEY (id);
ALTER TABLE l_event_work ADD CONSTRAINT l_event_work_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_genre ADD CONSTRAINT l_genre_genre_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_instrument ADD CONSTRAINT l_genre_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_label ADD CONSTRAINT l_genre_label_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_mood ADD CONSTRAINT l_genre_mood_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_place ADD CONSTRAINT l_genre_place_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_recording ADD CONSTRAINT l_genre_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_release ADD CONSTRAINT l_genre_release_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_release_group ADD CONSTRAINT l_genre_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_series ADD CONSTRAINT l_genre_series_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_url ADD CONSTRAINT l_genre_url_pkey PRIMARY KEY (id);
ALTER TABLE l_genre_work ADD CONSTRAINT l_genre_work_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_instrument ADD CONSTRAINT l_instrument_instrument_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_label ADD CONSTRAINT l_instrument_label_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_mood ADD CONSTRAINT l_instrument_mood_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_place ADD CONSTRAINT l_instrument_place_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_recording ADD CONSTRAINT l_instrument_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_release ADD CONSTRAINT l_instrument_release_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_release_group ADD CONSTRAINT l_instrument_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_series ADD CONSTRAINT l_instrument_series_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_url ADD CONSTRAINT l_instrument_url_pkey PRIMARY KEY (id);
ALTER TABLE l_instrument_work ADD CONSTRAINT l_instrument_work_pkey PRIMARY KEY (id);
ALTER TABLE l_label_label ADD CONSTRAINT l_label_label_pkey PRIMARY KEY (id);
ALTER TABLE l_label_mood ADD CONSTRAINT l_label_mood_pkey PRIMARY KEY (id);
ALTER TABLE l_label_place ADD CONSTRAINT l_label_place_pkey PRIMARY KEY (id);
ALTER TABLE l_label_recording ADD CONSTRAINT l_label_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_label_release ADD CONSTRAINT l_label_release_pkey PRIMARY KEY (id);
ALTER TABLE l_label_release_group ADD CONSTRAINT l_label_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_label_series ADD CONSTRAINT l_label_series_pkey PRIMARY KEY (id);
ALTER TABLE l_label_url ADD CONSTRAINT l_label_url_pkey PRIMARY KEY (id);
ALTER TABLE l_label_work ADD CONSTRAINT l_label_work_pkey PRIMARY KEY (id);
ALTER TABLE l_mood_mood ADD CONSTRAINT l_mood_mood_pkey PRIMARY KEY (id);
ALTER TABLE l_mood_place ADD CONSTRAINT l_mood_place_pkey PRIMARY KEY (id);
ALTER TABLE l_mood_recording ADD CONSTRAINT l_mood_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_mood_release ADD CONSTRAINT l_mood_release_pkey PRIMARY KEY (id);
ALTER TABLE l_mood_release_group ADD CONSTRAINT l_mood_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_mood_series ADD CONSTRAINT l_mood_series_pkey PRIMARY KEY (id);
ALTER TABLE l_mood_url ADD CONSTRAINT l_mood_url_pkey PRIMARY KEY (id);
ALTER TABLE l_mood_work ADD CONSTRAINT l_mood_work_pkey PRIMARY KEY (id);
ALTER TABLE l_place_place ADD CONSTRAINT l_place_place_pkey PRIMARY KEY (id);
ALTER TABLE l_place_recording ADD CONSTRAINT l_place_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_place_release ADD CONSTRAINT l_place_release_pkey PRIMARY KEY (id);
ALTER TABLE l_place_release_group ADD CONSTRAINT l_place_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_place_series ADD CONSTRAINT l_place_series_pkey PRIMARY KEY (id);
ALTER TABLE l_place_url ADD CONSTRAINT l_place_url_pkey PRIMARY KEY (id);
ALTER TABLE l_place_work ADD CONSTRAINT l_place_work_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_recording ADD CONSTRAINT l_recording_recording_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_release ADD CONSTRAINT l_recording_release_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_release_group ADD CONSTRAINT l_recording_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_series ADD CONSTRAINT l_recording_series_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_url ADD CONSTRAINT l_recording_url_pkey PRIMARY KEY (id);
ALTER TABLE l_recording_work ADD CONSTRAINT l_recording_work_pkey PRIMARY KEY (id);
ALTER TABLE l_release_group_release_group ADD CONSTRAINT l_release_group_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_release_group_series ADD CONSTRAINT l_release_group_series_pkey PRIMARY KEY (id);
ALTER TABLE l_release_group_url ADD CONSTRAINT l_release_group_url_pkey PRIMARY KEY (id);
ALTER TABLE l_release_group_work ADD CONSTRAINT l_release_group_work_pkey PRIMARY KEY (id);
ALTER TABLE l_release_release ADD CONSTRAINT l_release_release_pkey PRIMARY KEY (id);
ALTER TABLE l_release_release_group ADD CONSTRAINT l_release_release_group_pkey PRIMARY KEY (id);
ALTER TABLE l_release_series ADD CONSTRAINT l_release_series_pkey PRIMARY KEY (id);
ALTER TABLE l_release_url ADD CONSTRAINT l_release_url_pkey PRIMARY KEY (id);
ALTER TABLE l_release_work ADD CONSTRAINT l_release_work_pkey PRIMARY KEY (id);
ALTER TABLE l_series_series ADD CONSTRAINT l_series_series_pkey PRIMARY KEY (id);
ALTER TABLE l_series_url ADD CONSTRAINT l_series_url_pkey PRIMARY KEY (id);
ALTER TABLE l_series_work ADD CONSTRAINT l_series_work_pkey PRIMARY KEY (id);
ALTER TABLE l_url_url ADD CONSTRAINT l_url_url_pkey PRIMARY KEY (id);
ALTER TABLE l_url_work ADD CONSTRAINT l_url_work_pkey PRIMARY KEY (id);
ALTER TABLE l_work_work ADD CONSTRAINT l_work_work_pkey PRIMARY KEY (id);
ALTER TABLE label ADD CONSTRAINT label_pkey PRIMARY KEY (id);
ALTER TABLE label_alias ADD CONSTRAINT label_alias_pkey PRIMARY KEY (id);
ALTER TABLE label_alias_type ADD CONSTRAINT label_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE label_annotation ADD CONSTRAINT label_annotation_pkey PRIMARY KEY (label, annotation);
ALTER TABLE label_attribute ADD CONSTRAINT label_attribute_pkey PRIMARY KEY (id);
ALTER TABLE label_attribute_type ADD CONSTRAINT label_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE label_attribute_type_allowed_value ADD CONSTRAINT label_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE label_gid_redirect ADD CONSTRAINT label_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE label_ipi ADD CONSTRAINT label_ipi_pkey PRIMARY KEY (label, ipi);
ALTER TABLE label_isni ADD CONSTRAINT label_isni_pkey PRIMARY KEY (label, isni);
ALTER TABLE label_meta ADD CONSTRAINT label_meta_pkey PRIMARY KEY (id);
ALTER TABLE label_rating_raw ADD CONSTRAINT label_rating_raw_pkey PRIMARY KEY (label, editor);
ALTER TABLE label_tag ADD CONSTRAINT label_tag_pkey PRIMARY KEY (label, tag);
ALTER TABLE label_tag_raw ADD CONSTRAINT label_tag_raw_pkey PRIMARY KEY (label, editor, tag);
ALTER TABLE label_type ADD CONSTRAINT label_type_pkey PRIMARY KEY (id);
ALTER TABLE language ADD CONSTRAINT language_pkey PRIMARY KEY (id);
ALTER TABLE link ADD CONSTRAINT link_pkey PRIMARY KEY (id);
ALTER TABLE link_attribute ADD CONSTRAINT link_attribute_pkey PRIMARY KEY (link, attribute_type);
ALTER TABLE link_attribute_credit ADD CONSTRAINT link_attribute_credit_pkey PRIMARY KEY (link, attribute_type);
ALTER TABLE link_attribute_text_value ADD CONSTRAINT link_attribute_text_value_pkey PRIMARY KEY (link, attribute_type);
ALTER TABLE link_attribute_type ADD CONSTRAINT link_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE link_creditable_attribute_type ADD CONSTRAINT link_creditable_attribute_type_pkey PRIMARY KEY (attribute_type);
ALTER TABLE link_text_attribute_type ADD CONSTRAINT link_text_attribute_type_pkey PRIMARY KEY (attribute_type);
ALTER TABLE link_type ADD CONSTRAINT link_type_pkey PRIMARY KEY (id);
ALTER TABLE link_type_attribute_type ADD CONSTRAINT link_type_attribute_type_pkey PRIMARY KEY (link_type, attribute_type);
ALTER TABLE medium ADD CONSTRAINT medium_pkey PRIMARY KEY (id);
ALTER TABLE medium_attribute ADD CONSTRAINT medium_attribute_pkey PRIMARY KEY (id);
ALTER TABLE medium_attribute_type ADD CONSTRAINT medium_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE medium_attribute_type_allowed_format ADD CONSTRAINT medium_attribute_type_allowed_format_pkey PRIMARY KEY (medium_format, medium_attribute_type);
ALTER TABLE medium_attribute_type_allowed_value ADD CONSTRAINT medium_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE medium_attribute_type_allowed_value_allowed_format ADD CONSTRAINT medium_attribute_type_allowed_value_allowed_format_pkey PRIMARY KEY (medium_format, medium_attribute_type_allowed_value);
ALTER TABLE medium_cdtoc ADD CONSTRAINT medium_cdtoc_pkey PRIMARY KEY (id);
ALTER TABLE medium_format ADD CONSTRAINT medium_format_pkey PRIMARY KEY (id);
ALTER TABLE medium_index ADD CONSTRAINT medium_index_pkey PRIMARY KEY (medium);
ALTER TABLE mood ADD CONSTRAINT mood_pkey PRIMARY KEY (id);
ALTER TABLE mood_alias ADD CONSTRAINT mood_alias_pkey PRIMARY KEY (id);
ALTER TABLE mood_alias_type ADD CONSTRAINT mood_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE mood_annotation ADD CONSTRAINT mood_annotation_pkey PRIMARY KEY (mood, annotation);
ALTER TABLE orderable_link_type ADD CONSTRAINT orderable_link_type_pkey PRIMARY KEY (link_type);
ALTER TABLE place ADD CONSTRAINT place_pkey PRIMARY KEY (id);
ALTER TABLE place_alias ADD CONSTRAINT place_alias_pkey PRIMARY KEY (id);
ALTER TABLE place_alias_type ADD CONSTRAINT place_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE place_annotation ADD CONSTRAINT place_annotation_pkey PRIMARY KEY (place, annotation);
ALTER TABLE place_attribute ADD CONSTRAINT place_attribute_pkey PRIMARY KEY (id);
ALTER TABLE place_attribute_type ADD CONSTRAINT place_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE place_attribute_type_allowed_value ADD CONSTRAINT place_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE place_gid_redirect ADD CONSTRAINT place_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE place_meta ADD CONSTRAINT place_meta_pkey PRIMARY KEY (id);
ALTER TABLE place_rating_raw ADD CONSTRAINT place_rating_raw_pkey PRIMARY KEY (place, editor);
ALTER TABLE place_tag ADD CONSTRAINT place_tag_pkey PRIMARY KEY (place, tag);
ALTER TABLE place_tag_raw ADD CONSTRAINT place_tag_raw_pkey PRIMARY KEY (place, editor, tag);
ALTER TABLE place_type ADD CONSTRAINT place_type_pkey PRIMARY KEY (id);
ALTER TABLE recording ADD CONSTRAINT recording_pkey PRIMARY KEY (id);
ALTER TABLE recording_alias ADD CONSTRAINT recording_alias_pkey PRIMARY KEY (id);
ALTER TABLE recording_alias_type ADD CONSTRAINT recording_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE recording_annotation ADD CONSTRAINT recording_annotation_pkey PRIMARY KEY (recording, annotation);
ALTER TABLE recording_attribute ADD CONSTRAINT recording_attribute_pkey PRIMARY KEY (id);
ALTER TABLE recording_attribute_type ADD CONSTRAINT recording_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE recording_attribute_type_allowed_value ADD CONSTRAINT recording_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE recording_first_release_date ADD CONSTRAINT recording_first_release_date_pkey PRIMARY KEY (recording);
ALTER TABLE recording_gid_redirect ADD CONSTRAINT recording_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE recording_meta ADD CONSTRAINT recording_meta_pkey PRIMARY KEY (id);
ALTER TABLE recording_rating_raw ADD CONSTRAINT recording_rating_raw_pkey PRIMARY KEY (recording, editor);
ALTER TABLE recording_tag ADD CONSTRAINT recording_tag_pkey PRIMARY KEY (recording, tag);
ALTER TABLE recording_tag_raw ADD CONSTRAINT recording_tag_raw_pkey PRIMARY KEY (recording, editor, tag);
ALTER TABLE release ADD CONSTRAINT release_pkey PRIMARY KEY (id);
ALTER TABLE release_alias ADD CONSTRAINT release_alias_pkey PRIMARY KEY (id);
ALTER TABLE release_alias_type ADD CONSTRAINT release_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE release_annotation ADD CONSTRAINT release_annotation_pkey PRIMARY KEY (release, annotation);
ALTER TABLE release_attribute ADD CONSTRAINT release_attribute_pkey PRIMARY KEY (id);
ALTER TABLE release_attribute_type ADD CONSTRAINT release_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE release_attribute_type_allowed_value ADD CONSTRAINT release_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE release_country ADD CONSTRAINT release_country_pkey PRIMARY KEY (release, country);
ALTER TABLE release_first_release_date ADD CONSTRAINT release_first_release_date_pkey PRIMARY KEY (release);
ALTER TABLE release_gid_redirect ADD CONSTRAINT release_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE release_group ADD CONSTRAINT release_group_pkey PRIMARY KEY (id);
ALTER TABLE release_group_alias ADD CONSTRAINT release_group_alias_pkey PRIMARY KEY (id);
ALTER TABLE release_group_alias_type ADD CONSTRAINT release_group_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE release_group_annotation ADD CONSTRAINT release_group_annotation_pkey PRIMARY KEY (release_group, annotation);
ALTER TABLE release_group_attribute ADD CONSTRAINT release_group_attribute_pkey PRIMARY KEY (id);
ALTER TABLE release_group_attribute_type ADD CONSTRAINT release_group_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE release_group_attribute_type_allowed_value ADD CONSTRAINT release_group_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE release_group_gid_redirect ADD CONSTRAINT release_group_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE release_group_meta ADD CONSTRAINT release_group_meta_pkey PRIMARY KEY (id);
ALTER TABLE release_group_primary_type ADD CONSTRAINT release_group_primary_type_pkey PRIMARY KEY (id);
ALTER TABLE release_group_rating_raw ADD CONSTRAINT release_group_rating_raw_pkey PRIMARY KEY (release_group, editor);
ALTER TABLE release_group_secondary_type ADD CONSTRAINT release_group_secondary_type_pkey PRIMARY KEY (id);
ALTER TABLE release_group_secondary_type_join ADD CONSTRAINT release_group_secondary_type_join_pkey PRIMARY KEY (release_group, secondary_type);
ALTER TABLE release_group_tag ADD CONSTRAINT release_group_tag_pkey PRIMARY KEY (release_group, tag);
ALTER TABLE release_group_tag_raw ADD CONSTRAINT release_group_tag_raw_pkey PRIMARY KEY (release_group, editor, tag);
ALTER TABLE release_label ADD CONSTRAINT release_label_pkey PRIMARY KEY (id);
ALTER TABLE release_meta ADD CONSTRAINT release_meta_pkey PRIMARY KEY (id);
ALTER TABLE release_packaging ADD CONSTRAINT release_packaging_pkey PRIMARY KEY (id);
ALTER TABLE release_raw ADD CONSTRAINT release_raw_pkey PRIMARY KEY (id);
ALTER TABLE release_status ADD CONSTRAINT release_status_pkey PRIMARY KEY (id);
ALTER TABLE release_tag ADD CONSTRAINT release_tag_pkey PRIMARY KEY (release, tag);
ALTER TABLE release_tag_raw ADD CONSTRAINT release_tag_raw_pkey PRIMARY KEY (release, editor, tag);
ALTER TABLE release_unknown_country ADD CONSTRAINT release_unknown_country_pkey PRIMARY KEY (release);
ALTER TABLE replication_control ADD CONSTRAINT replication_control_pkey PRIMARY KEY (id);
ALTER TABLE script ADD CONSTRAINT script_pkey PRIMARY KEY (id);
ALTER TABLE series ADD CONSTRAINT series_pkey PRIMARY KEY (id);
ALTER TABLE series_alias ADD CONSTRAINT series_alias_pkey PRIMARY KEY (id);
ALTER TABLE series_alias_type ADD CONSTRAINT series_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE series_annotation ADD CONSTRAINT series_annotation_pkey PRIMARY KEY (series, annotation);
ALTER TABLE series_attribute ADD CONSTRAINT series_attribute_pkey PRIMARY KEY (id);
ALTER TABLE series_attribute_type ADD CONSTRAINT series_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE series_attribute_type_allowed_value ADD CONSTRAINT series_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE series_gid_redirect ADD CONSTRAINT series_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE series_ordering_type ADD CONSTRAINT series_ordering_type_pkey PRIMARY KEY (id);
ALTER TABLE series_tag ADD CONSTRAINT series_tag_pkey PRIMARY KEY (series, tag);
ALTER TABLE series_tag_raw ADD CONSTRAINT series_tag_raw_pkey PRIMARY KEY (series, editor, tag);
ALTER TABLE series_type ADD CONSTRAINT series_type_pkey PRIMARY KEY (id);
ALTER TABLE tag ADD CONSTRAINT tag_pkey PRIMARY KEY (id);
ALTER TABLE tag_relation ADD CONSTRAINT tag_relation_pkey PRIMARY KEY (tag1, tag2);
ALTER TABLE track ADD CONSTRAINT track_pkey PRIMARY KEY (id);
ALTER TABLE track_gid_redirect ADD CONSTRAINT track_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE track_raw ADD CONSTRAINT track_raw_pkey PRIMARY KEY (id);
ALTER TABLE unreferenced_row_log ADD CONSTRAINT unreferenced_row_log_pkey PRIMARY KEY (table_name, row_id);
ALTER TABLE url ADD CONSTRAINT url_pkey PRIMARY KEY (id);
ALTER TABLE url_gid_redirect ADD CONSTRAINT url_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE vote ADD CONSTRAINT vote_pkey PRIMARY KEY (id);
ALTER TABLE work ADD CONSTRAINT work_pkey PRIMARY KEY (id);
ALTER TABLE work_alias ADD CONSTRAINT work_alias_pkey PRIMARY KEY (id);
ALTER TABLE work_alias_type ADD CONSTRAINT work_alias_type_pkey PRIMARY KEY (id);
ALTER TABLE work_annotation ADD CONSTRAINT work_annotation_pkey PRIMARY KEY (work, annotation);
ALTER TABLE work_attribute ADD CONSTRAINT work_attribute_pkey PRIMARY KEY (id);
ALTER TABLE work_attribute_type ADD CONSTRAINT work_attribute_type_pkey PRIMARY KEY (id);
ALTER TABLE work_attribute_type_allowed_value ADD CONSTRAINT work_attribute_type_allowed_value_pkey PRIMARY KEY (id);
ALTER TABLE work_gid_redirect ADD CONSTRAINT work_gid_redirect_pkey PRIMARY KEY (gid);
ALTER TABLE work_language ADD CONSTRAINT work_language_pkey PRIMARY KEY (work, language);
ALTER TABLE work_meta ADD CONSTRAINT work_meta_pkey PRIMARY KEY (id);
ALTER TABLE work_rating_raw ADD CONSTRAINT work_rating_raw_pkey PRIMARY KEY (work, editor);
ALTER TABLE work_tag ADD CONSTRAINT work_tag_pkey PRIMARY KEY (work, tag);
ALTER TABLE work_tag_raw ADD CONSTRAINT work_tag_raw_pkey PRIMARY KEY (work, editor, tag);
ALTER TABLE work_type ADD CONSTRAINT work_type_pkey PRIMARY KEY (id);
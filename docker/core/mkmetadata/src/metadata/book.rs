/*

async def metadata_periodicals_lookup(db_connection, download_data):
    """
    Lookup via isbn and then name
    """
    metadata_uuid = None  # so not found checks verify later
    # check if isbn in metaid
    if download_data['ProviderMetaID'] is not None:
        # check local database
        metadata_uuid = db_connection.db_meta_book_guid_by_isbn(
            download_data['ProviderMetaID'], download_data['ProviderMetaID'])
    else:
        # try to pull isbn out..might not be long enough, so try
        try:
            metadata_uuid = db_connection.db_meta_book_guid_by_isbn(download_data['Path'][-10:],
                                                                    download_data['Path'][-13:])
        except:
            pass
    if metadata_uuid is None:
        if download_data['ProviderMetaID'] is None:
            lookup_name = os.path.basename(
                os.path.splitext(download_data['Path'])[0]).replace('_', ' ')
            metadata_uuid = db_connection.db_meta_book_guid_by_name(lookup_name)
        if metadata_uuid is None:
            # save the updated status
            await db_connection.db_begin()
            await db_connection.db_download_update(download_que_id, 'Search')
            # set provider last so it's not picked up by the wrong thread
            await db_connection.db_download_update_provider(
                'isbndb', download_que_id)
            await db_connection.db_commit()
    else:
        # meta uuid is found so delete
        await db_connection.db_download_delete(download_que_id)
        await db_connection.db_commit()
    return metadata_uuid


async def metadata_periodicals_cover(db_connection, isbn):
    """
    pull and save the cover image for periodical
    """
    # TODO use the cover pull in common_metadata_open_library
    url = ('http://covers.openlibrary.org/b/ISBN/%s-L.jpg?default=false', isbn)

    common_metadata.com_meta_image_path(download_data['Name'],
                                        'poster', 'themoviedb', download_data['Poster'])

    return False

 */

// cargo test -- --show-output
#[cfg(test)]
mod test_mk_lib_common {
    use super::*;

    macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }
}
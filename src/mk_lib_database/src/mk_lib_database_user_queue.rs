

/*
async def db_meta_queue_list_count(self, user_id, search_value=None, db_connection=None):
    """
    Return count of queued media for user
    """
    if search_value is not None:
        return await db_conn.fetchval('select count(*)'
                                      ' from mm_user_queue'
                                      ' where mm_user_queue_name % $1'
                                      ' and mm_user_queue_user_id = $2',
                                      search_value, user_id)
    else:
        return await db_conn.fetchval('select count(*) from mm_user_queue'
                                      ' where mm_user_queue_user_id = $1', user_id)

 */
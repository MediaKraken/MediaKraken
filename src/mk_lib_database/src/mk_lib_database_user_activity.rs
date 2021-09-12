

/*
async def db_activity_insert(self, activity_name, activity_overview,
                             activity_short_overview, activity_type, activity_itemid,
                             activity_userid,
                             activity_log_severity, db_connection=None):
    """
    Insert server or user activity record
    """
    new_guid = uuid.uuid4()
    await db_conn.execute('insert into mm_user_activity (mm_activity_guid,'
                          ' mm_activity_name,'
                          ' mm_activity_overview,'
                          ' mm_activity_short_overview,'
                          ' mm_activity_type,'
                          ' mm_activity_itemid,'
                          ' mm_activity_userid, '
                          ' mm_activity_datecreated,'
                          ' mm_activity_log_severity)'
                          ' values ($1,$2,$3,$4,$5,$6,$7,$8,$9)',
                          (new_guid, activity_name, activity_overview,
                           activity_short_overview,
                           activity_type, activity_itemid, activity_userid,
                           datetime.datetime.now(), activity_log_severity))
    await db_conn.execute('commit')
    return new_guid


async def db_activity_purge(self, days_old, db_connection=None):
    """
    Purge records older than specified days
    """
    # TODO broken.....passing %s then + the field
    await db_conn.execute('delete from mm_user_activity'
                          ' where mm_activity_datecreated'
                          ' < now() - interval $1;', str(days_old) + ' day')
    await db_conn.execute('commit')

 */
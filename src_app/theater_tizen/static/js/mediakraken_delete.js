// Generic delete-confirmation + AJAX helper.
//
// Each delete button carries data-id and data-delete="link" (etc). A single
// modal per kind lives in the page (#delete_link, #delete_sync, ...) and is
// shown here rather than by six near-identical functions.

var DELETE_KINDS = {
    link:         { modal: '#delete_link',         url: '../link_delete',         redirect: '../link_server'   },
    sync:         { modal: '#delete_sync',         url: '../sync_delete',         redirect: '../sync'          },
    library:      { modal: '#delete_library',      url: '../library_delete',      redirect: '../library'       },
    user:         { modal: '#delete_user',         url: '../user_delete',         redirect: '../users'         },
    backup:       { modal: '#delete_backup',       url: '../backup_delete',       redirect: '../backup'        },
    transmission: { modal: '#delete_transmission', url: '../transmission_delete', redirect: '../transmission'  }
};

function confirmDelete(elem, kind) {
    var config = DELETE_KINDS[kind];
    if (!config) {
        return;
    }
    var id = $(elem).attr('data-id');
    $(config.modal).data('deleteId', id).modal();
}

function runDelete(kind) {
    var config = DELETE_KINDS[kind];
    if (!config) {
        return;
    }
    var id = $(config.modal).data('deleteId');
    $.ajax({
        url: config.url,
        data: { id: id },
        type: 'POST',
        success: function (res) {
            var result;
            try {
                result = JSON.parse(res);
            } catch (e) {
                console.error(config.url + ': invalid JSON', e, res);
                return;
            }
            if (result.status === 'OK') {
                $(config.modal).modal('hide');
                window.location = config.redirect;
            } else {
                alert(result.status);
            }
        },
        error: function (error) {
            console.error(config.url + ' failed', error);
        }
    });
}

// Backwards-compatible shims for inline onclick handlers already in templates.
function ConfirmLinkDelete(elem)         { confirmDelete(elem, 'link'); }
function Link_Delete()                   { runDelete('link'); }
function ConfirmSyncDelete(elem)         { confirmDelete(elem, 'sync'); }
function Sync_Delete()                   { runDelete('sync'); }
function ConfirmLibraryDelete(elem)      { confirmDelete(elem, 'library'); }
function Library_Delete()                { runDelete('library'); }
function ConfirmUserDelete(elem)         { confirmDelete(elem, 'user'); }
function User_Delete()                   { runDelete('user'); }
function ConfirmBackupDelete(elem)       { confirmDelete(elem, 'backup'); }
function Backup_Delete()                 { runDelete('backup'); }
function ConfirmTransmissionDelete(elem) { confirmDelete(elem, 'transmission'); }
function Transmission_Delete()           { runDelete('transmission'); }

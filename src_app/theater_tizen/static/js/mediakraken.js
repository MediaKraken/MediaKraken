$(function () {
    function setMediaStatus(mediaGuid, mediaType, mediaStatus) {
        $.ajax({
            url: '../media_status/' + encodeURIComponent(mediaGuid)
                + '/' + encodeURIComponent(mediaType)
                + '/' + encodeURIComponent(mediaStatus),
            data: { id: mediaGuid },
            type: 'POST',
            success: function (res) {
                var result;
                try {
                    result = JSON.parse(res);
                } catch (e) {
                    console.error('media_status: invalid JSON', e, res);
                    return;
                }
                if (result.status === 'OK') {
                    window.location = '../media_status/'
                        + encodeURIComponent(mediaGuid)
                        + '/' + encodeURIComponent(mediaType)
                        + '/' + encodeURIComponent(mediaStatus);
                } else {
                    alert(result.status);
                }
            },
            error: function (error) {
                console.error('media_status request failed', error);
            }
        });
    }

    $(document).on('click', '.media_context_right [data-action]', function (e) {
        e.preventDefault();
        var $trigger = $(this).closest('.media_context_right');
        var guid = $trigger.data('id');
        var status = $(this).data('action');
        if (!guid || !status) {
            return;
        }
        setMediaStatus(guid, 'movie', status);
    });
});

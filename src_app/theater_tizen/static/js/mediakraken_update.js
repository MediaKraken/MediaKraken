function EditLibrary(elm) {
    var id = $(elm).attr('data-id');
    $('#editLibraryModal').data('editLibraryId', id);
    $.ajax({
        url: '../getLibraryById',
        data: { id: id },
        type: 'POST',
        success: function (res) {
            var data;
            try {
                data = JSON.parse(res);
            } catch (e) {
                console.error('getLibraryById: invalid JSON', e, res);
                return;
            }
            $('#editPath').val(data['Path']);
            $('#editClass').val(data['Media Class']);
            $('#editLibraryModal').modal();
        },
        error: function (error) {
            console.error('getLibraryById failed', error);
        }
    });
}

$(function () {
    $('#btnLibraryUpdate').click(function () {
        $.ajax({
            url: '../updateLibrary',
            data: {
                new_path: $('#editPath').val(),
                new_class: $('#editClass').val(),
                id: $('#editLibraryModal').data('editLibraryId')
            },
            type: 'POST',
            success: function () {
                $('#editLibraryModal').modal('hide');
            },
            error: function (error) {
                console.error('updateLibrary failed', error);
            }
        });
    });
});

function EditTransmission(elm) {
    var id = $(elm).attr('data-id');
    $('#editTransmissionModal').data('editId', id);
    $.ajax({
        url: '../transmission_edit',
        data: { id: id },
        type: 'POST',
        success: function (res) {
            var data;
            try {
                data = JSON.parse(res);
            } catch (e) {
                console.error('transmission_edit: invalid JSON', e, res);
                return;
            }
            $('#editTitle').val(data['Title']);
            $('#editDescription').val(data['Description']);
            $('#editTransmissionModal').modal();
        },
        error: function (error) {
            console.error('transmission_edit failed', error);
        }
    });
}

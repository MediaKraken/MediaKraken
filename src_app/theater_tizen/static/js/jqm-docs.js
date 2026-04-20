// Collapse page navs after a tap inside their contents.
$(function () {
    $(document).on('click', '.content-secondary .ui-collapsible-content', function () {
        $(this).trigger('collapse');
    });
});

function setDefaultTransition() {
    var winwidth = $(window).width();
    var trans = 'slide';
    if (winwidth >= 1000) {
        trans = 'none';
    } else if (winwidth >= 650) {
        trans = 'fade';
    }
    $.mobile.defaultPageTransition = trans;
}

$(function () {
    setDefaultTransition();
    $(window).on('throttledresize', setDefaultTransition);
});

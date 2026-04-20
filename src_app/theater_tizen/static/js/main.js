// Tizen back-button / exit handling.
var backEventListener = null;

function unregister() {
    if (backEventListener !== null) {
        document.removeEventListener('tizenhwkey', backEventListener);
        backEventListener = null;
        if (window.tizen && window.tizen.application) {
            window.tizen.application.getCurrentApplication().exit();
        }
    }
}

function init() {
    if (backEventListener !== null) {
        return;
    }

    var backEvent = function (e) {
        if (e.keyName !== 'back') {
            return;
        }
        try {
            if (!$.mobile || !$.mobile.urlHistory
                    || $.mobile.urlHistory.activeIndex <= 0) {
                unregister();
            } else {
                window.history.back();
            }
        } catch (ex) {
            unregister();
        }
    };

    document.addEventListener('tizenhwkey', backEvent);
    backEventListener = backEvent;
}

$(document).on('pageinit', init);
$(window).on('beforeunload', unregister);

// Remote-key focus navigation for the tile grid.
var focusIndex = 0;

function focusableItems() {
    return $.mobile.activePage.find('a[href], a[data-section]');
}

function setFocus(index) {
    var list = focusableItems();
    list.removeClass('mk-focused');
    if (index < 0 || index >= list.length) {
        return;
    }
    var item = list[index];
    $(item).addClass('mk-focused');
    if (typeof item.focus === 'function') {
        item.focus();
    }
}

function activate(index) {
    var list = focusableItems();
    var item = list[index];
    if (!item) {
        return;
    }
    var section = item.getAttribute('data-section');
    if (section) {
        alert(section + ' section is not yet implemented');
        return;
    }
    var path = item.getAttribute('href');
    if (path && path !== '#') {
        $.mobile.changePage(path);
    }
}

function handlePageKey(e) {
    var list = focusableItems();
    if (list.length === 0) {
        return;
    }
    switch (e.keyCode) {
        case TvKeyCode.KEY_LEFT:
        case TvKeyCode.KEY_UP:
            if (focusIndex > 0) {
                focusIndex -= 1;
                setFocus(focusIndex);
            }
            break;
        case TvKeyCode.KEY_RIGHT:
        case TvKeyCode.KEY_DOWN:
            if (focusIndex < list.length - 1) {
                focusIndex += 1;
                setFocus(focusIndex);
            }
            break;
        case TvKeyCode.KEY_ENTER:
            activate(focusIndex);
            break;
        default:
            break;
    }
}

function bindKeyNavigation() {
    focusIndex = 0;
    setFocus(focusIndex);
    document.body.removeEventListener('keydown', handlePageKey, false);
    document.body.addEventListener('keydown', handlePageKey, false);
}

$(document).on('pageshow', '[data-role="page"]', bindKeyNavigation);

// Click fallback for tiles that have no destination yet.
$(document).on('click', '.mk-tile[data-section]', function (e) {
    e.preventDefault();
    var section = $(this).data('section');
    alert(section + ' section is not yet implemented');
});

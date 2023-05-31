$(function(){
    $('#the-movie-node').contextMenu({
        selector: 'div',
        callback: function(key, options) {
            var m = "clicked: " + key + " on " + $(this).attr('data-id');
        $.ajax({
                url: '/user/user_status_movie/' + $(this).attr('data-id') + '/' + key,
                type: 'POST',
                success: function(res) {
                    var result = JSON.parse(res);
                    if (result.status == 'OK') {
                                window.location = window.location.href
                    } else {
                        alert(result.status);
                    }
                },
                error: function(error) {
                    console.log(error);
                }
            });
        },
        items: {
            "watched": {name: "Set Watched", icon: "/static/image/eye.png"},
            "sync": {name: "Sync Media", icon: "/static/image/synced.jpg"},
            "towatch": {name: "Add to Watch Queue", icon: "/static/image/rectangles.png"},
            "sep1": "---------",
            "favorite": {name: "Set Favorite", icon: "/static/image/favorite-mark.png"},
            "like": {name: "Set Upvote", icon: "/static/image/thumbs-up.png"},
            "dislike": {name: "Set Downvote", icon: "/static/image/dislike-thumb.png"},
            "poo": {name: "Set Avoid", icon: "/static/image/pile-of-dung.png"},
            "sep2": "---------",
            "mismatch": {name: "Set Mismatch", icon: "/static/image/exclamation-circle-frame.png"},
        }
    });
});


$(function(){
    $('#the-movie-metadata-node').contextMenu({
        selector: 'div',
        callback: function(key, options) {
            var m = "clicked: " + key + " on " + $(this).attr('data-id');
        $.ajax({
                url: '/user/user_status_movie_metadata/' + $(this).attr('data-id') + '/' + key,
                type: 'POST',
                success: function(res) {
                    var result = JSON.parse(res);
                    if (result.status == 'OK') {
                                window.location = window.location.href
                    } else {
                        alert(result.status);
                    }
                },
                error: function(error) {
                    console.log(error);
                }
            });
        },
        items: {
            "watched": {name: "Set Watched", icon: "/static/image/eye.png"},
            "towatch": {name: "Add to Watch Queue", icon: "/static/image/rectangles.png"},
            "sep1": "---------",
            "favorite": {name: "Set Favorite", icon: "/static/image/favorite-mark.png"},
            "like": {name: "Set Upvote", icon: "/static/image/thumbs-up.png"},
            "dislike": {name: "Set Downvote", icon: "/static/image/dislike-thumb.png"},
            "poo": {name: "Set Avoid", icon: "/static/image/pile-of-dung.png"},
            "sep1": "---------",
            "request": {name: "Request Media", icon: "/static/image/add-button.png"},
        }
    });
});


$(function(){
    $('#the-tv-node').contextMenu({
        selector: 'div',
        callback: function(key, options) {
            var m = "clicked: " + key + " on " + $(this).attr('data-id');
        $.ajax({
                url: '/user/user_status_tv/' + $(this).attr('data-id') + '/' + key,
                type: 'POST',
                success: function(res) {
                    var result = JSON.parse(res);
                    if (result.status == 'OK') {
                                window.location = window.location.href
                    } else {
                        alert(result.status);
                    }
                },
                error: function(error) {
                    console.log(error);
                }
            });
            window.console && console.log(m) || alert(m);
        },
        items: {
            "watched": {name: "Set Episode Watched", icon: "/static/image/eye.png"},
            "watched_season": {name: "Set Season Watched", icon: "/static/image/eye.png"},
            "watched_show": {name: "Set Show Watched", icon: "/static/image/eye.png"},
            "sync": {name: "Sync Episode Media", icon: "/static/image/synced.jpg"},
            "sync_season": {name: "Sync Season Media", icon: "/static/image/synced.jpg"},
            "sync_show": {name: "Sync Show Media", icon: "/static/image/synced.jpg"},
            "towatch": {name: "Add to Watch Queue", icon: "/static/image/rectangles.png"},
            "towatch_season": {name: "Add Season to Watch Queue", icon: "/static/image/rectangles.png"},
            "towatch_show": {name: "Add Show to Watch Queue", icon: "/static/image/rectangles.png"},
            "sep1": "---------",
            "favorite": {name: "Set Episdoe Favorite", icon: "/static/image/favorite-mark.png"},
            "favorite_season": {name: "Set Season Favorite", icon: "/static/image/favorite-mark.png"},
            "favorite_show": {name: "Set Show Favorite", icon: "/static/image/favorite-mark.png"},
            "poo": {name: "Set Episode Downvote", icon: "/static/image/pile-of-dung.png"},
            "poo_season": {name: "Set Season Downvote", icon: "/static/image/pile-of-dung.png"},
            "poo_show": {name: "Set Show Downvote", icon: "/static/image/pile-of-dung.png"},
            "sep2": "---------",
            "mismatch": {name: "Set Episode Mismatch", icon: "/static/image/exclamation-circle-frame.png"},
            "mismatch_season": {name: "Set Season Mismatch", icon: "/static/image/exclamation-circle-frame.png"},
            "mismatch_show": {name: "Set Show Mismatch", icon: "/static/image/exclamation-circle-frame.png"},
        }
    });
});


$(function(){
    $('#the-tv-metadata-node').contextMenu({
        selector: 'div',
        callback: function(key, options) {
            var m = "clicked: " + key + " on " + $(this).attr('data-id');
        $.ajax({
                url: '/user/user_meta_tv_status/' + $(this).attr('data-id') + '/' + key,
                type: 'POST',
                success: function(res) {
                    var result = JSON.parse(res);
                    if (result.status == 'OK') {
                                window.location = window.location.href
                    } else {
                        alert(result.status);
                    }
                },
                error: function(error) {
                    console.log(error);
                }
            });
            window.console && console.log(m) || alert(m);
        },
        items: {
            "watched": {name: "Set Episode Watched", icon: "/static/image/eye.png"},
            "watched_season": {name: "Set Season Watched", icon: "/static/image/eye.png"},
            "watched_show": {name: "Set Show Watched", icon: "/static/image/eye.png"},
            "sync": {name: "Sync Episode Media", icon: "/static/image/synced.jpg"},
            "sync_season": {name: "Sync Season Media", icon: "/static/image/synced.jpg"},
            "sync_show": {name: "Sync Show Media", icon: "/static/image/synced.jpg"},
            "towatch": {name: "Add to Watch Queue", icon: "/static/image/rectangles.png"},
            "towatch_season": {name: "Add Season to Watch Queue", icon: "/static/image/rectangles.png"},
            "towatch_show": {name: "Add Show to Watch Queue", icon: "/static/image/rectangles.png"},
            "sep1": "---------",
            "favorite": {name: "Set Episdoe Favorite", icon: "/static/image/favorite-mark.png"},
            "favorite_season": {name: "Set Season Favorite", icon: "/static/image/favorite-mark.png"},
            "favorite_show": {name: "Set Show Favorite", icon: "/static/image/favorite-mark.png"},
            "poo": {name: "Set Episode Downvote", icon: "/static/image/pile-of-dung.png"},
            "poo_season": {name: "Set Season Downvote", icon: "/static/image/pile-of-dung.png"},
            "poo_show": {name: "Set Show Downvote", icon: "/static/image/pile-of-dung.png"},
            "sep2": "---------",
            "mismatch": {name: "Set Episode Mismatch", icon: "/static/image/exclamation-circle-frame.png"},
            "mismatch_season": {name: "Set Season Mismatch", icon: "/static/image/exclamation-circle-frame.png"},
            "mismatch_show": {name: "Set Show Mismatch", icon: "/static/image/exclamation-circle-frame.png"},
        }
    });
});

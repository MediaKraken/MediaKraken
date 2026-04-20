' HomeScene: top-level scene showing the MediaKraken category grid.
sub init()
    m.titleLabel = m.top.findNode("titleLabel")
    m.serverLabel = m.top.findNode("serverLabel")
    m.statusLabel = m.top.findNode("statusLabel")
    m.grid = m.top.findNode("categoryGrid")

    m.serverLabel.text = "Server: " + MK_GetServerUrl()
    m.statusLabel.text = "Press OK on a category to load titles. Press * for settings."

    m.grid.content = buildCategoryContent()
    m.grid.observeField("itemSelected", "onCategorySelected")
    m.grid.setFocus(true)

    m.top.observeField("deepLinkContentId", "onDeepLink")
end sub

function buildCategoryContent() as object
    categories = [
        { title: "Movies",        slug: "movie",       icon: "pkg:/images/cat-movie.png" },
        { title: "Music",         slug: "music",       icon: "pkg:/images/cat-music.png" },
        { title: "TV",            slug: "tv",          icon: "pkg:/images/cat-tv.png" },
        { title: "Sports",        slug: "sports",      icon: "pkg:/images/cat-sports.png" },
        { title: "Live TV",       slug: "livetv",      icon: "pkg:/images/cat-livetv.png" },
        { title: "Image Gallery", slug: "imagegallery",icon: "pkg:/images/cat-photo.png" },
        { title: "Games",         slug: "games",       icon: "pkg:/images/cat-games.png" },
        { title: "Books",         slug: "books",       icon: "pkg:/images/cat-books.png" },
        { title: "iRadio",        slug: "iradio",      icon: "pkg:/images/cat-radio.png" },
        { title: "Home Movies",   slug: "home_media",  icon: "pkg:/images/cat-homemovie.png" },
        { title: "3D",            slug: "3D",          icon: "pkg:/images/cat-3d.png" },
        { title: "Internet",      slug: "internet",    icon: "pkg:/images/cat-internet.png" }
    ]

    root = CreateObject("roSGNode", "ContentNode")
    for each entry in categories
        node = root.createChild("ContentNode")
        node.title = entry.title
        node.shortDescriptionLine1 = entry.slug
        node.HDPosterUrl = entry.icon
        node.SDPosterUrl = entry.icon
    end for
    return root
end function

sub onCategorySelected()
    index = m.grid.itemSelected
    item = m.grid.content.getChild(index)
    if item = invalid then return
    slug = item.shortDescriptionLine1
    m.statusLabel.text = "Loading " + item.title + "..."
    startCatalogFetch(slug)
end sub

sub startCatalogFetch(slug as string)
    if m.task <> invalid and m.task.state = "RUN" then
        m.task.control = "STOP"
    end if
    m.task = CreateObject("roSGNode", "CatalogTask")
    m.task.serverUrl = MK_GetServerUrl()
    m.task.category = slug
    m.task.observeField("result", "onCatalogResult")
    m.task.control = "RUN"
end sub

sub onCatalogResult()
    result = m.task.result
    if result = invalid then return
    if result.ok then
        count = 0
        if result.items <> invalid then count = result.items.Count()
        m.statusLabel.text = result.category + ": " + count.toStr() + " items loaded."
    else
        showError("Catalog request failed", result.detail)
    end if
end sub

sub showError(title as string, detail as string)
    dialog = CreateObject("roSGNode", "Dialog")
    dialog.title = title
    dialog.message = detail
    dialog.buttons = ["OK"]
    dialog.observeField("buttonSelected", "onDialogClosed")
    m.top.dialog = dialog
end sub

sub onDialogClosed()
    if m.top.dialog <> invalid then m.top.dialog.close = true
end sub

sub onDeepLink()
    id = m.top.deepLinkContentId
    if id = invalid or id = "" then return
    m.statusLabel.text = "Deep link: " + id
end sub

function onKeyEvent(key as string, press as boolean) as boolean
    if not press then return false
    if key = "info" or key = "options" then
        promptForServerUrl()
        return true
    end if
    if key = "back" then
        if m.top.dialog <> invalid then
            m.top.dialog.close = true
            return true
        end if
    end if
    return false
end function

sub promptForServerUrl()
    keyboard = CreateObject("roSGNode", "KeyboardDialog")
    keyboard.title = "MediaKraken server URL"
    keyboard.text = MK_GetServerUrl()
    keyboard.buttons = ["Save", "Cancel"]
    keyboard.observeField("buttonSelected", "onServerUrlEntered")
    m.serverDialog = keyboard
    m.top.dialog = keyboard
end sub

sub onServerUrlEntered()
    if m.serverDialog = invalid then return
    if m.serverDialog.buttonSelected = 0 then
        url = m.serverDialog.text
        if url <> invalid and url <> "" then
            MK_RegistryWrite("ServerUrl", url)
            m.serverLabel.text = "Server: " + url
        end if
    end if
    m.serverDialog.close = true
    m.serverDialog = invalid
end sub

sub init()
    m.icon = m.top.findNode("icon")
    m.caption = m.top.findNode("caption")
end sub

sub onContentChanged()
    content = m.top.itemContent
    if content = invalid then return
    m.caption.text = content.title
    if content.HDPosterUrl <> "" then m.icon.uri = content.HDPosterUrl
end sub

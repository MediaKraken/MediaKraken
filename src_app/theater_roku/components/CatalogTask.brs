' CatalogTask: fetch a category listing from the MediaKraken server.
' Runs on a Task thread; only reads/writes its own interface fields.
sub init()
    m.top.functionName = "execute"
end sub

sub execute()
    serverUrl = m.top.serverUrl
    category = m.top.category
    if serverUrl = invalid or serverUrl = "" then
        m.top.result = failure(category, "server URL is empty")
        return
    end if
    if category = invalid or category = "" then
        m.top.result = failure(category, "category is empty")
        return
    end if

    url = serverUrl + "/api/titlesearch/" + urlEncode(category)

    port = CreateObject("roMessagePort")
    transfer = CreateObject("roUrlTransfer")
    transfer.setMessagePort(port)
    transfer.setCertificatesFile("common:/certs/ca-bundle.crt")
    transfer.initClientCertificates()
    transfer.setUrl(url)
    transfer.addHeader("Accept", "application/json")

    if not transfer.asyncGetToString() then
        m.top.result = failure(category, "asyncGetToString failed for " + url)
        return
    end if

    timeoutMs = 10000
    msg = wait(timeoutMs, port)
    if msg = invalid then
        transfer.asyncCancel()
        m.top.result = failure(category, "request timed out after " + timeoutMs.toStr() + "ms")
        return
    end if

    if type(msg) <> "roUrlEvent" then
        m.top.result = failure(category, "unexpected event " + type(msg))
        return
    end if

    code = msg.getResponseCode()
    body = msg.getString()
    if code <> 200 then
        m.top.result = failure(category, "HTTP " + code.toStr() + " from " + url)
        return
    end if

    parsed = ParseJson(body)
    items = []
    if type(parsed) = "roArray" then
        items = parsed
    else if type(parsed) = "roAssociativeArray" and parsed.DoesExist("items") and type(parsed.items) = "roArray" then
        items = parsed.items
    end if

    m.top.result = {
        ok: true,
        category: category,
        items: items,
        detail: ""
    }
end sub

function failure(category as dynamic, detail as string) as object
    cat = ""
    if category <> invalid then cat = category
    return {
        ok: false,
        category: cat,
        items: [],
        detail: detail
    }
end function

function urlEncode(value as string) as string
    transfer = CreateObject("roUrlTransfer")
    return transfer.escape(value)
end function

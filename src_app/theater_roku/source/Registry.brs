' Thin wrappers around roRegistrySection for the "MediaKraken" namespace.
function MK_RegistryRead(key as string, default as string) as string
    section = CreateObject("roRegistrySection", "MediaKraken")
    if section = invalid then return default
    if not section.Exists(key) then return default
    value = section.Read(key)
    if value = invalid or value = "" then return default
    return value
end function

function MK_RegistryWrite(key as string, value as string) as boolean
    section = CreateObject("roRegistrySection", "MediaKraken")
    if section = invalid then return false
    ok = section.Write(key, value)
    section.Flush()
    return ok
end function

function MK_DefaultServerUrl() as string
    ' Override at sideload time by writing the "ServerUrl" registry key, or
    ' edit this fallback for a fixed deployment.
    return "http://mediakraken.local:8080"
end function

function MK_GetServerUrl() as string
    return MK_RegistryRead("ServerUrl", MK_DefaultServerUrl())
end function

' Entry point for the MediaKraken Roku channel.
' Launches the SceneGraph HomeScene and pumps the message port until exit.
sub Main(args as dynamic)
    screen = CreateObject("roSGScreen")
    port = CreateObject("roMessagePort")
    screen.setMessagePort(port)

    scene = screen.CreateScene("HomeScene")
    screen.show()

    if type(args) = "roAssociativeArray" and args.DoesExist("contentId") then
        scene.deepLinkContentId = args.contentId
    end if

    while true
        msg = wait(0, port)
        msgType = type(msg)
        if msgType = "roSGScreenEvent" then
            if msg.isScreenClosed() then return
        end if
    end while
end sub

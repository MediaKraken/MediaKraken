#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/RoseSecurity/Abusing-Roku-APIs
// http://sdkdocs.roku.com/display/sdkdoc/External+Control+Guide

/*
def com_roku_network_command(roku_addr, roku_port, roku_command, roku_command_seconds):
    """
    Send command to roku device
    """
    # urllib2.post('http://' + self.roku_address + "/keypress/" + roku_command)
    if roku_command_seconds > 0:
        urllib.request.urlopen(
            roku_addr + ':' + roku_port + '/keydown/' + roku_command)
        time.sleep(roku_command_seconds)
        response = urllib.request.urlopen(
            roku_addr + ':' + roku_port + '/keyup/' + roku_command)
    else:
        response = urllib.request.urlopen(
            roku_addr + ':' + roku_port + '/keypress/' + roku_command)
    return response


def com_roku_network_app_query(roku_addr, roku_port):
    """
    Get list of apps installed
    """
    return urllib.request.urlopen(roku_addr + ':' + roku_port + '/query/apps')


def com_roku_network_app_launch(roku_addr, roku_port, roku_app_id):
    """
    Launch app by id
    """
    return urllib.request.urlopen(roku_addr + ':' + roku_port + '/launch/' + roku_app_id)


def com_roku_network_app_icon(roku_addr, roku_port, roku_app_id):
    """
    Grab app icon
    """
    return urllib.request.urlopen(roku_addr + ':' + roku_port + '/query/icon/' + roku_app_id)


def com_roku_network_touch(roku_addr, roku_port, x_pos, y_pos):
    """
    'Click' screen
    """
    return urllib.request.urlopen(roku_addr + ':' + roku_port + '/input?touch.0.x=' + str(x_pos)
                                  + '.0&touch.0.y=' + str(y_pos) + '.0&touch.0.op=down')
 */
'''
  Copyright (C) 2016 Quinn D Granfor <spootdev@gmail.com>

  This program is free software; you can redistribute it and/or
  modify it under the terms of the GNU General Public License
  version 2, as published by the Free Software Foundation.

  This program is distributed in the hope that it will be useful, but
  WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
  General Public License version 2 for more details.

  You should have received a copy of the GNU General Public License
  version 2 along with this program; if not, write to the Free
  Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
  MA 02110-1301, USA.
'''

from .test_webserver_base import *


def test_main_menu_server(driver):
    """
    Click server on nav menu
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('menu_server').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_backup(driver):
    """
    Click admin_navmenu_backup page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_backup').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_book_add(driver):
    """
    Click admin_navmenu_book_add page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_book_add').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_cron(driver):
    """
    Click admin_navmenu_cron page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_cron').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_cloud(driver):
    """
    Click admin_navmenu_cloud page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_cloud').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_database_stats(driver):
    """
    Click admin_navmenu_cloud page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_database_stats').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_docker_stats(driver):
    """
    Click admin_navmenu_docker_stats page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_docker_stats').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_game_metadata(driver):
    """
    Click admin_navmenu_game_metadata page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_game_metadata').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_hardware(driver):
    """
    Click admin_navmenu_hardware page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_hardware').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_library(driver):
    """
    Click admin_navmenu_library page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_library').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_messages(driver):
    """
    Click admin_navmenu_messages page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_messages').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_chart_browser(driver):
    """
    Click admin_navmenu_chart_browser page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_chart_browser').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_client_usage(driver):
    """
    Click admin_navmenu_client_usage page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_client_usage').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_all_dup(driver):
    """
    Click admin_navmenu_all_dup page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_all_dup').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_all_media(driver):
    """
    Click admin_navmenu_all_media page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_all_media').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_all_matched(driver):
    """
    Click admin_navmenu_all_matched page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_all_matched').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_all_unmatched(driver):
    """
    Click admin_navmenu_all_unmatched page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_all_unmatched').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_top10_all_time(driver):
    """
    Click admin_navmenu_top10_all_time page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_top10_all_time').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_top10_movie(driver):
    """
    Click admin_navmenu_top10_movie page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_top10_movie').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_top10_tvshow(driver):
    """
    Click admin_navmenu_top10_tvshow page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_top10_tvshow').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_top10_tveps(driver):
    """
    Click admin_navmenu_top10_tveps page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_top10_tveps').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_server_settings(driver):
    """
    Click admin_navmenu_server_settings page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_server_settings').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_server_link(driver):
    """
    Click admin_navmenu_server_link page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_server_link').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_share(driver):
    """
    Click admin_navmenu_share page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_share').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_media_import(driver):
    """
    Click admin_navmenu_media_import page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_media_import').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_transmission(driver):
    """
    Click admin_navmenu_transmission page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_transmission').click()
    assert 'MediaKraken' in driver.title


def test_admin_navmenu_users(driver):
    """
    Click admin_navmenu_users page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('admin_navmenu_users').click()
    assert 'MediaKraken' in driver.title


def test_main_menu(driver):
    """
    Click home page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('menu_home').click()
    assert 'MediaKraken' in driver.title

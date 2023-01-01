from .test_webserver_base import *


def test_main_menu_sync(driver):
    """
    Click sync on nav menu.
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('menu_sync').click()
    assert 'MediaKraken' in driver.title


def test_main_menu(driver):
    """
    Click home page link.
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('menu_home').click()
    assert 'MediaKraken' in driver.title

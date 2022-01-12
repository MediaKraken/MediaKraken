from .test_webserver_base import *


def test_main_internet(driver):
    """
    Click internet
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('user_internet_page').click()
    assert 'MediaKraken' in driver.title


def test_main_internet_youtube(driver):
    """
    Click youtube
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('user_internet_youtube_page').click()
    assert 'MediaKraken' in driver.title


def test_main_internet_vimeo(driver):
    """
    Click vimeo
    """
    driver.get(TEST_TARGET)
    driver.back()  # go back to main internet page
    driver.find_element_by_id('user_internet_vimeo_page').click()
    assert 'MediaKraken' in driver.title


def test_main_internet_twitch(driver):
    """
    Click vimeo
    """
    driver.get(TEST_TARGET)
    driver.back()  # go back to main internet page
    driver.find_element_by_id('user_internet_twitch_page').click()
    assert 'MediaKraken' in driver.title


def test_main_internet_flickr(driver):
    """
    Click vimeo
    """
    driver.get(TEST_TARGET)
    driver.back()  # go back to main internet page
    driver.find_element_by_id('user_internet_flickr_page').click()
    assert 'MediaKraken' in driver.title


def test_main_menu(driver):
    """
    Click home page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('menu_home').click()
    assert 'MediaKraken' in driver.title

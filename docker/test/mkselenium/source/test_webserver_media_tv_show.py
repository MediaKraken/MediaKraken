from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.ui import WebDriverWait

from .test_webserver_base import *


def test_main_menu_metadata_tv_shows(driver):
    """
    Click metadata tv shows on nav menu
    """
    driver.get(TEST_TARGET)
    hov = ActionChains(driver).move_to_element(
        driver.find_element_by_id('menu_metadata'))
    hov.perform()
    element = WebDriverWait(driver, 10).until(EC.element_to_be_clickable((By.ID,
                                                                          "menu_metadata_tv_shows")))
    element.click()
    assert 'MediaKraken' in driver.title


def test_main_menu(driver):
    """
    Click home page link
    """
    driver.get(TEST_TARGET)
    driver.find_element_by_id('menu_home').click()
    assert 'MediaKraken' in driver.title

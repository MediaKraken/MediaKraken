import os

import pytest  # pylint: disable=W0611
from selenium import webdriver

TEST_TARGET = 'https://th-mkbuild-1:8900'

browsers = {
    'chrome': webdriver.Chrome,
    # webdriver.ChromeOptions
    # 'firefox': webdriver.Firefox,
    # webdriver.FirefoxProfile
    # 'ie': webdriver.Ie,
    # 'Opera': webdriver.Opera,
    # 'PhantomJS': webdriver.PhantomJS,
    # webdriver.Remote
    # webdriver.DesiredCapabilities
    # webdriver.ActionChains
    # webdriver.TouchActions
    # webdriver.Proxy
}


@pytest.fixture(scope='session', params=browsers.keys())
def driver(request):
    """
    Determine if webdriver exists
    """
    if 'DISPLAY' not in os.environ:
        pytest.skip('Test requires display server')
    b = browsers[request.param]()
    request.addfinalizer(lambda *args: b.quit())
    return b

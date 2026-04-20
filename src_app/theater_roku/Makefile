PKG_NAME := theater_roku
BUILD_DIR := build
ZIP := $(BUILD_DIR)/$(PKG_NAME).zip

SOURCES := manifest \
    $(shell find source -type f -name '*.brs') \
    $(shell find components -type f \( -name '*.brs' -o -name '*.xml' \)) \
    $(shell find images -type f \( -name '*.png' -o -name '*.jpg' \) 2>/dev/null)

.PHONY: all clean sideload

all: $(ZIP)

$(ZIP): $(SOURCES)
	@mkdir -p $(BUILD_DIR)
	@rm -f $@
	zip -r $@ $(SOURCES) >/dev/null
	@echo "built $@"

clean:
	rm -rf $(BUILD_DIR)

# Override ROKU_HOST/ROKU_USER/ROKU_PASS via env or `make sideload ROKU_HOST=...`.
ROKU_HOST ?= roku.local
ROKU_USER ?= rokudev
ROKU_PASS ?= rokudev

sideload: $(ZIP)
	curl --user $(ROKU_USER):$(ROKU_PASS) --digest \
	    -F "mysubmit=Install" -F "archive=@$(ZIP)" \
	    http://$(ROKU_HOST)/plugin_install

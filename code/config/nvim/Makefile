SOURCE_TAR_GZ := $(HOME)/Downloads/nvim-linux-x86_64.tar.gz
INSTALL_PREFIX := $(HOME)/Downloads/tmp

nothing:

clean:
	-rm -rf $(INSTALL_PREFIX)/bin/nvim $(INSTALL_PREFIX)/lib/nvim $(INSTALL_PREFIX)/share/nvim \
		$(INSTALL_PREFIX)/share/man/man1/nvim.1 $(INSTALL_PREFIX)/share/icons/hicolor/128x128/apps/nvim.png

tar: clean
	mkdir -p $(INSTALL_PREFIX)
	tar -xf $(SOURCE_TAR_GZ) -C $(INSTALL_PREFIX) --strip-components=1

# To build from source, see
# https://neovim.io/doc/build/

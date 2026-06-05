# Khang's setup script for Debian 13.

USER := khang

# Note: before diving in to a minimal Debian install without browser access,
# fill in NVIDIA_VERSION first. Go to https://nvidia.com/drivers and fill out
# the GPU specs. It should lead to a page that offers a download link of the
# same format as NVIDIA_DL_URL below. Copy the version from there and update
# NVIDIA_VERSION.
NVIDIA_VERSION := 595.71.05
NVIDIA_TARGET  := Linux-x86_64
NVIDIA_DL_URL  := https://us.download.nvidia.com/XFree86/$(NVIDIA_TARGET)/$(NVIDIA_VERSION)/NVIDIA-$(NVIDIA_TARGET)-$(NVIDIA_VERSION).run
NVIDIA_DL_FILE := $(notdir $(NVIDIA_DL_URL))

# Build requirements for dwm.
APT_LIST_DWM := libx11-dev libxft-dev libfreetype-dev libfontconfig-dev

# General searching tools.
APT_LIST_SEARCH := ripgrep fd-find fzf

# General build requirements.
APT_LIST_DEV := linux-headers-generic pkg-config autoconf ninja-build cmake

# Apps that I use.
APT_LIST_APPS := zsh kitty rofi xclip gpg

APT_LIST := git curl wget rsync
APT_LIST += xorg
APT_LIST += libglvnd-dev # required to install nvidia drivers
APT_LIST += libncurses-dev # build requirements for zsh
APT_LIST += polkitd
APT_LIST += $(APT_LIST_DWM)
APT_LIST += $(APT_LIST_SEARCH)
APT_LIST += $(APT_LIST_DEV)
APT_LIST += $(APT_LIST_APPS)

all:
	@echo "Khang's setup script for Debian 13."

s1:
	# Step 1: Add sudo capabilities to current user.
	# * With the assumption that sudo is not even installed yet.
	# * Do this step manually.
	#
	# 1. Change to root user with `su -`.
	#    Using any other method may not let us use `usermod`.
	# 2. Install sudo through the package manager.
	# 3. Run `usermod -aG sudo <username>`.
	#    From the help text of `usermod` itself:
	#      -a, --append                  append the user to the supplemental GROUPS
	#                                    mentioned by the -G option without removing
	#                                    the user from other groups
	#      -G, --groups GROUPS           new list of supplementary GROUPS
	# 4. That's it. Check that the user has indeed been added with `groups <username>`.
	# 5. Log the user out and back in.


s2:
	# Step 2: Install the base list of applications.
	# * This requires sudo, which comes from step 1.
	#
	apt install $(APT_LIST)

s3: 
	# Step 3: Install NVIDIA drivers.
	# * This step requires `linux-headers` and `libglvnd-dev` to be installed.
	#
	# 1. Run `make s3-dl` to download the NVIDIA driver installer shell script.
	# 2. Run `make s3-install` to install the NVIDIA drivers.
	#     a) Select the MIT/GPL version instead of the NVIDIA Proprietary one.
	#        For some reason the Proprietary one has never worked before.
	#     b) To uninstall, run `make s3-uninstall`.
	#     c) For all options, run `make s3-help`.
	# 3. If you're experiencing issues, then maybe initramfs hasn't been updated
	#    by the NVIDIA driver installer. Run `make s3-initramfs` manually
	#    and then reboot.

s3-dl: $(NVIDIA_DL_FILE)

s3-install: $(NVIDIA_DL_FILE)
	sh $(NVIDIA_DL_FILE)

s3-uninstall: $(NVIDIA_DL_FILE)
	sh $(NVIDIA_DL_FILE) --uninstall

s3-help: $(NVIDIA_DL_FILE)
	sh $(NVIDIA_DL_FILE) --advanced-options > nvidia-help.txt
	
$(NVIDIA_DL_FILE):
	curl --fail --location --output $(NVIDIA_DL_FILE) $(NVIDIA_DL_URL)

s3-initramfs:
	update-initramfs -u

s4:
	# Step 4: Install a window manager.
	# * We'll be using dwm.
	# * This requires the list of libraries in APT_LIST_DWM.
	#
	# 1. Run `make s4-get` to get the git repo.
	# 2. Go to that repo and build dwm and install it.

s4-get:
	mkdir -p $(HOME)/repos
	git clone https://github.com/nguyenvukhang/dwm.git $(HOME)/repos/dwm

s5:
	# Step 5: Install firefox through the APT repository.
	# * At this point, it is assumed that we've installed stuff through APT
	#   already so /etc/apt/keyrings should already exist. Nevertheless, go
	#   and check that it exists manually.
	# Then,
	# 1. Run `make s5-key` to grab the GPG keys from Mozilla.
	# 2. Run `make s5-src` to add the Mozilla APT repository to your sources.list.

s5-key:
	wget -q https://packages.mozilla.org/apt/repo-signing-key.gpg -O- \
		| sudo tee /etc/apt/keyrings/packages.mozilla.org.asc > /dev/null

define MOZILLA_SOURCES
Types: deb
URIs: https://packages.mozilla.org/apt
Suites: mozilla
Components: main
Signed-By: /etc/apt/keyrings/packages.mozilla.org.asc
endef

define MOZILLA_PREFERENCES
Package: *
Pin: origin packages.mozilla.org
Pin-Priority: 1000
endef

export MOZILLA_SOURCES
export MOZILLA_PREFERENCES
s5-src:
	# Add to sources list:
	@echo "$$MOZILLA_SOURCES" > /etc/apt/sources.list.d/mozilla.sources
	# Configure APT to prioritize packages from the Mozilla repository:
	@echo "$$MOZILLA_PREFERENCES" > /etc/apt/preferences.d/mozilla

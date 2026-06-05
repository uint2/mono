# Arch Install

this set of instructions is specifically for 2017's Macbook Air. Follow with caution, and prepare a pinch of salt.

tame the brightness
```
echo 42 > /sys/class/backlight/acpi_video0/brightness
```

## chop up the drive
start with a conservative
```
fdisk -l
```
establish your victim (most probably `/dev/sda`)  
mis-en-place:
```
fdisk /dev/sda
```
create a gpt partition table `g`  
add a partition `n`
1. default number, default sector, last sector = `+550M` (min required for EFI partition)
2. default number, default sector, last sector = `+2G`
3. default number, default sector, default last sector  

change the partition types `t`
1. change to `1` (EFI System)
2. change to `19` (Linux swap)
3. leave it as `20` (Linux filesystem)

write the table `w`
## format the filesystems
```
mkfs.fat -F32 /dev/sda1
mkswap /dev/sda2
swapon /dev/sda2
mkfs.ext4 /dev/sda3
```
mount the filesystem
```
mount /dev/sda3 /mnt
```
## install linux
**this requires an internet connection**  
(what I did was to just buy a ethernet adapter for mac)  
install the bare minimum
```
pacstrap /mnt base linux linux-firmware
```
generate the file system table
```
genfstab -U /mnt >> /mnt/etc/fstab
```
change-root into the new installation
```
arch-chroot /mnt
```
## sync watch
3... 2... 1... press!
```
timedatectl set-ntp true
```
ensure watch is sunc
```
timedatectl status
```
set timezone
```
ln -sf /usr/share/zoneinfo/<region>/<city> /etc/localtime
```
> use `ls` to search the `/usr/share/zoneinfo` directory for your own region and city
## set locale
start with installing your favorite text editor
I'll do `neovim`
```
pacman -Syu neovim
```
open and edit `locale.gen`  
```
nvim /etc/locale.gen
```
uncomment the line with your locale  
for my case I changed the line
```
#en_US.UTF-8 UTF-8
```
to this
```
en_US.UTF-8 UTF-8
```
generate your locale-specific settings with
```
locale-gen
```
open and edit `locale.conf`
```
nvim /etc/locale.conf
```
what I wrote in:
```
LANG=en_US.UTF-8
```
## set your hosts
open and edit `hostname`
```
nvim /etc/hostname
```
give your computer a name, say for example `nyr`  
open and edit `hosts`
```
nvim /etc/hosts
```
fill it in like so (fill the gaps with tabs/spaces)
```
127.0.0.1   localhost
::1         localhost
127.0.1.1   nyr.localdomain   nyr
```
remember to use your own hostname instead of `nyr`
## set up accounts
set up root password
```
passwd
```
create your user
```
useradd -m your_username
```
set your user password
```
passwd your_username
```
add your user to some exclusive groups
```
usermod -aG wheel,audio,video,optical,storage your_username
```
install sudo
```
pacman -Syu sudo
```
edit `visudo`
```
EDITOR=nvim visudo
```
uncomment the wheel group  
I changed the line
```
# %wheel ALL=(ALL) ALL
```
to this:
```
%wheel ALL=(ALL) ALL
```
## set up boot items
install the necessary tools
```
pacman -Syu grub efibootmgr dosfstools os-prober mtools
```
set up EFI partition
```
mkdir /boot/EFI
mount /dev/sda1 /boot/EFI
```
install grub
```
grub-install --target=x86_64-efi --bootloader-id=grub_uefi --recheck
grub-mkconfig -o /boot/grub/grub.cfg
```
## congratulate yourself
at this point, you can reboot and survive it.
but first, might as well install some essentials first
```
pacman -Syu networkmanager
systemctl enable NetworkManager
```
prime things for Macbook Air wifi:
```
pacman -S linux-headers dkms broadcom-wl-dkms iw
```
## pack up go home
exit the chroot
```
exit
```
unmount
```
umount -l /mnt
```
# Arch Setup
## pacman boys
really basic stuff
```
chsh -s /bin/zsh
xf86-video-intel
```
finish setting up neovim:
```
sudo pacman -Syu python-pip perl
pip install pynvim
```
get the AUR helper up and running
```
sudo pacman -Syu git openssh
# AUR helper
cd /opt && sudo git clone https://aur.archlinux.org/yay-git.git
sudo chown -R nyr ./yay-git
cd yay-git && makepkg -si
```
get Xmonad up and running (required to get onto GitHub to enable SSH)
```
# install cursor theme
cd cursors
tar -xvf macOSBigSur.tar.gz
mv macOSBigSur /usr/share/icons
sudo pacman -Syu xmonad xmonad-contrib picom libnotify dunst kitty playerctl rofi feh noto-fonts-cjk xmobar maim xorg xorg-xinit firefox-ublock-origin xclip cronie
```
download my own config files
```
# set up basic git configs
git config --global user.name "name"
git config --global user.email "email"

# set up openssh
ssh-keygen -t ed25519 -C "your_email@example.com"
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# copy the contents of ~/.ssh/id_ed25519 and paste it in GitHub

cd && mkdir -p repos
cd repos
git clone git@github.com:NguyenVuKhang/arch.git arch
```
install my apps
```
sudo pacman -Syu fzf the_silver_searcher ripgrep unzip gzip xorg firefox lsd maim qmk vlc noto-fonts-cjk 
yay -S spotify
```
set up hourly alert:
```
crontab -e
0 * * * * export DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus; export DISPLAY=:0 . $HOME/.profile; ~/.config/zsh/scripts/hour-bell.sh
:wq
```
set up tlp:
```
sudo pacman -S tlp
systemctl enable tlp.service
```

sudo dd if=2025-10-01-raspios-trixie-arm64.img.xz of=/dev/sda bs=4M

bah, just use the pi imager

sudo apt-get install git libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# restart terminal
cargo install cargo-audit

git clone https://github.com/MediaKraken/MediaKraken

cd MediaKraken
git checkout dev

sudo apt-get -y install libx11-dev libxext-dev libxft-dev libxinerama-dev \
libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev

sudo apt-get -y install librust-libudev-sys-dev

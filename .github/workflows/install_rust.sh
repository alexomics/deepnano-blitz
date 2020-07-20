curl https://sh.rustup.rs -sSfo install-rust.sh
sh install-rust.sh -y
source $HOME/.cargo/env
rustup default nightly

if [ "$(uname)" == "Darwin" ]; then
    brew install openssl@1.1
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    yum install -y openssl-devel
fi

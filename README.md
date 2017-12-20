# forkoff

Experiment that uses kafel to set seccomp filters to prevent forking
Really bad prototype using neon + rust to build bindings.

(very hacky)

get kafel

    git submodule update --init --recursive

install rust (if not already)
	curl https://sh.rustup.rs -sSf | sh

install neon-cli
	npm install -g neon-cli

make libkafel
	build.rs will look for the file in the root directory
	node expects libkafel.so.1 to exist so you need to symlink or rename

build
	neon build

run
	node lib/index.js



Currently the process just hangs... so that's not good.. 

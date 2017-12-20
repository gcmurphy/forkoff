# forkoff

Experiment that uses kafel to set seccomp filters to prevent forking
Really bad prototype using neon + rust to build bindings.

## Hacking

### Install submodules:

```
git submodule update --init --recursive
```


### Install Rust

```
curl https://sh.rustup.rs -sSf | sh
```

### Install [Neon](https://www.neon-bindings.com/)

```
npm install -g neon-cli
```

### Ensure libkafel is built and accessible

* build.rs will look for the file in the root directory
* node expects libkafel.so.1 to exist so you need to symlink or rename

```
cd vendor/kafel
make
cp libkafel.so ../
cp libkafel.so ../libkafel.so.1
```

### Run the build

``` 
neon build 
```


### Run the demo

```
npm start
```



Currently the process just hangs... so that's not good.. I need to look into this some more...

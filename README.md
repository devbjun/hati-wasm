# **HATI-WASM**

## **About**
[Swiss Ephemeris](http://www.astro.com/swisseph/swephinfo_e.htm) binding for Yew WASM starter template of Rust-lang.  
[HATI](https://github.com/devbjun/hati) is Horoscope Astroploy Tool Integration. 

## **Development Environment**

OS: Windows 11 (WSL2: Debian Bullseye)  
Language: Rust (2021 Edition)  

## **Getting Started**
To preinstall run:
``` bash
# OpenSSL
$ sudo apt install libssl-dev pkg-config

# Cargo
$ cargo install wasm-pack
$ cargo install cargo-make
$ cargo install simple-http-server
```

To build:
``` bash
$ cargo make build
```

To run:
``` bash
$ cargo make serve
```

## **License**
The license for this project is the same as original [Swiss Ephemeris](http://www.astro.com/swisseph/swephinfo_e.htm).

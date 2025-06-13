RDKit-Sys
---

Rust code that binds to the C++ rdkit library!

How does it work?
---

RDKit is a C++ mega-library, full of cheminformatics wisdom. We don't want to rewrite RDKit in Rust, we should instead meet somewhere in the middle and
"bridge" Rust to C++ through some wrappers.

The goal is to do 1-1 bindings with the C++ library, exposing all the classes as we need them. The goal is _not_ to create
a high-level functionality like the MinimalLib (cffiwrapper). Our goal is to expose the building blocks. If you're looking
for idiomatic Rust, check out the [rdkit](https://crates.io/crate/rdkit) crate.

Prerequisites
---

On Mac:

    brew install rdkit

Also known to work with conda-managed RDKit, be sure to set the `dynamic-linking-from-conda` feature. Not as tested, please open an issue if you have a hard time.

Linking rdkit statically
---

It is possible to link boost and rdkit statically:

    rust-sys = {version = "0.4.12", features = ["static-linking"]}

Testing
---

Or just run the test suite:

    cargo test

TODO
---

 - [ ] figure out how to `cargo publish` without `--no-verify` (otherwise it detects changes outside of OUTDIR)
 - [X] specify path to RDKit's cffiwrapper.h and all required search paths for other dependent headers
 - [X] use conditional rebuild logic to make the library build experience more reliable (for now, if you get stuck, try `cargo clean` and retry with `cargo build -vv`)

Related Documentation
---

 - https://www.rdkit.org/docs/cppapi/index.html
 - https://cxx.rs/

Prior art
---

 - https://github.com/apahl/rdkit_cxx
 - [rdkafka's excellent librdkafka build.rs](https://github.com/fede1024/rust-rdkafka/blob/master/rdkafka-sys/build.rs)
 - https://iwatobipen.wordpress.com/2022/01/29/use-rdkit-from-rust-rdkit-rdkitcffi-rust/
 - [an attempt at using rdkit in rust but without docs on how to build rdkit](https://github.com/iwatobipen/rust_rdkit/)

Building rdKit on Linux
---

On linux, you need to build the static rdkit library from source.

First, install build prerequisities:

    TODO

You will need the boost library:

    sudo apt-get install libboost-all-dev

Now you can clone rdkit source code and compile the binaries:

    git clone https://github.com/rdkit/rdkit.git
    cd rdkit
    mkdir build && cd build
    cmake \
        -DRDK_BUILD_PYTHON_WRAPPERS=OFF\
        -DRDK_BUILD_INCHI_SUPPORT=ON\
        -DRDK_BUILD_CAIRO_SUPPORT=ON\
        -DRDK_BUILD_XYZ2MOL_SUPPORT=ON\
        ..
    make -j8 install

Make sure the header files and compiled binaries are in path:

    export CPLUS_INCLUDE_PATH=$(realpath ../Code)
    export LIBRARY_PATH=$(realpath ../lib)
    export LD_LIBRARY_PATH=$(realpath ../lib)

Now you should be able to compile rust projects with rdkit as a dependency!

export CPLUS_INCLUDE_PATH="/home/honza/Desktop/temp/rdkit_cpp/rdkit/Code:/home/honza/Desktop/temp/rdkit_cpp/rdkit/External"
export LIBRARY_PATH="/home/honza/Desktop/temp/rdkit_cpp/rdkit/lib"
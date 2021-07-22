# The Rust Programming Language for Espressif chips

This fork enables projects to be built for the Xtensa-based ESP32, ESP32-SXX and ESP8266 using [Espressif's llvm fork](https://github.com/espressif/llvm-project). (RiscV chips like ESP32-CXX are already supported in stock Rust.)

Moreover, this fork enables Rust STD support (networking, threads, and filesystem) for all chips in the ESP32 family (Xtensa and RiscV), by optionally linking with the ESP-IDF framework.

The [esp-rs](https://github.com/esp-rs) organization has been formed to develop runtime, pac and hal crates for the Espressif chips (bare-metal as well as ESP-IDF based).

## Installation

Espressif offers [pre-built binaries of this fork](https://github.com/espressif/rust-esp32-example/blob/main/docs/rust-on-xtensa.md). Follow the instructions for your operating system.

## Building
Install [Rustup](https://rustup.rs/).

Build using these steps:
```sh
$ git clone https://github.com/esp-rs/rust
$ cd rust
$ git checkout esp
$ ./configure --experimental-targets=Xtensa
$ ./x.py build --stage 2
```

* **NOTE 1**: Building might take **close to an hour**
* **NOTE 2**: Make sure you are using the `esp` GIT branch of the fork (the default)
* **NOTE 3**: Do NOT rename the directory ('rust') where you've cloned the Rust fork. It must be 'rust' or you might have strange issues later on when using it. You can however place it anywhere in your file tree

### Fix toolchain vendor/ directory, so that building STD with Cargo does work

(Assuming you are still in the rust/ directory):

```sh
$ mkdir vendor
$ cd vendor
$ ln -s ../library/rustc-std-workspace-alloc/ rustc-std-workspace-alloc
$ ln -s ../library/rustc-std-workspace-core/ rustc-std-workspace-core
$ ln -s ../library/rustc-std-workspace-std/ rustc-std-workspace-std
```

Make Rustup aware of the newly built compiler:

```sh
$ rustup toolchain link esp ~/<...>/rust/build/x86_64-unknown-linux-gnu/stage2
```

Switch to the new compiler in Rustup:

```sh
$ rustup default esp
```

Check the compiler:
```sh
$ rustc --print target-list
```

At the end of the printed list of targets you should see:
```
...
xtensa-esp32-none-elf
xtensa-esp8266-none-elf
xtensa-none-elf
```

## Building LLVM clang

You'll need the custom LLVM clang based on the Espressif LLVM fork for Rust STD support. Build as follows:
```sh
$ git clone https://github.com/espressif/llvm-project
$ cd llvm-project
$ mkdir build
$ cd build
$ cmake -G Ninja -DLLVM_ENABLE_PROJECTS='clang' -DCMAKE_BUILD_TYPE=Release ../llvm
$ cmake --build .
$ export PATH=`pwd`/bin:$PATH
```

Check that you have the custom clang on your path:
```sh
$ which clang
$ which llvm-config
```

The above should output locations pointing at your custom-built clang toolchain.

* **NOTE 1**: Building LLVM clang might take **even longer** time than building the Rustc toolchain
* **NOTE 2**: You might want to make the PATH modification step from above permanent. Please make sure that the custom Clang compiler is the first on your PATH so that it takes precedence over any clang compiler you might have installed using your distro / OS

## Updating this fork

The patch set can be found [here](https://github.com/MabezDev/rust-xtensa-patches). Checkout from upstream/master, apply the patches on at a time using `git am -3 < path/to/patch.patch`, fixing any conflicts if necessary (remember to PR the changes back to the patches [repo]((https://github.com/MabezDev/rust-xtensa-patches))). Once it builds submit a PR against this repo with the branch name `esp-update-$DATE`.

If the llvm submodule needs to be updated, the following should work:

```bash
git submodule set-url src/llvm-project https://github.com/espressif/llvm-project
git submodule set-branch -b $BRANCH_NAME src/llvm-project
git submodule update --init --recursive --remote src/llvm-project
```

Once accepted, the new branch will be renamed `esp-target`, hence making it the default.
Don't worry about the README changes, I will port those across once I accept the PR.

---

This is the main source code repository for [Rust]. It contains the compiler,
standard library, and documentation.

[Rust]: https://www.rust-lang.org

**Note: this README is for _users_ rather than _contributors_.
If you wish to _contribute_ to the compiler, you should read the
[Getting Started][gettingstarted] of the rustc-dev-guide instead of this
section.**

## Quick Start

Read ["Installation"] from [The Book].

["Installation"]: https://doc.rust-lang.org/book/ch01-01-installation.html
[The Book]: https://doc.rust-lang.org/book/index.html

## Installing from Source

The Rust build system uses a Python script called `x.py` to build the compiler,
which manages the bootstrapping process. More information about it can be found
by running `./x.py --help` or reading the [rustc dev guide][rustcguidebuild].

[gettingstarted]: https://rustc-dev-guide.rust-lang.org/getting-started.html
[rustcguidebuild]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html

### Building on a Unix-like system
1. Make sure you have installed the dependencies:

   * `g++` 5.1 or later or `clang++` 3.5 or later
   * `python` 3 or 2.7
   * GNU `make` 3.81 or later
   * `cmake` 3.4.3 or later
   * `ninja`
   * `curl`
   * `git`
   * `ssl` which comes in `libssl-dev` or `openssl-devel`
   * `pkg-config` if you are compiling on Linux and targeting Linux

2. Clone the [source] with `git`:

   ```sh
   $ git clone https://github.com/rust-lang/rust.git
   $ cd rust
   ```

[source]: https://github.com/rust-lang/rust

3. Configure the build settings:

    The Rust build system uses a file named `config.toml` in the root of the
    source tree to determine various configuration settings for the build.
    Copy the default `config.toml.example` to `config.toml` to get started.

    ```sh
    $ cp config.toml.example config.toml
    ```

    If you plan to use `x.py install` to create an installation, it is recommended
    that you set the `prefix` value in the `[install]` section to a directory.

    Create install directory if you are not installing in default directory

4. Build and install:

    ```sh
    $ ./x.py build && ./x.py install
    ```

    When complete, `./x.py install` will place several programs into
    `$PREFIX/bin`: `rustc`, the Rust compiler, and `rustdoc`, the
    API-documentation tool. This install does not include [Cargo],
    Rust's package manager. To build and install Cargo, you may
    run `./x.py install cargo` or set the `build.extended` key in
    `config.toml` to `true` to build and install all tools.

[Cargo]: https://github.com/rust-lang/cargo

### Building on Windows

There are two prominent ABIs in use on Windows: the native (MSVC) ABI used by
Visual Studio, and the GNU ABI used by the GCC toolchain. Which version of Rust
you need depends largely on what C/C++ libraries you want to interoperate with:
for interop with software produced by Visual Studio use the MSVC build of Rust;
for interop with GNU software built using the MinGW/MSYS2 toolchain use the GNU
build.

#### MinGW

[MSYS2][msys2] can be used to easily build Rust on Windows:

[msys2]: https://msys2.github.io/

1. Grab the latest [MSYS2 installer][msys2] and go through the installer.

2. Run `mingw32_shell.bat` or `mingw64_shell.bat` from wherever you installed
   MSYS2 (i.e. `C:\msys64`), depending on whether you want 32-bit or 64-bit
   Rust. (As of the latest version of MSYS2 you have to run `msys2_shell.cmd
   -mingw32` or `msys2_shell.cmd -mingw64` from the command line instead)

3. From this terminal, install the required tools:

   ```sh
   # Update package mirrors (may be needed if you have a fresh install of MSYS2)
   $ pacman -Sy pacman-mirrors

   # Install build tools needed for Rust. If you're building a 32-bit compiler,
   # then replace "x86_64" below with "i686". If you've already got git, python,
   # or CMake installed and in PATH you can remove them from this list. Note
   # that it is important that you do **not** use the 'python2', 'cmake' and 'ninja'
   # packages from the 'msys2' subsystem. The build has historically been known
   # to fail with these packages.
   $ pacman -S git \
               make \
               diffutils \
               tar \
               mingw-w64-x86_64-python \
               mingw-w64-x86_64-cmake \
               mingw-w64-x86_64-gcc \
               mingw-w64-x86_64-ninja
   ```

4. Navigate to Rust's source code (or clone it), then build it:

   ```sh
   $ ./x.py build && ./x.py install
   ```

#### MSVC

MSVC builds of Rust additionally require an installation of Visual Studio 2017
(or later) so `rustc` can use its linker.  The simplest way is to get the
[Visual Studio], check the “C++ build tools” and “Windows 10 SDK” workload.

[Visual Studio]: https://visualstudio.microsoft.com/downloads/

(If you're installing cmake yourself, be careful that “C++ CMake tools for
Windows” doesn't get included under “Individual components”.)

With these dependencies installed, you can build the compiler in a `cmd.exe`
shell with:

```sh
> python x.py build
```

Currently, building Rust only works with some known versions of Visual Studio. If
you have a more recent version installed and the build system doesn't understand,
you may need to force rustbuild to use an older version. This can be done
by manually calling the appropriate vcvars file before running the bootstrap.

```batch
> CALL "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
> python x.py build
```

#### Specifying an ABI

Each specific ABI can also be used from either environment (for example, using
the GNU ABI in PowerShell) by using an explicit build triple. The available
Windows build triples are:
- GNU ABI (using GCC)
    - `i686-pc-windows-gnu`
    - `x86_64-pc-windows-gnu`
- The MSVC ABI
    - `i686-pc-windows-msvc`
    - `x86_64-pc-windows-msvc`

The build triple can be specified by either specifying `--build=<triple>` when
invoking `x.py` commands, or by copying the `config.toml` file (as described
in [Installing From Source](#installing-from-source)), and modifying the
`build` option under the `[build]` section.

### Configure and Make

While it's not the recommended build system, this project also provides a
configure script and makefile (the latter of which just invokes `x.py`).

```sh
$ ./configure
$ make && sudo make install
```

When using the configure script, the generated `config.mk` file may override the
`config.toml` file. To go back to the `config.toml` file, delete the generated
`config.mk` file.

## Building Documentation

If you’d like to build the documentation, it’s almost the same:

```sh
$ ./x.py doc
```

The generated documentation will appear under `doc` in the `build` directory for
the ABI used. I.e., if the ABI was `x86_64-pc-windows-msvc`, the directory will be
`build\x86_64-pc-windows-msvc\doc`.

## Notes

Since the Rust compiler is written in Rust, it must be built by a
precompiled "snapshot" version of itself (made in an earlier stage of
development). As such, source builds require a connection to the Internet, to
fetch snapshots, and an OS that can execute the available snapshot binaries.

Snapshot binaries are currently built and tested on several platforms:

| Platform / Architecture                     | x86 | x86_64 |
|---------------------------------------------|-----|--------|
| Windows (7, 8, 10, ...)                     | ✓   | ✓      |
| Linux (kernel 2.6.32, glibc 2.11 or later)  | ✓   | ✓      |
| macOS (10.7 Lion or later)                  | (\*) | ✓      |

(\*): Apple dropped support for running 32-bit binaries starting from macOS 10.15 and iOS 11.
Due to this decision from Apple, the targets are no longer useful to our users.
Please read [our blog post][macx32] for more info.

[macx32]: https://blog.rust-lang.org/2020/01/03/reducing-support-for-32-bit-apple-targets.html

You may find that other platforms work, but these are our officially
supported build environments that are most likely to work.

## Getting Help

The Rust community congregates in a few places:

* [Stack Overflow] - Direct questions about using the language.
* [users.rust-lang.org] - General discussion and broader questions.
* [/r/rust] - News and general discussion.

[Stack Overflow]: https://stackoverflow.com/questions/tagged/rust
[/r/rust]: https://reddit.com/r/rust
[users.rust-lang.org]: https://users.rust-lang.org/

## Contributing

If you are interested in contributing to the Rust project, please take a look
at the [Getting Started][gettingstarted] guide in the [rustc-dev-guide].

[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org

## License

Rust is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

## Trademark

The Rust programming language is an open source, community project governed
by a core team. It is also sponsored by the Mozilla Foundation (“Mozilla”),
which owns and protects the Rust and Cargo trademarks and logos
(the “Rust Trademarks”).

If you want to use these names or brands, please read the [media guide][media-guide].

Third-party logos may be subject to third-party copyrights and trademarks. See
[Licenses][policies-licenses] for details.

[media-guide]: https://www.rust-lang.org/policies/media-guide
[policies-licenses]: https://www.rust-lang.org/policies/licenses
In the third post of this series, let's talk about `topgrade`.

# What is topgrade?
You've probably run into this scenario before: you have a package manager for your system; `brew` on Mac, `pacman` on Arch Linux etc, and you have a bunch of packages installed using these package managers.

Now comes the twist, you end up having several other package managers (eg: `pip`, `npm`); because nobody can agree on a standard and it's probably too far gone at this point to care anyway.

To keep the entirety of your system up to date, you end up having to run several commands, which eats up precious time that you could be spending watching cat videos.

`topgrade` remedies this situation by detecting the tools and package managers installed on your system and runs the appropriate commands to update them.

# How do I get it?
Pretty straightforward. `topgrade` provides packages for Arch Linux, NixOS, and Mac (via `homebrew` or `macports`).

If you're on a different platform or OS, install `topgrade` via the `cargo` package manager by running `cargo install topgrade`. 

If you're a normie who doesn't use Rust, go ahead and download one of the binaries available in the [releases](https://github.com/r-darwish/topgrade/releases) and place it on your `$PATH`.

# Usage
Dead simple. Just run `topgrade`.
[![asciicast](https://asciinema.org/a/499879.svg)](https://asciinema.org/a/499879)

And that's it from me. Enjoy those cat videos!
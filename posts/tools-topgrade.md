---
title: 'Tools of the Trade: Topgrade'
tags: [tooling, sysadmin]
date: 2022-06-07
blurb: "In the third post of this series, let's talk about `topgrade`."
---

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

# Configuration
`topgrade` can be configured via a configuration file, which is (usually) located at `$HOME/.config/topgrade.toml`. An example file is shown here.

```toml
# Don't ask for confirmations
#assume_yes = true

# Disable specific steps - same options as the command line flag
disable = ["pnpm"]

# Ignore failures for these steps
ignore_failures = ["powershell"]

# Run specific steps - same options as the command line flag
#only = ["system", "emacs"]

# Do not ask to retry failed steps (default: false)
#no_retry = true

# Run inside tmux
run_in_tmux = true

# List of remote machines with Topgrade installed on them
#remote_topgrades = ["toothless", "pi", "parnas"]

# Arguments to pass SSH when upgrading remote systems
#ssh_arguments = "-o ConnectTimeout=2"

# Path to Topgrade executable on remote machines
#remote_topgrade_path = ".cargo/bin/topgrade"

# Arguments to pass tmux when pulling Repositories
#tmux_arguments = "-S /var/tmux.sock"

# Do not set the terminal title
#set_title = false

# Display the time in step titles
# display_time = true

# Cleanup temporary or old files
#cleanup = true

[git]
#max_concurrency = 5
# Additional git repositories to pull
#repos = [
#    "~/src/*/",
#    "~/.config/something"
#]

# Don't pull the predefined git repos
#predefined_repos = false

# Arguments to pass Git when pulling Repositories
#arguments = "--rebase --autostash"

[composer]
#self_update = true

# Commands to run before anything
[pre_commands]
#"Emacs Snapshot" = "rm -rf ~/.emacs.d/elpa.bak && cp -rl ~/.emacs.d/elpa ~/.emacs.d/elpa.bak"

# Custom commands
[commands]
#"Python Environment" = "~/dev/.env/bin/pip install -i https://pypi.python.org/simple -U --upgrade-strategy eager jupyter"

[brew]
#greedy_cask = true

[linux]
# Arch Package Manager to use. Allowed values: autodetect, trizen, paru, yay, pacman.
#arch_package_manager = "pacman"
# Arguments to pass yay (or paru) when updating packages
#yay_arguments = "--nodevel"
#show_arch_news = true
#trizen_arguments = "--devel"
#enable_tlmgr = true
#emerge_sync_flags = "-q"
#emerge_update_flags = "-uDNa --with-bdeps=y world"
#redhat_distro_sync = false
#rpm_ostree = false

[windows]
# Manually select Windows updates
#accept_all_updates = false
#open_remotes_in_new_terminal = true

# Causes Topgrade to rename itself during the run to allow package managers
# to upgrade it. Use this only if you installed Topgrade by using a package
# manager such as Scoop to Cargo
#self_rename = true

[npm]
# Use sudo if the NPM directory isn't owned by the current user
use_sudo = true

[firmware]
# Offer to update firmware; if false just check for and display available updates
upgrade = true

[flatpak]
# Use sudo for updating the system-wide installation
#use_sudo = true
```

# Usage
Dead simple. Just run `topgrade`.

<script id="asciicast-499879" src="https://asciinema.org/a/499879.js" async></script>

And that's it from me. Enjoy those cat videos!
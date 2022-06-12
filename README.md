[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[total-lines]: https://img.shields.io/tokei/lines/github/MedzikUser/HomeDisk?style=for-the-badge&logo=github&color=fede00
[code-size]: https://img.shields.io/github/languages/code-size/MedzikUser/HomeDisk?style=for-the-badge&color=c8df52&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[ci]: https://img.shields.io/github/workflow/status/MedzikUser/rust-crypto-utils/Rust/main?style=for-the-badge&logo=github

# Imgurs - CLI and Library for Imgur API

[![github]](https://github.com/MedzikUser/imgurs)
[![total-lines]](https://github.com/MedzikUser/HomeDisk)
[![code-size]](https://github.com/MedzikUser/HomeDisk)
[![crates-io]](https://crates.io/crates/imgurs)
[![docs-rs]](https://docs.rs/imgurs)
[![ci]](https://github.com/MedzikUser/imgurs/actions/workflows/rust.yml)

## Screenshots

![](https://i.imgur.com/MG35kvf.png)

![](https://i.imgur.com/TSxBrhO.png)

## Shell completions

Here are some examples of usage
```bash
# For bash
imgurs completions bash > ~/.local/share/bash-completion/completions/imgurs
# For zsh
imgurs completions zsh > /usr/local/share/zsh/site-functions/_imgurs
# For fish
imgurs completions fish > ~/.config/fish/completions/imgurs.fish
```

## Man page

Generate manpage

    imgurs manpage | gzip > /usr/share/man/man1/imgurs.1.gz

## Dependencies
- support clipboard on Linux
    - **xsel**
    - **xclip** - alternative to **xsel**
    - **termux-api** - on **Termux**
- **libnotify** - support notification on Linux

## How to install Imgurs CLI?

### **Linux**
Download imgurs-linux from [the releases page](https://github.com/MedzikUser/imgurs/releases/latest) and run

    chmod +x imgurs-linux
    ./imgurs-linux

#### **Arch Linux**
Using yay ([AUR](https://aur.archlinux.org/packages/imgurs))

    yay -S imgurs

or can add [this repo](https://github.com/archlinux-pkg/packages) and run

    sudo pacman -Sy imgurs

### **OSX**
Download imgurs-darwin from [the releases page](https://github.com/MedzikUser/imgurs/releases/latest) and run

    chmod +x imgurs-darwin
    ./imgurs-darwin

### **Windows**
Download imgurs-windows.exe from [the releases page](https://github.com/MedzikUser/imgurs/releases/latest) and run

    imgurs-windows.exe

### **Compile with Cargo**
Make sure you have a recent version of Rust. Then you can run

    cargo install imgurs

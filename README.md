# Imgurs - CLI and Library for Imgur API

## Screenshots

![upload](https://cdn.magicuser.cf/tcqqxEh.png)

![delete](https://cdn.magicuser.cf/hBqbsIm.png)

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

## How to install Imgurs CLI?

### **Linux**
Download imgurs-linux from [the releases page](https://github.com/MedzikUser/imgurs/releases/latest) and run

    chmod +x imgurs-linux
    ./imgurs-linux

#### **Arch Linux**
Using yay ([AUR](https://aur.archlinux.org/packages/imgurs))

    yay -S imgurs

If you are using arch linux you can add [this repo](https://github.com/archlinux-pkg/packages) and run

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

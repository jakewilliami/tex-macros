# LaTeX Macros

This repository will show the progression of my macros used on LaTeX, in order to **a)** keep track of history of my macros, and **b)** extend my (currently limited) knowledge of Git and GitHub.  I hope that this provides good coding practice for any future coding-related endeavours.

---

## Installation
Simply run
```
cd ${HOME} && git clone https://github.com/jakewilliami/tex-macros.git
```
If you don't have a TeX version, I ran the following
```
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)" && \
echo -e "\u001b[1;38;5;2mHomebrew installed successfully.\u001b[0;38m" && \
brew tap caskroom/cask && \
echo -e "\u001b[1;38;5;2mHomebrew cask installed successfully.\u001b[0;38m" && \
brew cask install mactex && \
echo -e "\u001b[1;38;5;2mTeXLive (TeX distribution) and MacTeX apps/tools installed successfully.\u001b[0;38m" && \
cd /tmp && \
curl --remote-name https://www.tug.org/fonts/getnonfreefonts/install-getnonfreefonts && \
sudo texlua install-getnonfreefonts && \
sudo getnonfreefonts --sys --all && \
cd ${HOME} && \
echo -e "\u001b[1;38;5;2mFont \`garamondx\` installed successfully.\u001b[0;38m"
```

On my Linux machine I ran
```
sudo pacman -S texmaker texlive-most && \
echo -e "\u001b[1;38;5;2mTeXLive (TeX distribution) and a LaTeX Editor installed successfully.\u001b[0;38m" && \
cd /tmp && \
curl --remote-name https://www.tug.org/fonts/getnonfreefonts/install-getnonfreefonts && \
sudo texlua install-getnonfreefonts && \
sudo getnonfreefonts --sys --all && \
cd ${HOME} && \
echo -e "\u001b[1;38;5;2mFont \`garamondx\` installed successfully.\u001b[0;38m"
```
A note on Linux: any mention of `/Users/` in the .tex templates will need to be changed to `/home/`.

## Usage
See templates.

Three main directories exist in this repository.  One for general macros in everyday use, with my favourite font (garamond; see above for how to obtain on a Mac).  One is an American Psychology Association format, in Times New Roman Font, and APA-style bibliography.  Finally, we have a directory for some templates using these.  Happy LaTeX-ing!

---

### Version History

LaTeX macros using a macOS Mojave version 10.14.6 

Macros written using TeXShop Version 4.27

9.9.2019

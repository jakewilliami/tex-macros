# LaTeX Macros

This repository will show the progression of my macros used on LaTeX, in order to **a)** keep track of history of my macros, and **b)** extend my (currently limited) knowledge of Git and GitHub.  I hope that this provides good coding practice for any future coding-related endeavours.

---

## Installation
Simply run
```
cd ${HOME} && \
git clone https://github.com/jakewilliami/tex-macros.git
```

I also downloaded [pdflatex.py](https://github.com/marcuswhybrow/texshop-pdflatex) for removal of auxilary files to a temporary directory, as well as [pythontex](https://github.com/gpoore/pythontex/) to integrate python into LaTeX.  To get the former, you can simply run the following in any desired directory (with the knowledge you will be downloading a binary file):
```
curl https://raw.githubusercontent.com/jakewilliami/scripts/master/bash/pytex > pytex && \
chmod u+x pytex && \
./pytex -h
```


## Usage
See `mktex -h`.

Two directories exist in this repository:

1) One directory for general macros for everyday use, with my favourite font (Garamond; see below for how to obtain on a Mac).  This directory has a sub-style sheet for bibliography (called `tea_hyperlinks_and_references_cite.sty`).  Ensure you have a `references.bib` in your working directory for this style sheet.  Note that, using TeXShop, I have added the line `% !TEX TS-program = pdflatex` at the top of the template.  We can also use `pdflatexmk` for our bibliography class.  Just as importantly, in this directory is the `class` subdirectory, which reads files from its parent to
2) We also have a directory for some templates using these, which `mktex` references, as well as some some `beamer` stuff in this directory.

Happy LaTeX-ing!

---

### Obtaining LaTeX and GaramondX

If you don't have a TeX version, I ran the following
```
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)" && \
echo -e "\u001b[1;38;5;2mHomebrew installed successfully.\u001b[0;38m" && \
brew install cask && \
echo -e "\u001b[1;38;5;2mHomebrew cask installed successfully.\u001b[0;38m" && \
brew cask install mactex && \
echo -e "\u001b[1;38;5;2mTeXLive (TeX distribution) and MacTeX apps/tools installed successfully.\u001b[0;38m" && \
chmod u+x mktex
```

On my Arch machine I ran
```
sudo pacman -S texmaker texlive-most && \
echo -e "\u001b[1;38;5;2mTeXLive (TeX distribution) and a LaTeX Editor installed successfully.\u001b[0;38m" && \
chmod u+x mktex
```

To get the font Garamond, I ran
```
cd /tmp && \
curl --remote-name https://www.tug.org/fonts/getnonfreefonts/install-getnonfreefonts && \
sudo texlua install-getnonfreefonts && \
sudo getnonfreefonts --sys --all && \
cd ${HOME} && \
echo -e "\u001b[1;38;5;2mFont \`garamondx\` installed successfully.\u001b[0;38m" && \
```

### Version History

LaTeX macros using a macOS Mojave version 10.14.6 

Macros written using TeXShop Version 4.27

~~9.9.2019~~ 13.3.2020

# LaTeX Macros

This repository will show the progression of my macros used on LaTeX, in order to **a)** keep track of history of my macros, and **b)** extend my (currently limited) knowledge of Git and GitHub.  I hope that this provides good coding practice for any future coding-related endeavours.


## Contents

  - [LaTeX Macros](#latex-macros)
  - [Contents](#contents)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Obtaining LaTeX, GaramondX, and PDFLaTeX.py](#obtaining-latex-garamondx-and-pdflatex.py)
    - [Installing LaTeX](#installing-latex)
	- [Installing Garamond](#installing-garamond)
	- [Installing PDFLaTeX.py](#installing-pdflatex.py)
  - [Plots, Flow Charts, Dot Graphs, and Figures](#plots-flow-charts-dot-graphs-and-figures)
  - [Version History](#version-history)

---

## Installation
Simply run
```bash
cd ${HOME} && \
git clone https://github.com/jakewilliami/tex-macros.git && \
export PATH=$PATH:${HOME}/tex-macros && \
for i in ${HOME}/tex-macros/*
do
  if [[ -f ${i} && -x ${i} ]]
  then
    chmod u+x ${i}
  fi
done
```

Now try it out!
```bash
chmod u+x ${HOME}/tex-macros/mktex && \
cd ${HOME}/Documents/ && \
mkdir test && \
mktex -c test
```

## Usage
See `mktex -h`.

Two directories exist in this repository:

1) One directory for general macros for everyday use, with my favourite font (Garamond; see below for how to obtain on a Mac).  This directory has a sub-style sheet for bibliography (called `tea_hyperlinks_and_references_cite.sty`).  Ensure you have a `references.bib` in your working directory for this style sheet.  Note that, using TeXShop, I have added the line `% !TEX TS-program = pdflatex` at the top of the template.  We can also use `pdflatexmk` for our bibliography class.  Just as importantly, in this directory is the `class` subdirectory, which reads files from its parent to
2) We also have a directory for some templates using these, which `mktex` references, as well as some some `beamer` stuff in this directory.

I have configured TeXShop: press `⌘` + `,` and click on the `Engine` tab.  In the `pdflatex` box, I have
```bash
/Library/TeX/Root/bin/pdflatex.py --file-line-error --synctex=1
```

Happy LaTeX-ing!

---

### Obtaining LaTeX, GaramondX, and PDFLaTeX.py

#### Installing LaTeX

If you don't have a TeX version, I ran the following
```bash
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)" && \
echo -e "\u001b[1;38;5;2mHomebrew installed successfully.\u001b[0;38m" && \
brew install cask && \
echo -e "\u001b[1;38;5;2mHomebrew cask installed successfully.\u001b[0;38m" && \
brew cask install mactex && \
echo -e "\u001b[1;38;5;2mTeXLive (TeX distribution) and MacTeX apps/tools installed successfully.\u001b[0;38m"
```

On my Arch machine I ran
```bash
sudo pacman -S texlive-most texmaker && \
echo -e "\u001b[1;38;5;2mTeXLive (TeX distribution) and a LaTeX Editor installed successfully.\u001b[0;38m"
```

On my Fedora machine I ran
```bash
sudo dnf install texlive-scheme-full texmaker && \
echo -e "\u001b[1;38;5;2mTeXLive (TeX distribution) and a LaTeX Editor installed successfully.\u001b[0;38m"
```

On my Debian machine I ran
```bash
sudo apt install texlive-full texmaker texmaker-data && \
echo -e "\u001b[1;38;5;2mTeXLive (TeX distribution) and a LaTeX Editor installed successfully.\u001b[0;38m"
```

#### Installing Garamond

To get the font Garamond on any Unix machine, I ran:
```bash
cd /tmp && \
curl --remote-name https://www.tug.org/fonts/getnonfreefonts/install-getnonfreefonts && \
sudo texlua install-getnonfreefonts && \
sudo getnonfreefonts --sys --force --all && \
cd - > /dev/null && \
echo -e "\u001b[1;38;5;2mFont \`garamondx\` installed successfully.\u001b[0;38m"
```
It should be noted that I have had issues with Garamondx on Arch particularly.  It works, and then I get the error `/usr/share/texmf-dist/tex/latex/filehook-scrlfile.sty Error; ! Package filehook Error: Detected 'scrlfile' package with unknown definition of \InputFileExists.  Use the 'force' option of 'filehook' to overwrite it.`  Here are some potential fixes:
- Simply reinstalling `garamondx` may work, though this is annoying.  
- It looks like this error is actually from the [`chemmacros`](https://tex.stackexchange.com/questions/512189/problem-with-chemmacros-beamer-and-filehook-scrlfile-sty) package; adding `\PassOptionsToPackage{force}{filehook}` to your preamble (**before adding everything else**).  
- Ensure you are running the correct version of TeXLive if all else fails.
I don't think this is a problem with Arch, I think this is a problem with my system. 

It should also be noted that since upgrading to BigSur, I had to download the file that `getnonfreefonts` uses and make `wget` ignore the certificate in order to download.

#### Installing PDFLaTeX.py

To get PDFLaTeX.py installed, run the following:
```bash
cd /Library/TeX/Root/bin/ && \
curl -s https://raw.githubusercontent.com/marcuswhybrow/texshop-pdflatex/master/pdflatex.py | sudo tee pdflatex.py > /dev/null && \
sudo chmod +x /Library/TeX/Root/bin/pdflatex.py && \
cd - > /dev/null && \
echo -e "\u001b[1;38;5;2mLaTeX compiler \`pdflatex.py\` installed successfully.\u001b[0;38m"
```

I also made a fork for PDFLaTeXmk.py to [allow BibTeX to work](https://github.com/marcuswhybrow/texshop-pdflatex/issues/1#issuecomment-645712757):
```bash
cd /Library/TeX/Root/bin/ && \
curl -s https://raw.githubusercontent.com/jakewilliami/texshop-pdflatexmk/master/pdflatexmk.py | sudo tee pdflatexmk.py > /dev/null && \
sudo chmod +x /Library/TeX/Root/bin/pdflatexmk.py && \
cd - > /dev/null && \
echo -e "\u001b[1;38;5;2mLaTeX compiler \`pdflatexmk.py\` installed successfully.\u001b[0;38m"
```

### Plots, Flow Charts, Dot Graphs, and Figures

 - **Flow Charts&mdash;**
 Flow charts are easy enough to make with TiKz.  We are okay here.  Some plots are also okay to make in TiKz; see examples.

 - **Dot Graphs&mdash;**
 For graph theory, I tend to use [GraphViz](https://www.graphviz.org/).  Of course, nothing can beat TiKz sometimes, but for help with transfering `.dot` files into TeX, run `mkgraph -h`.

 - **Plots&mdash;**
 Another option specific to plotting is to use [Python](https://www.python.org/).  For integration of Python into LaTeX, see [pythontex](https://github.com/gpoore/pythontex/), a tool that allows python code *within* a LaTeX document.  See also [tikzplotlib](https://github.com/nschloe/tikzplotlib) for transfering python graphs to TiKz.  If this is not producing great results, try exporting as pdf from [matplotlib](https://matplotlib.org/tutorials/text/pgf.html) directly.
 One such plot I have made a script for are [Slope Fields](https://www.wikiwand.com/en/Slope_field).  For installation of this script, run `mksfield -h`.  I also downloaded [pdflatex.py](https://github.com/marcuswhybrow/texshop-pdflatex) for removal of auxilary files to a temporary directory.  To get the former (pythontex), you can simply run the following in any desired directory: `pytex -h`.
 
 - **Figures&mdash;**
 Please see examples for some figures.
 
 
 TeX macros are currently being developed for dot-graphs, flow charts, plots, and general figures.  I find that TiKz is a steep learning curve, so have patience with me...

### Version History

LaTeX macros using a macOS Mojave version 10.14.6 

Macros written using TeXShop Version 4.27

~~9.9.2019~~ 13.3.2020

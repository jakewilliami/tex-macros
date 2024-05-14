# LaTeX Macros

This repository will show the progression of my macros used on LaTeX, in order to **a)** keep track of history of my macros, and **b)** extend my (currently limited) knowledge of Git and GitHub.  I hope that this provides good coding practice for any future coding-related endeavours.


TODO: redo this whole thing

## Contents

  - [Installation](#installation)
  - [Project Usage](#usage)
    - [Project Contents](#project-contents)
    - [Additional Installation](#additional-installation.py)
      - [Installing LaTeX](#installing-latex)
      - [Installing Garamond](#installing-garamond)
    - [Plots, Flow Charts, Dot Graphs, and Figures](#plots-flow-charts-dot-graphs-and-figures)
    - [Version History](#version-history)

---

## Installation

Clone the repository
```bash
$ git clone https://github.com/jakewilliami/tex-macros.git
```

We also recommend you clone the helper programme `mktex`:
```bash
$ git clone https://github.com/jakewilliami/mktex.git
```

## Project Usage

The templates have a [TeXShop](https://en.wikipedia.org/wiki/TeXShop) shebang at the top (typically telling TeXShip to use `pdflatexmk`.  We also suggest considering [`pdflatexmk.py`](https://github.com/jakewilliami/texshop-pdflatexmk) (Which [allows BibTeX to work](https://github.com/marcuswhybrow/texshop-pdflatex/issues/1#issuecomment-645712757)).  You can configure TeXShop to recognise this engine: press `⌘` + `,` and click on the `Engine` tab.  In the `pdflatexmk` box, I have
```bash
/Library/TeX/Root/bin/pdflatexmk.py --file-line-error --synctex=1
```

See also `mktex -h`.

Happy LaTeX-ing!

---

### Project Contents

We have the following subdirectories in the current project:
  - [`macros/`](./macros/): Primary personal macros (common patterns I don't want to rewrite so I keep them standardised in one place);
  - [`class/`](./class/): Custom classes (using personal macros);
  - [`templates/`](./templates/): High-level project templates.  Some of these are used by my [`mktex`](https://github.com/jakewilliami/mktex) script for classes;
  - [`examples/`](./examples/): Examples (mostly of figures) that can be easily copied and adapted.  Most of them have associated PDFs so that you don't have to compile them yourself.  I can't speak to their correctness as most of them probably compiled years ago;
  - [`archive/`](./archive/): Archive of files before The Big Refactor.

### Additional Installation

#### Installing LaTeX

Make sure you have LaTeX installed.  I use [MacTeX](https://en.wikipedia.org/wiki/MacTeX) from [Homebrew](https://en.wikipedia.org/wiki/Homebrew_(package_manager)).  On most Linux systems you can simply install TeXLive directly.  Consider also [TeXmaker](https://en.wikipedia.org/wiki/Texmaker) for editing and compiling.

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

It should also be noted that since upgrading to BigSur, I had to download the file that `getnonfreefonts` uses and make `wget` ignore the certificate in order to download.  However, before attempting to troubleshoot, try running the above script again, as I just did so after it failing and it worked...

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

% !TEX TS-program = pdflatex
\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{arteacle}[A LaTeX Article Class of Teagoslavia]

\LoadClass[11pt,a4paper]{article}
\ProcessOptions\relax

\input{\string~/tex-macros/general_macros/general_typesetting.sty}
\input{\string~/tex-macros/general_macros/hyperlinks_and_references.sty}
\input{\string~/tex-macros/general_macros/figures.sty}
\input{\string~/tex-macros/general_macros/tables.sty}
\input{\string~/tex-macros/general_macros/table_of_x.sty}
\input{\string~/tex-macros/general_macros/footnotes.sty}
\input{\string~/tex-macros/general_macros/lists.sty}
\input{\string~/tex-macros/general_macros/mathematical.sty}
\input{\string~/tex-macros/general_macros/chemical.sty}
\input{\string~/tex-macros/general_macros/miscellaneous.sty}


\makeatletter
\xpatchcmd{\@maketitle}
  {\@title}
  {\vspace*{-9ex}\fontsize{20}{28}\scshape\@title}
  {}{}
  \xpatchcmd{\@maketitle}
  {\@author}
  {\fontsize{10}{18}\scshape\small\bfseries\@author}
  {}{}
\makeatother


\addto\captionsenglish{\renewcommand*\abstractname{Abstract}}		%Renew abstract name

\AtBeginDocument{
	\maketitle
				}
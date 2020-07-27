\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{arteacle}[A LaTeX Article Class of Teagoslavia]
\LoadClass[11pt,a4paper]{article} % extend article class

% define bibapa boolean
\newif\ifbibapa
% set boolean to false by default
\bibapafalse
% define bib-apa option
\DeclareOption{bib-apa}{\bibapatrue}
% see https://tex.stackexchange.com/questions/524468/, https://www.reddit.com/r/LaTeX/comments/epftjv/, https://tex.stackexchange.com/a/394471/181375

% pass other options to class
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{article}} % pass options to 
\ProcessOptions\relax


\input{\string~/tex-macros/general_macros/general_typesetting.sty}

\ifbibapa
	\input{\string~/tex-macros/general_macros/hyperlinks_and_references_cite.sty} 
\else
	\input{\string~/tex-macros/general_macros/hyperlinks_and_references.sty} 
\fi

\input{\string~/tex-macros/general_macros/figures.sty}
\input{\string~/tex-macros/general_macros/tables.sty}
\input{\string~/tex-macros/general_macros/table_of_x.sty}
\input{\string~/tex-macros/general_macros/footnotes.sty}
\input{\string~/tex-macros/general_macros/lists.sty}
\input{\string~/tex-macros/general_macros/mathematical.sty}
\input{\string~/tex-macros/general_macros/code_format.sty}
\input{\string~/tex-macros/general_macros/chemical.sty}
\input{\string~/tex-macros/general_macros/miscellaneous.sty}


%%%%%% Put any custom features after this line! %%%%%%

%\texorpdfstring{\protect\ref{}}{}		%for when referencing in section titles
\addto\captionsenglish{
	\renewcommand*\abstractname{Abstract}		%Renew abstract name
}


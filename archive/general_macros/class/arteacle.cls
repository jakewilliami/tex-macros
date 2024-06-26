\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{arteacle}[The LaTeX Article Class of Teagoslavia]

% See the following resources for handling class options:
%   - https://tex.stackexchange.com/questions/524468/
%   - https://www.reddit.com/r/LaTeX/comments/epftjv/
%   - https://tex.stackexchange.com/a/394471/181375/
%   - https://stackoverflow.com/a/587054/12069968/
%   - https://tex.stackexchange.com/a/418947/181375/

% define bib-apa boolean
\newif\ifbibapa
% set boolean to false by default
\bibapafalse
% define bib-apa option
\DeclareOption{bib-apa}{\bibapatrue}

% Also process numeric option
\newif\ifbibnumeric
\bibnumericfalse
\DeclareOption{bib-apa=numeric}{\bibnumerictrue}
\DeclareOption{numeric}{
  \bibnumerictrue
  \ClassWarning{arteacle}{Use of numeric is deprecated in favour of bib-apa=numeric}
}

% Define font switches
% We have the following options:
%  - old-style-numbers
%  - Optionally old-style-numbers=math for OSF just in math mode
%  - osfI: TODO
%  - swishQ: TODO
%  - times-font
%  - double-spacing
\newif\ifoldstylenumbers
\oldstylenumbersfalse
\DeclareOption{old-style-numbers}{\oldstylenumberstrue}
\newif\ifoldstylenumbersmath
\oldstylenumbersmathfalse
\DeclareOption{old-style-numbers=math}{\oldstylenumbersmathtrue}
\newif\iftimesfont
\timesfontfalse
\DeclareOption{times-font}{\timesfonttrue}
\newif\ifdoublespacing
\doublespacingfalse
\DeclareOption{double-spacing}{\doublespacingtrue}

% Pass any remaining class options (unhandled by arteacle) to
% the underlying article class
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{article}}
\ProcessOptions\relax

% Extend the article class options
% By default we use 11pt font and a4 paper
\LoadClass[11pt, a4paper]{article}


% Process font options
\ifoldstylenumbers
    \PassOptionsToPackage{osf}{garamondx}
    \oldstylenumbersmathtrue
\fi
\ifoldstylenumbersmath
    \PassOptionsToPackage{garamondx}{newtxmath}
\fi
% TODO: get osfI option working with newtxmath (#10)
\iftimesfont
    % https://tex.stackexchange.com/a/153558
    % TODO: I can't write square brackets in maths mode with this package
    %\usepackage{mathptmx}
    % https://tex.stackexchange.com/a/95348
    % TODO: get timesfont working
    \usepackage{newtxtext, newtxmath}
\fi
\ifdoublespacing
    \usepackage{setspace}
    \setstretch{2}
\fi

% Input macro source files
\input{\string~/projects/tex-macros/general_macros/general_typesetting.sty}

% If bib-apa and numeric, we input cite numeric, otherwise input regular cite file
\ifbibnumeric\bibapatrue\fi % ensure bib-apa is set if numeric is set too
\ifbibapa
    \ifbibnumeric
	    \input{\string~/projects/tex-macros/general_macros/hyperlinks_and_references_cite_numeric.sty} 
    \else
		\input{\string~/projects/tex-macros/general_macros/hyperlinks_and_references_cite.sty} 
	\fi
\else
    \input{\string~/projects/tex-macros/general_macros/hyperlinks_and_references.sty} 
\fi

% Continue inputting macro source files
\input{\string~/projects/tex-macros/general_macros/figures.sty}
\input{\string~/projects/tex-macros/general_macros/tables.sty}
\input{\string~/projects/tex-macros/general_macros/table_of_x.sty}
\input{\string~/projects/tex-macros/general_macros/footnotes.sty}
\input{\string~/projects/tex-macros/general_macros/lists.sty}
\input{\string~/projects/tex-macros/general_macros/mathematical.sty}
\input{\string~/projects/tex-macros/general_macros/code_format.sty}
\input{\string~/projects/tex-macros/general_macros/chemical.sty}
\input{\string~/projects/tex-macros/general_macros/miscellaneous.sty}


%%%%%% Put any custom features after this line! %%%%%%

%\texorpdfstring{\protect\ref{}}{}		        % for when referencing in section titles
\addto\captionsenglish{
	\renewcommand*\abstractname{Abstract}		% Renew abstract name
}


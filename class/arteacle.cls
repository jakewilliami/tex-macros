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
\DeclareOption{numeric}{%
  \bibnumerictrue
  \ClassWarning{arteacle}{Use of numeric is deprecated as of 07d0ecbc in favour of bib-apa=numeric}
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
\newif\ifoldstylenumbersmath
\oldstylenumbersmathfalse
\newif\iftimesfont
\timesfontfalse
\newif\ifdoublespacing
\doublespacingfalse
\def\timesfonterror{%
  \ClassError{arteacle}%
             {Cannot use old-style-numbers with times-font, as old-style-numbers pertains to Garamond}%
             {If Times font is required, remove old-style-numbers}
}
\DeclareOption{old-style-numbers}{%
  \iftimesfont
    \timesfonterror
  \else
    \oldstylenumberstrue
  \fi
}
\DeclareOption{old-style-numbers=math}{%
  \iftimesfont
    \timesfonterror
  \else
    \oldstylenumbersmathtrue
  \fi
}
\DeclareOption{times-font}{%
  \ifoldstylenumbers
    \timesfonterror
  \elseifoldstylenumbersmath
    \timesfonterror
  \else
    \timesfonttrue
  \fi
}
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
\fi % end if oldstylenumbers
\ifoldstylenumbersmath
  % This option somehow requires newtxmath to be loaded but after other packages
  % It is hence loaded in mathematical.sty
  \PassOptionsToPackage{garamondx}{newtxmath}
\fi % end if oldstylenumbersmath
% TODO: get osfI option working with newtxmath (#10)
\iftimesfont
  % https://tex.stackexchange.com/a/153558
  \RequirePackage{mathptmx}
  % https://tex.stackexchange.com/a/95348
  % Note: newtxmath is loaded in mathematical.sty
  \RequirePackage{newtxtext}
\else
  % Default font is Garamond
  \RequirePackage{garamondx}
\fi % end if timesfont
\ifdoublespacing
  \RequirePackage{setspace}
  \setstretch{2}
\fi % end if doublespacing

% Input macro source files
\input{jakewilliami/tex-macros/macros/general.sty}
% Float might need to be loaded before hyperref
% https://tex.stackexchange.com/a/1868/
\input{jakewilliami/tex-macros/macros/figures.sty}
% Colours also required before hyperref setup
\input{jakewilliami/tex-macros/macros/colours.sty}

% If bib-apa and numeric, we input cite numeric, otherwise input regular cite file
\ifbibnumeric\bibapatrue\fi % ensure bib-apa is set if numeric is set too
\ifbibapa
  \ifbibnumeric
    \input{jakewilliami/tex-macros/macros/cite/numeric/reflink.sty}
  \else % else not bibnumeric
    \input{jakewilliami/tex-macros/macros/cite/reflink.sty}
  \fi % end if bibnumeric
\else % else not bibapa
  \input{jakewilliami/tex-macros/macros/reflink.sty}
\fi % end if bibapa

% Continue inputting macro source files
\input{jakewilliami/tex-macros/macros/tables.sty}
\input{jakewilliami/tex-macros/macros/footnotes.sty}
\input{jakewilliami/tex-macros/macros/mathematical.sty}
\input{jakewilliami/tex-macros/macros/units.sty}
\input{jakewilliami/tex-macros/macros/miscellaneous.sty}


%%%%%% Put any custom features after this line! %%%%%%

%\texorpdfstring{\protect\ref{}}{}		        % for when referencing in section titles
\addto\captionsenglish{
  \renewcommand*\abstractname{Abstract}		% Renew abstract name
}


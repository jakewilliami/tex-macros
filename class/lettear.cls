\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{lettear}[A LaTeX Letter Class of Teagoslavia]

% Pass any remaining class options (unhandled by arteacle) to
% the underlying letter class
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{letter}}
\ProcessOptions\relax

% Extend the letter class options
% By default we use 11pt font with a5 paper
\LoadClass[11pt, a5paper]{letter}

\input{jakewilliami/tex-macros/macros/letter/general.sty}
\input{jakewilliami/tex-macros/macros/figures.sty}
\input{jakewilliami/tex-macros/macros/colours.sty}
\input{jakewilliami/tex-macros/macros/tables.sty}
\input{jakewilliami/tex-macros/macros/footnotes.sty}
\input{jakewilliami/tex-macros/macros/mathematical.sty}
\input{jakewilliami/tex-macros/macros/units.sty}
\input{jakewilliami/tex-macros/macros/letter/signature.sty}
\input{jakewilliami/tex-macros/macros/miscellaneous.sty}


%%%%%% Put any custom features after this line! %%%%%%

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{teamer}[A LaTeX Beamer Class of Teagoslavia]

% Load required packages
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{booktabs, multirow}
\usepackage{sfmath}
\usepackage{xfrac}  % Nice fractions
% \usepackage{graphicx} % Load graphics
% \usepackage{showframe} % Useful for debugging

% See the following resources for handling class options:
%   - https://tex.stackexchange.com/questions/524468/
%   - https://www.reddit.com/r/LaTeX/comments/epftjv/
%   - https://tex.stackexchange.com/a/394471/181375/
%   - https://stackoverflow.com/a/587054/12069968/
%   - https://tex.stackexchange.com/a/418947/181375/

% Set up starting pause option
% With this option enabled, all frames will automatically
% inject a pause after the title.  This option also provides
% the `handoutframe` environment to override all pauses for
% specific frames, as well as the `nostartingpause` to override
% just the starting pause.
\newif\ifstartingpause
% set boolean to false by default
\startingpausefalse
\DeclareOption{startingpause}{\startingpausetrue}
% Also declare boolean for the handoutframe env
% https://tex.stackexchange.com/a/315757/181375
\newif\ifonhandoutframemodetransition

% Also set up BibLaTeX option
% With this option enabled, BibLaTeX will be loaded, and a
% bibliography pulled from references.bib
\newif\ifbiblatex
\biblatexfalse
\DeclareOption{biblatex}{\biblatextrue}

% Pass any remaining class options (unhandled by teamer) to
% the underlying article class
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{beamer}}
\ProcessOptions\relax

% Extend the article class
% We have no default options, but these would be added inside
% the square brackets
\LoadClass[]{beamer}


% Load theme
% https://tex.stackexchange.com/a/284157/181375
\makeatletter
  \def\beamer@calltheme#1#2#3{%
    \def\beamer@themelist{#2}
    \@for\beamer@themename:=\beamer@themelist\do
    {\usepackage[{#1}]{\beamer@themelocation/#3\beamer@themename}}}

  \def\usefolder#1{
    \def\beamer@themelocation{#1}
  }
  \def\beamer@themelocation{}

% State where to find theme and figures
\usefolder{teamer}
\usetheme{tea}
\graphicspath{{./figs/}}

% Handle biblatex option
\ifbiblatex
	\usepackage[bibencoding=utf8, style=apa, backend=biber]{biblatex}
	\addbibresource{references.bib}
\fi

% Handle starting pause option
% TODO: fix this option with allowframebreaks
\ifstartingpause
	% Inject starting pause after frame titles
	% Note: we do not do this if we are using handout mode
	% https://tex.stackexchange.com/a/563680/181375
	% https://tex.stackexchange.com/a/54727/181375
	\def\pauseatstart{\pause}
	% Do not inject pause if in nostartingpause environment
	\newenvironment{nostartingpause}{\def\pauseatstart{}}{\let\pauseatstart=\pause}
	\mode<beamer>{%
		% Inject starting pause
		\makeatletter
		\def\beamer@checkframetitle{%
			\@ifnextchar\bgroup\beamer@inlineframetitle{\pauseatstart}%
		}%
		\def\beamer@inlineframetitle#1{%
			\@ifnextchar\bgroup{\frametitle{#1}\framesubtitle}{\frametitle{#1}\relax}%
			\pauseatstart%
		}%
		\makeatother
	}

	% Add `handoutframe` environment to selectively 
	% bypass the starting pause
	% https://tex.stackexchange.com/a/356907/181375
	\makeatletter
	\newenvironment{handoutframe}{%
		\begingroup%
		\mode<beamer>{%
			\gdef\beamer@currentmode{handout}%
			\onhandoutframemodetransitiontrue%
		}%
	}{%
		\mode<handout>{%
			\ifonhandoutframemodetransition%
				\onhandoutframemodetransitionfalse
				\gdef\beamer@currentmode{beamer}%
			\fi
		}%
		\endgroup%
	}%
	\makeatother
\fi

% The all-important em dash!
\def\emdash
	{\discretionary{}{}{\kern 0.1em}---\discretionary{}{}{\kern 0.1em}}

% Automatically add pauses to lists
% https://stackoverflow.com/a/65815307/12069968
\beamerdefaultoverlayspecification{<+->}

% Convenience command to print the title page
\def\titleframe{%
	\ifstartingpause%
		\begin{handoutframe}%
	\fi%
	\begin{frame}%
		\titlepage%
	\end{frame}%
	\ifstartingpause%
		\end{handoutframe}%
	\fi%
}

% Convenience command for printing bibliography
\ifbiblatex
	\def\bibliographyframe{%
		\ifstartingpause%
			\begin{handoutframe}%
		\fi%
		\begin{frame}[allowframebreaks]{References}%
			\printbibliography%
		\end{frame}%
		\ifstartingpause%
			\end{handoutframe}%
		\fi%
	}
\fi


%%%%%% Put any custom features after this line! %%%%%%


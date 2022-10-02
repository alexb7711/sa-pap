;;; build-paper.el --- Build all the org files into LaTeX documents.  -*- lexical-binding: t; -*-

;; Copyright (C) 2022  Alexander Brown

;; Author: Alexander Brown <alex.brown7711@gmail.com>

;; Load the publishing project
(require 'ox-publish)

;; Create new `article' class for `org-latex-classes'
(with-eval-after-load 'ox-latex
(add-to-list 'org-latex-classes
             '("article"
               "\\documentclass[11pt,a4paper,final]{article}
\\usepackage[a4paper, total={7in, 10in}]{geometry}
\\usepackage{algorithm2e}
\\usepackage{booktabs}
\\usepackage{hyperref}
\\usepackage{subcaption}
\\usepackage{graphicx}
\\usepackage{tikz}"
              ("\\section{%s}" . "\\section*{%s}")
              ("\\subsection{%s}" . "\\subsection*{%s}")
              ("\\subsubsection{%s}" . "\\subsubsection*{%s}")
              ("\\paragraph{%s}" . "\\paragraph*{%s}")
              ("\\subparagraph{%s}" . "\\subparagraph*{%s}"))))


;; Define the publishing project
(setq org-publish-project-alist
      (list                                                                     ; List for all projects
       (list "papers"                                                           ; List for `notes' project
             :author               "Alexander Brown"
             :publishing-directory "./"
             :base-directory       "./"
             :recursive            nil
             :with-author          t
             :with-toc             t
             :section-numbers      t
             :time-stamp-file      nil
             :publishing-function  'org-latex-publish-to-pdf)))

(setq
 org-export-headline-levels   5                                                 ; Max level that `org' will export a
                                                                                ; header to LaTeX
 org-latex-prefer-user-labels 1)                                                ; Use user labels, not gereated ones

;; Generate the site output
(org-publish-all t)

(message "Build complete!")

;;; build-paper.el ends here.

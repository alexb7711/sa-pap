;;; build-paper.el --- Build all the org files into LaTeX documents.  -*- lexical-binding: t; -*-

;; Copyright (C) 2022  Alexander Brown

;; Author: Alexander Brown <alex.brown7711@gmail.com>

;; Load the publishing project
(require 'ox-publish)

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

;; Generate the site output
(org-publish-all t)

(message "Build complete!")

;;; build-paper.el ends here.

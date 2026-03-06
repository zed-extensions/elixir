; Directive expressions can use the following tag delimiters:
;     `<%` and `%>`
;     `<%=` and `%>`
;     `<%%` and `%>`
;     `<%%=` and `%>`
(directive) @keyword

; Comments can use the following tag delimiters:
;     `<%!--` and `--%>`
;     `<%#` and `%>`
[
  (comment
    "<%!--"
    "--%>")
  (comment
    "<%#"
    "%>")
] @comment

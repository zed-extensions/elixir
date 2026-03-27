; Comments
(comment) @comment.inclusive

; Strings
[
  (string)
  (charlist)
] @string

; Quoted atoms
[
  (quoted_atom)
  (quoted_keyword)
] @quoted_atom

; Identifiers, atoms, and module attributes
[
  (identifier)
  (atom)
  (keyword)
  (unary_operator
    operator: "@")
] @identifier

; Chars (e.g. `?\n`, `?\s`)
(char) @char

; Parameter placeholders when capturing anonymous functions
(unary_operator
  operator: "&"
  operand: (integer)) @capture

; Sigils using curly bracket delimiters
(sigil
  quoted_start: "{"
  quoted_end: "}") @sigil_curly

; Sigils using square bracket delimiters
(sigil
  quoted_start: "["
  quoted_end: "]") @sigil_square

; Sigils using round bracket delimiters
(sigil
  quoted_start: "("
  quoted_end: ")") @sigil_round

; Sigils using quote mark delimiters
[
  (sigil
    quoted_start: "\"\"\""
    quoted_end: "\"\"\"")
  (sigil
    quoted_start: "\""
    quoted_end: "\"")
  (sigil
    quoted_start: "'''"
    quoted_end: "'''")
  (sigil
    quoted_start: "'"
    quoted_end: "'")
] @sigil_quote

; HEEx templates
(sigil
  (sigil_name) @_sigil_name
  (#eq? @_sigil_name "H")) @heex

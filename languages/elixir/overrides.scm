(comment) @comment.inclusive

[
  (string)
  (charlist)
] @string

[
  (quoted_atom)
  (quoted_keyword)
] @quoted_atom

[
  (identifier)
  (atom)
  (keyword)
  (unary_operator
    operator: "@")
] @identifier

(char) @char

(unary_operator
  operator: "&"
  operand: (integer)) @capture

(sigil
  quoted_start: "{"
  quoted_end: "}") @sigil_curly

(sigil
  quoted_start: "["
  quoted_end: "]") @sigil_square

(sigil
  quoted_start: "("
  quoted_end: ")") @sigil_round

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
] @sigil_string

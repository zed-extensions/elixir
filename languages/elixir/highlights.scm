; Reserved keywords
[
  "when"
  "and"
  "or"
  "not"
  "in"
  "not in"
  "fn"
  "do"
  "end"
  "catch"
  "rescue"
  "after"
  "else"
] @keyword

; Capture operand
(unary_operator
  operator: "&"
  operand: (integer) @operator)

; Operand identifiers
(operator_identifier) @operator

; Unary operands (`@/1, `+/1`, `-/1`, `!/1`, `^/1`, `not/1`, `&/1`, `.../1`)
(unary_operator
  operator: _ @operator)

; Binary operands (e.g. `+/2`, `++/2`, `<>/2`, `in/2`)
(binary_operator
  operator: _ @operator)

; Dot operand (`.`)
(dot
  operator: _ @operator)

; Stab operand (`->`)
(stab_clause
  operator: _ @operator)

; Special atom literals
[
  (boolean)
  (nil)
] @constant

; Number literals
[
  (integer)
  (float)
] @number

; Modules
(alias) @type

; Erlang modules
(call
  target: (dot
    left: (atom) @type))

; Char literals (e.g. `?a`, `?{`)
(char) @constant

; Escape characters (e.g. `\s`, `\n`)
(escape_sequence) @string.escape

; Atom literals
[
  (atom)
  (quoted_atom)
  (keyword)
  (quoted_keyword)
] @string.special.symbol

; String literals
[
  (string)
  (charlist)
] @string

; String sigils
(sigil
  (sigil_name) @__name__
  quoted_start: _ @string
  quoted_end: _ @string
  (#any-of? @__name__ "s" "S")) @string

; Regex sigils
(sigil
  (sigil_name) @__name__
  quoted_start: _ @string.regex
  quoted_end: _ @string.regex
  (#any-of? @__name__ "r" "R")) @string.regex

; Sigils
(sigil
  (sigil_name) @__name__
  quoted_start: _ @string.special
  quoted_end: _ @string.special) @string.special

; Regular identifiers
(identifier) @variable

; Unused identifiers
((identifier) @comment.unused
  (#match? @comment.unused "^_"))

; Function/macro calls
(call
  target: [
    (identifier) @function
    (dot
      right: (identifier) @function)
  ])

; Function/macro definitions
(call
  target: (identifier) @keyword
  (arguments
    [
      (identifier) @function
      (binary_operator
        left: (identifier) @function
        operator: "when")
      (binary_operator
        operator: "|>"
        right: (identifier))
    ])
  (#any-of? @keyword
    "def"
    "defp"
    "defdelegate"
    "defguard"
    "defguardp"
    "defmacro"
    "defmacrop"
    "defn"
    "defnp"))

; Function piping
(binary_operator
  operator: "|>"
  right: (identifier) @function)

; Definition keywords
(call
  target: (identifier) @keyword
  (#any-of? @keyword
    "def"
    "defp"
    "defdelegate"
    "defoverridable"
    "defguard"
    "defguardp"
    "defmacro"
    "defmacrop"
    "defstruct"
    "defexception"
    "defmodule"
    "defprotocol"
    "defimpl"
    "defn"
    "defnp"))

; Kernel/special form keywords
(call
  target: (identifier) @keyword
  (#any-of? @keyword
    "alias"
    "case"
    "cond"
    "else"
    "for"
    "if"
    "import"
    "quote"
    "raise"
    "receive"
    "require"
    "reraise"
    "super"
    "throw"
    "try"
    "unless"
    "unquote"
    "unquote_splicing"
    "use"
    "with"))

; Special identifiers
((identifier) @constant.builtin
  (#any-of? @constant.builtin
    "__MODULE__"
    "__DIR__"
    "__ENV__"
    "__CALLER__"
    "__STACKTRACE__"))

; Documentation attributes
(unary_operator
  operator: "@" @comment.doc
  operand: (call
    target: (identifier) @__attribute__ @comment.doc
    (arguments
      [
        (string)
        (charlist)
        (sigil)
        (boolean)
      ] @comment.doc))
  (#any-of? @__attribute__
    "moduledoc"
    "typedoc"
    "doc"))

; Comments
(comment) @comment

; Punctuations
[
 "%"
] @punctuation

; Delimiters
[
 ","
 ";"
] @punctuation.delimiter

; Brackets
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "<<"
  ">>"
] @punctuation.bracket

; String interpolations
(interpolation "#{" @punctuation.special "}" @punctuation.special) @embedded

; HEEx sigil
(sigil
  (sigil_name) @__name__
  (quoted_content) @embedded
  (#eq? @__name__ "H"))

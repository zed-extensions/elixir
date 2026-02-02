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

; Comments
(comment) @comment

; String interpolations
(interpolation "#{" @punctuation.special "}" @punctuation.special) @embedded

; Escape characters (e.g. `\s`, `\n`)
(escape_sequence) @string.escape

; Atom literals
[
  (atom)
  (quoted_atom)
  (keyword)
  (quoted_keyword)
] @string.special.symbol

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

; String literals
[
  (string)
  (charlist)
] @string

; Char literals (e.g. `?a`, `?{`)
(char) @constant

; Modules
(alias) @type

; Erlang modules
(call
  target: (dot
    left: [
      (atom)
      (quoted_atom)
    ] @type))

; Regular identifiers
(identifier) @variable

; Unused identifiers
((identifier) @comment.unused
  (#match? @comment.unused "^_"))

; Special identifiers
((identifier) @constant.builtin
  (#any-of? @constant.builtin
    "__MODULE__"
    "__DIR__"
    "__ENV__"
    "__CALLER__"
    "__STACKTRACE__"))

; Operator identifiers
(operator_identifier) @operator

; Unary operators (`@/1, `+/1`, `-/1`, `!/1`, `^/1`, `not/1`, `&/1`, `.../1`)
(unary_operator
  operator: _ @operator)

; Binary operators (e.g. `+/2`, `++/2`, `<>/2`, `in/2`)
(binary_operator
  operator: _ @operator)

; Dot operator (`.`)
(dot
  operator: _ @operator)

; Stab clause operator (`->`)
(stab_clause
  operator: _ @operator)

; Sigils
(sigil
  (sigil_name) @string.special
  quoted_start: _ @string.special
  quoted_end: _ @string.special) @string.special

; String/charlist sigils
(sigil
  (sigil_name) @_sigil_name @string
  quoted_start: _ @string
  quoted_end: _ @string
  (#any-of? @_sigil_name "C" "c" "S" "s")) @string

; Regex sigils
(sigil
  (sigil_name) @_sigil_name @string.regex
  quoted_start: _ @string.regex
  quoted_end: _ @string.regex
  (#any-of? @_sigil_name "R" "r")) @string.regex

; HEEx sigil
(sigil
  (sigil_name) @string.special
  (quoted_content) @embedded
  (#eq? @string.special "H"))

; Function/macro calls (with parentheses)
(call
  target: [
    (identifier) @function
    (dot right: (identifier) @function)
  ])

; Map dot field access
(call
  target: (dot
    right: [
      (identifier)
      (string)
    ] @property)
  .)

; Remote function/macro calls without parentheses
(call
  target: (dot
    left: [
      (alias)
      (atom)
      (quoted_atom)
    ]
    right: (identifier) @function)
  .)

; Parameter placeholders when capturing anonymous functions
(unary_operator
  operator: "&" @variable.parameter
  operand: (integer) @variable.parameter)

; Capture local functions
(unary_operator
  operator: "&"
  operand: [
    (identifier) @function
    (binary_operator
      left: (identifier) @function
      operator: "/")
  ])

; Capture remote Erlang functions
(unary_operator
  operator: "&"
  operand: [
    (atom) @type
    (quoted_atom) @type
  ])

; Capture functions from a map field/variable holding a module
(unary_operator
  operator: "&"
  operand: [
    (call
      target: (dot
        right: (identifier) @function))
    (binary_operator
      left: (call
        target: (dot
          right: (identifier) @function))
      operator: "/")
  ])

; Piping into a local function/macro that has no parentheses
(binary_operator
  operator: "|>"
  right: (identifier) @function)

; Piping into a map field/variable holding a module that has no parentheses
(binary_operator
  operator: "|>"
  right: (call
    target: (dot
      right: (identifier) @function)))

; Function/macro definitions
(call
  target: (identifier) @keyword
  (arguments
    [
      (identifier) @function
      (binary_operator
        left: (identifier) @function
        operator: "when")
      ; Targets the function definition for piping in the Kernel module
      (binary_operator
        operator: "|>"
        right: (identifier) @variable)
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
    "defnp"
    "deftransform"
    "deftransformp"))

; Module attributes
(unary_operator
  operator: "@" @attribute
  operand: [
    (identifier) @attribute
    (call
      target: (identifier) @attribute)
    (boolean) @attribute
    (nil) @attribute
  ])

; Doc attributes
(unary_operator
  operator: "@" @comment.doc
  operand: [
    (identifier) @_identifier @comment.doc
    (call
      target: (identifier) @_identifier @comment.doc)
    ]
  (#any-of? @_identifier
    "deprecated"
    "moduledoc"
    "typedoc"
    "shortdoc"
    "doc"))

; Typespec attributes
(unary_operator
  operator: "@" @enum
  operand: [
    (identifier) @_identifier @enum
    (call
      target: (identifier) @_identifier @enum)
    ]
  (#any-of? @_identifier
    "type"
    "typep"
    "opaque"
    "spec"
    "callback"
    "macrocallback"))

; Doc attribute arguments
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @_identifier @comment.doc
    (arguments
      [
        (string) @comment.doc
        (charlist) @comment.doc
        (boolean) @comment.doc
        (sigil
          (sigil_name) @_sigil_name @comment.doc
          quoted_start: _ @comment.doc
          quoted_end: _ @comment.doc
          (#any-of? @_sigil_name "C" "c" "S" "s"))
      ] @comment.doc))
  (#any-of? @_identifier
    "deprecated"
    "moduledoc"
    "typedoc"
    "shortdoc"
    "doc"))

; Typespec attribute arguments
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @enum
    (arguments
      [
        (identifier) @function
        (binary_operator
          left: (identifier) @function)
        (binary_operator
          left: (binary_operator
            left: (identifier) @function)
          operator: "when")
      ]))
  (#any-of? @enum
    "type"
    "typep"
    "opaque"
    "spec"
    "callback"
    "macrocallback"))

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
    "defrecord"
    "defrecordp"
    "defmodule"
    "defprotocol"
    "defimpl"
    "defn"
    "defnp"
    "deftransform"
    "deftransformp"))

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

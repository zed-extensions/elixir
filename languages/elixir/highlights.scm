; Punctuations
"%" @punctuation

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
(interpolation
  "#{" @punctuation.special
  "}" @punctuation.special) @embedded

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
(char) @constant.char

; Modules
(alias) @type.module

; Erlang modules
(call
  target: (dot
    left: [
      (atom)
      (quoted_atom)
    ] @type.module))

; Regular identifiers
(identifier) @variable

; Unused identifiers
((identifier) @comment.unused
  (#match? @comment.unused "^_"))

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

; Sigil modifiers
(sigil_modifiers) @label.modifier

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
  (sigil_modifiers)? @keyword.operator.regex
  (#any-of? @_sigil_name "R" "r")) @string.regex

; Phoenix HEEx template sigil
(sigil
  (sigil_name) @string.special
  (quoted_content) @embedded
  (#eq? @string.special "H"))

; Function/macro calls (with parentheses)
(call
  target: [
    (identifier) @function
    ((identifier) @comment.unused
      (#match? @comment.unused "^_"))
    (dot
      right: [
        (identifier) @function
        ((identifier) @comment.unused
          (#match? @comment.unused "^_"))
      ])
  ])

; Map dot field access
(call
  target: (dot
    right: [
      (identifier)
      (string)
    ] @property) .)

; Remote function/macro calls without parentheses
(call
  target: (dot
    left: [
      (alias)
      (atom)
      (quoted_atom)
    ]
    right: [
      (identifier) @function
      ((identifier) @comment.unused
        (#match? @comment.unused "^_"))
    ]) .)

; Parameter placeholders when capturing anonymous functions
(unary_operator
  operator: "&" @variable.parameter
  operand: (integer) @variable.parameter)

; Capture local functions
(unary_operator
  operator: "&"
  operand: [
    (identifier) @function
    ((identifier) @comment.unused
      (#match? @comment.unused "^_"))
    (binary_operator
      left: [
        (identifier) @function
        ((identifier) @comment.unused
          (#match? @comment.unused "^_"))
      ]
      operator: "/")
  ])

; Capture remote Erlang functions
(unary_operator
  operator: "&"
  operand: [
    (atom)
    (quoted_atom)
  ] @type.module)

; Capture functions from a map field/variable holding a module
(unary_operator
  operator: "&"
  operand: [
    (call
      target: (dot
        right: [
          (identifier) @function
          ((identifier) @comment.unused
            (#match? @comment.unused "^_"))
        ]))
    (binary_operator
      left: (call
        target: (dot
          right: [
            (identifier) @function
            ((identifier) @comment.unused
              (#match? @comment.unused "^_"))
          ]))
      operator: "/")
  ])

; Piping into a local function/macro that has no parentheses
(binary_operator
  operator: "|>"
  right: [
    (identifier) @function
    ((identifier) @comment.unused
      (#match? @comment.unused "^_"))
  ])

; Piping into a remote Erlang function
(binary_operator
  operator: "|>"
  right: [
    (atom)
    (quoted_atom)
  ] @type.module)

; Piping into a map field/variable holding a module that has no parentheses
(binary_operator
  operator: "|>"
  right: (call
    target: (dot
      right: [
        (identifier) @function
        ((identifier) @comment.unused
          (#match? @comment.unused "^_"))
      ])))

; Function/macro definitions
(call
  target: (identifier) @keyword.definition
  (arguments
    [
      (identifier) @function
      ((identifier) @comment.unused
        (#match? @comment.unused "^_"))
      (binary_operator
        left: [
          (identifier) @function
          ((identifier) @comment.unused
            (#match? @comment.unused "^_"))
        ]
        operator: "when")
      ; Targets the function definition for piping in the Kernel module
      (binary_operator
        operator: "|>"
        right: (identifier) @variable)
    ])
  (#any-of? @keyword.definition
    "def" "defp" "defdelegate" "defguard" "defguardp" "defmacro" "defmacrop" "defn" "defnp"
    "deftransform" "deftransformp"))

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
  (#any-of? @_identifier "deprecated" "moduledoc" "typedoc" "shortdoc" "doc"))

; Typespec attributes
(unary_operator
  operator: "@" @type.spec
  operand: [
    (identifier) @_identifier @type.spec
    (call
      target: (identifier) @_identifier @type.spec)
  ]
  (#any-of? @_identifier "type" "typep" "opaque" "spec" "callback" "macrocallback"))

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
  (#any-of? @_identifier "deprecated" "moduledoc" "typedoc" "shortdoc" "doc"))

; Typespec attribute arguments
(unary_operator
  operator: "@"
  operand: (call
    target: (identifier) @type.spec
    (arguments
      [
        (identifier) @function
        ((identifier) @comment.unused
          (#match? @comment.unused "^_"))
        (binary_operator
          left: [
            (identifier) @function
            ((identifier) @comment.unused
              (#match? @comment.unused "^_"))
          ])
        (binary_operator
          left: (binary_operator
            left: [
              (identifier) @function
              ((identifier) @comment.unused
                (#match? @comment.unused "^_"))
            ])
          operator: "when")
      ]))
  (#any-of? @type.spec "type" "typep" "opaque" "spec" "callback" "macrocallback"))

; Special identifiers
((identifier) @constant.builtin
  (#any-of? @constant.builtin "__MODULE__" "__DIR__" "__ENV__" "__CALLER__" "__STACKTRACE__"))

; Definition keywords
(call
  target: (identifier) @keyword.definition
  (#any-of? @keyword.definition
    "def" "defp" "defdelegate" "defoverridable" "defguard" "defguardp" "defmacro" "defmacrop"
    "defstruct" "defexception" "defrecord" "defrecordp" "defmodule" "defprotocol" "defimpl" "defn"
    "defnp" "deftransform" "deftransformp"))

; Reserved definition keyword
"fn" @keyword.definition

; Module import keywords
(call
  target: (identifier) @keyword.import
  (#any-of? @keyword.import "alias" "import" "require" "use"))

; Control flow keywords
(call
  target: (identifier) @keyword.control.conditional
  (#any-of? @keyword.control.conditional "case" "cond" "if" "receive" "unless" "with"))

; Reserved control flow keywords
[
  "after"
  "else"
] @keyword.control.conditional

; List comprehension keyword
(call
  target: (identifier) @keyword.control.repeat
  (#eq? @keyword.control.repeat "for"))

; Exception handling keywords
(call
  target: (identifier) @keyword.exception
  (#any-of? @keyword.exception "raise" "reraise" "throw" "try"))

; Reserved exception handling keywords
[
  "catch"
  "rescue"
] @keyword.exception

; Metaprogramming keywords
(call
  target: (identifier) @keyword
  (#any-of? @keyword "quote" "super" "unquote" "unquote_splicing"))

; Operator keywords
[
  "and"
  "in"
  "not"
  "not in"
  "or"
  "when"
] @keyword.operator

; Do-end block keywords
[
  "do"
  "end"
] @keyword

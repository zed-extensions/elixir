; Module/protocol definitions
(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "defmodule" "defprotocol" "defimpl"))
  (do_block
    "do"
    (_)* @class.inside
    "end")) @class.around

; Anonymous function definitions
(anonymous_function
  (stab_clause
    right: (body) @function.inside)) @function.around

; Function definitions with `do_block` bodies
(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier
      "def"
      "defp"
      "defmacro"
      "defmacrop"
      "defn"
      "defnp"))
  (do_block
    "do"
    (_)* @function.inside
    "end")) @function.around

; Function definitions with `keywords` bodies
(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier
      "def"
      "defp"
      "defmacro"
      "defmacrop"
      "defn"
      "defnp"))
  (arguments
    (_)
    (keywords
      (pair
        value: (_) @function.inside)))) @function.around

(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "defdelegate" "defguard" "defguardp"))) @function.around

; Comment definitions
(comment) @comment.around

; Documentation definitions
(unary_operator
  operator: "@"
  operand: (call
    target: ((identifier) @_identifier
      (#any-of? @_identifier "moduledoc" "typedoc" "shortdoc" "doc"))
    (arguments
      [
        (keywords) @comment.inside
        (string
          (quoted_content) @comment.inside)
      ]))) @comment.around

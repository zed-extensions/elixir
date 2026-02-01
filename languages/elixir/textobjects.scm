; Module/protocol definitions
(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "defmodule" "defprotocol" "defimpl"))
  (do_block "do" (_)* @class.inside "end")) @class.around

; Anonymous function definitions
(anonymous_function
  (stab_clause right: (body) @function.inside)+) @function.around

; Function definitions
(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier
      "def"
      "defp"
      "defmacro"
      "defmacrop"
      "defn"
      "defnp"
      "deftransform"
      "deftransformp"))
  (arguments
    (_)
    (keywords
      (pair
        key: ((keyword) @_keyword (#eq? @_keyword "do: "))
        value: (_) @function.inside))?)?
  (do_block "do" (_)* @function.inside "end")?) @function.around

; Function definitions from delegations
(call
  target: ((identifier) @_identifier
    (#eq? @_identifier "defdelegate"))
  (arguments
    (_)
    (keywords (pair)* @function.inside)?)?) @function.around

; Guard definitions
(call
  target: ((identifier) @_identifier
    (#any-of? @_identifier "defguard" "defguardp"))
  (arguments
    (binary_operator
      operator: "when"
      right: (_) @function.inside)?)) @function.around

; Comment definitions
((comment)+ @comment.around) @comment.inside

; Documentation definitions
(unary_operator
  operator: "@"
  operand: (call
    target: ((identifier) @_identifier
      (#any-of? @_identifier
        "deprecated"
        "moduledoc"
        "typedoc"
        "shortdoc"
        "doc"))
    (arguments
      [
        (string (quoted_content) @comment.inside)
        (charlist (quoted_content) @comment.inside)
        (sigil (quoted_content) @comment.inside)
        (keywords (pair)* @comment.inside)
        (_) @comment.inside
      ]))) @comment.around

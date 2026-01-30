; Module/protocol definitions
(call
  target: (identifier) @context
  (arguments (alias) @name)
  (#any-of? @context "defmodule" "defprotocol")) @item

; ExUnit setups
(call
  target: (identifier) @context
  (arguments (_) @name)?
  (#any-of? @context "setup" "setup_all")) @item

; ExUnit tests
(call
  target: (identifier) @context
  (arguments (string) @name)
  (#any-of? @context "describe" "test")) @item

; Typespec attributes
(unary_operator
  operator: "@" @name
  operand: (call
  target: (identifier) @context
    (arguments
      [
        (binary_operator
          left: (identifier) @name)
        (binary_operator
          left: (call
          target: (identifier) @name
          (arguments
            "(" @context.extra
            _* @context.extra
            ")" @context.extra)))
      ]))
  (#any-of? @context "type" "typep" "callback")) @item

; Function/macro definitions
(call
  target: (identifier) @context
  (arguments
    [
      (identifier) @name
      (call
          target: (identifier) @name
          (arguments
              "(" @context.extra
              _* @context.extra
              ")" @context.extra))
      (binary_operator
        left: (call
            target: (identifier) @name
            (arguments
                "(" @context.extra
                _* @context.extra
                ")" @context.extra))
        operator: "when")
    ])
  (#any-of? @context
    "def"
    "defp"
    "defdelegate"
    "defguard"
    "defguardp"
    "defmacro"
    "defmacrop"
    "defn"
    "defnp")) @item

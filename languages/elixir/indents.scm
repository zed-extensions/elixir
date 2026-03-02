[
  (tuple
    "}" @end)
  (map
    "}" @end)
  (list
    "]" @end)
  (access_call
    "]" @end)
  (block
    ")" @end)
  (arguments
    ")" @end)
  (unary_operator
    operator: "&"
    ")" @end)
  (bitstring
    ">>" @end)
  (do_block
    "end" @end)
  (anonymous_function
    "end" @end)
] @indent

; These have no end delimiter that can be captured,
; as they are usually nested inside `do_block` nodes
[
  (after_block)
  (catch_block)
  (else_block)
  (rescue_block)
  (stab_clause)
] @indent

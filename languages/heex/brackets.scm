(expression
  "{" @open
  "}" @close)

(directive
  _ @open
  [
    (expression_value)
    (partial_expression_value)
    (ending_expression_value)
  ]
  _ @close)

(start_tag
  "<" @open
  ">" @close
  (#set! rainbow.exclude))

(end_tag
  "</" @open
  ">" @close
  (#set! rainbow.exclude))

(self_closing_tag
  "<" @open
  "/>" @close
  (#set! rainbow.exclude))

(tag
  (start_tag) @open
  (end_tag) @close
  (#set! newline.only)
  (#set! rainbow.exclude))

(start_slot
  "<:" @open
  ">" @close
  (#set! rainbow.exclude))

(end_slot
  "</:" @open
  ">" @close
  (#set! rainbow.exclude))

(self_closing_slot
  "<:" @open
  "/>" @close
  (#set! rainbow.exclude))

(slot
  (start_slot) @open
  (end_slot) @close
  (#set! newline.only)
  (#set! rainbow.exclude))

(start_component
  "<" @open
  ">" @close
  (#set! rainbow.exclude))

(end_component
  "</" @open
  ">" @close
  (#set! rainbow.exclude))

(self_closing_component
  "<" @open
  "/>" @close
  (#set! rainbow.exclude))

(component
  (start_component) @open
  (end_component) @close
  (#set! newline.only)
  (#set! rainbow.exclude))

(quoted_attribute_value
  _ @open
  (attribute_value)
  _ @close
  (#set! rainbow.exclude))

; Directives are standalone tags like `<%= @x %>`
;
; Directives with `partial_expression_value` or `ending_expression_value`
; nodes are Elixir code that is part of an expression that spans multiple
; `directive` nodes, so they must be combined. For example:
;     <%= if true do %>
;       Hello, tree-sitter!
;     <% end %>
(directive
  [
    (partial_expression_value)
    (ending_expression_value)
  ] @injection.content
  (#set! injection.language "elixir")
  (#set! injection.combined))

; Directives with `expression_value` nodes do not need to be combined.
; For example: `<%= @content %>`
(directive
  (expression_value) @injection.content
  (#set! injection.language "elixir"))

; Comment parsing languages support
((comment
  "<%!--"
  "--%>") @injection.content
  (#set! injection.language "comment"))

((comment
  "<%#"
  "%>") @injection.content
  (#set! injection.language "comment"))

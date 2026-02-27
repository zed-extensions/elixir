; Expressions are HTML-aware interpolations of Elixir code like
; `<link href={ Routes.static_path(..) } />`
;
; Note that we include children because `expression_value` nodes may consist
; of multiple nodes when the value contains `{` and `}`
(expression
  (expression_value) @injection.content
  (#set! injection.language "elixir")
  (#set! injection.include-children))

; Directives are standalone tags like '<%= @x %>'
;
; Directives with `partial_expression_value` and `ending_expression_value`
; nodes are Elixir code that is part of an expression that spans multiple
; `directive` nodes, so they must be combined. For example:
;     <%= if true do %>
;       <p>hello, tree-sitter!</p>
;     <% end %>
(directive
  [
    (partial_expression_value)
    (ending_expression_value)
  ] @injection.content
  (#set! injection.language "elixir")
  (#set! injection.include-children)
  (#set! injection.combined))

; Directives with `expression_value` nodes do not need to be combined.
; For example:
;     <body>
;       <%= @inner_content %>
;     </body>
(directive
  (expression_value) @injection.content
  (#set! injection.language "elixir"))

; Syntax highlight for <script> tags
((tag
  (start_tag
    (tag_name) @tag_name (#eq? @tag_name "script"))
  (text)
  (end_tag)) @injection.content
  (#offset! @injection.content 1 0 0 -9)
  (#set! injection.language "javascript")
  (#set! injection.include-children)
  (#set! injection.combined))

; Syntax highlight for <style> tags
((tag
  (start_tag
    (tag_name) @tag_name (#eq? @tag_name "style"))
  (text)
  (end_tag)) @injection.content
  (#offset! @injection.content 1 0 0 -9)
  (#set! injection.language "css")
  (#set! injection.include-children)
  (#set! injection.combined))

; Comment parsing languages support
((comment) @injection.content
  (#set! injection.language "comment"))

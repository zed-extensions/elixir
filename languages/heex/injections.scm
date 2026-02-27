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

; Syntax highlight for `style="..."` attributes
(attribute
  (attribute_name) @_attribute_name
  (quoted_attribute_value (attribute_value) @injection.content)
  (#eq? @_attribute_name "style")
  (#set! injection.language "css"))

; Syntax highlight for `onEVENT="..."` attributes
(attribute
  (attribute_name) @_attribute_name
  (quoted_attribute_value (attribute_value) @injection.content)
  (#match? @_attribute_name "^on[a-z]+$")
  (#set! injection.language "javascript"))

; Comment parsing languages support
((comment) @injection.content
  (#set! injection.language "comment"))

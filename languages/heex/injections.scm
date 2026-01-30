; Directives are standalone tags like '<%= @x %>'
;
; `partial_expression_value`s are Elixir code that is part of an expression that
; spans multiple directive nodes, so they must be combined. For example:
;     <%= if true do %>
;       <p>hello, tree-sitter!</p>
;     <% end %>
((directive
  [
    (partial_expression_value)
    (ending_expression_value)
  ] @injection.content)
  (#set! injection.language "elixir")
  (#set! injection.include-children)
  (#set! injection.combined))

; Regular `expression_value`s do not need to be combined
((directive (expression_value) @injection.content)
  (#set! injection.language "elixir"))

; Expressions live within HTML tags, and do not need to be combined
;     <link href={ Routes.static_path(..) } />
; note that we include children, because `expression_value` may consist
; of multiple nodes, when the value contains { and }
((expression (expression_value) @injection.content)
  (#set! injection.language "elixir")
  (#set! injection.include-children))

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

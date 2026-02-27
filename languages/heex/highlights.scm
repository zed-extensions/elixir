; General HEEx tag delimiters are highlighted the same as HTML tag delimiters
[
  "<"
  "<!"
  "</"
  "<:"
  "</:"
  ">"
  "/>"
] @punctuation.bracket.html

; HEEx expression delimiters
[
  "{"
  "}"
] @punctuation.bracket

; HEEx partial expressions can use the following tag delimiters:
;     `<%` and `%>`
;     `<%=` and `%>`
;     `<%%=` and `%>`
(directive) @keyword

; HEEx comments can use the following tag delimiters:
;     `<!--` and `-->`
;     `<%!--` and `--%>`
;     `<%#` and `%>`
(comment) @comment

; HEEx operators are highlighted as such
"=" @operator

; HEEx inherits the DOCTYPE tag from HTML
(doctype) @tag.doctype

; HEEx tags and slots are highlighted the same as HTML tags
[
 (tag_name)
 (slot_name)
] @tag

; HEEx components are highlighted the same as Elixir modules and functions
(component_name
  [
    (module) @type
    (function) @function
    "." @operator
  ])

; HEEx attributes are highlighted the same as HTML attributes
(attribute_name) @attribute

; HEEx special attributes can be any of the following:
;     `:let`
;     `:if`
;     `:for`
;     `:key`
;     `:stream`
(special_attribute_name) @keyword

; HEEx attribute values are highlighted the same as HTML attribute values
[
  (attribute_value)
  (quoted_attribute_value)
] @string

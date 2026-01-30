; Markdown documentation attributes
(unary_operator
  operator: "@"
  operand: (call
  target: (identifier) @__atribute__
  (arguments [
    (string (quoted_content) @injection.content)
    (sigil (quoted_content) @injection.content)
  ]))
  (#any-of? @__atribute__ "moduledoc" "typedoc" "shortdoc" "doc")
  (#set! injection.language "markdown"))

; Regex sigils
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
  (#any-of? @_sigil_name "R" "r")
  (#set! injection.language "regex")
  (#set! injection.combined))

; Phoenix HEEx template sigil
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
  (#eq? @_sigil_name "H")
  (#set! injection.language "heex"))

; Jason sigils
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
  (#any-of? @_sigil_name "J" "j")
  (#set! injection.language "json")
  (#set! injection.combined))

; Phoenix LiveView Component Macros
(call
  (identifier) @_identifier
  (arguments
    (atom)+
    (keywords (pair
      ((keyword) @_keyword (#eq? @_keyword "doc: "))
      [
        (string (quoted_content) @injection.content)
        (sigil (quoted_content) @injection.content)
      ]))
  (#any-of? @_identifier "attr" "slot")
  (#set! injection.language "markdown")))

; Comment parsing languages support
((comment) @injection.content
  (#set! injection.language "comment"))

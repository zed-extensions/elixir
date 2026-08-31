; Elixir expressions in `iex>` prompt lines
((expression) @injection.content
  (#set! injection.language "elixir")
  (#set! injection.combined))

; Elixir expressions in newline after `iex>` prompt lines
((result) @injection.content
  (#set! injection.language "elixir"))

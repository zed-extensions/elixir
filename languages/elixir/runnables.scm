; Macros `describe`, `test`, `property`, `test_with_mock`, and `test_with_mocks`.
; This matches the ExUnit test style.
((call
  target: (identifier) @run) @_elixir-test
  (#any-of? @run "describe" "test" "property" "test_with_mock" "test_with_mocks")
  (#set! tag elixir-test))

; Modules containing at least one `describe`, `test` and `property`.
; This matches the ExUnit test style.
((call
  target: (identifier) @run
  (do_block
    (call target: (identifier) @_keyword
      (#any-of? @_keyword "describe" "test" "property")))) @_elixir-module-test
  (#eq? @run "defmodule")
  (#set! tag elixir-module-test))

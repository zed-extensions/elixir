; Macros `describe`, `test`, `property`, `test_with_mock`, and `test_with_mocks`.
; This matches the ExUnit test style.
(
    (call
        target: (identifier) @run (#any-of? @run "describe" "test" "property" "test_with_mock" "test_with_mocks")
    ) @_elixir-test
    (#set! tag elixir-test)
)

; Modules containing at least one `describe`, `test` and `property`.
; This matches the ExUnit test style.
(
    (call
        target: (identifier) @run (#eq? @run "defmodule")
        (do_block
            (call target: (identifier) @_keyword (#any-of? @_keyword "describe" "test" "property"))
        )
    ) @_elixir-module-test
    (#set! tag elixir-module-test)
)

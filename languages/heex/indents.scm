[
  (start_tag ">" @end)
  (self_closing_tag "/>" @end)
  (start_slot ">" @end)
  (self_closing_slot "/>" @end)
  (start_component ">" @end)
  (self_closing_component "/>" @end)
] @indent

[
  (tag
    (start_tag) @start
    (end_tag)? @end)
  (slot
    (start_slot) @start
    (end_slot)? @end)
  (component
    (start_component) @start
    (end_component)? @end)
] @indent

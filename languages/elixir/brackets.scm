(tuple
  "{" @open
  "}" @close)

(list
  "[" @open
  "]" @close)

(block
  "(" @open
  ")" @close)

(map
  "{" @open
  "}" @close)

(do_block
  "do" @open
  "end" @close)

(arguments
  "(" @open
  ")" @close)

(anonymous_function
  "fn" @open
  "end" @close)

(unary_operator
  operator: "&"
  "(" @open
  ")" @close)

(interpolation
  "#{" @open
  "}" @close
  (#set! rainbow.exclude))

(bitstring
  "<<" @open
  ">>" @close
  (#set! rainbow.exclude))

(string
  quoted_start: _ @open
  quoted_end: _ @close
  (#set! rainbow.exclude))

(charlist
  quoted_start: _ @open
  quoted_end: _ @close
  (#set! rainbow.exclude))

(sigil
  quoted_start: _ @open
  quoted_end: _ @close
  (#set! rainbow.exclude))

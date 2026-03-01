(tuple
  "{" @open
  "}" @close)

(map
  "{" @open
  "}" @close)

(list
  "[" @open
  "]" @close)

(access_call
  "[" @open
  "]" @close)

(block
  "(" @open
  ")" @close)

(arguments
  "(" @open
  ")" @close)

(unary_operator
  operator: "&"
  "(" @open
  ")" @close)

(do_block
  "do" @open
  "end" @close)

(anonymous_function
  "fn" @open
  "end" @close)

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

(quoted_atom
  quoted_start: _ @open
  quoted_end: _ @close
  (#set! rainbow.exclude))

(quoted_keyword
  quoted_start: _ @open
  quoted_end: _ @close
  (#set! rainbow.exclude))

(sigil
  quoted_start: _ @open
  quoted_end: _ @close
  (#set! rainbow.exclude))

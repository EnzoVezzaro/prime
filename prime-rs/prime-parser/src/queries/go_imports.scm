; Go imports query

(import_declaration
  (import_spec
    path: (interpreted_string_literal) @module)) @import

(import_declaration
  (import_spec
    name: (identifier) @alias
    path: (interpreted_string_literal) @module)) @import
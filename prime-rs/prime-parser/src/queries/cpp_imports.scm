; C++ imports query (extends C)

(preproc_include
  path: (string_literal) @module) @import

(preproc_include
  path: (system_lib_string) @module) @import

; Module imports
(import_declaration
  name: (identifier) @module) @import

; Module exports
(export_declaration) @export
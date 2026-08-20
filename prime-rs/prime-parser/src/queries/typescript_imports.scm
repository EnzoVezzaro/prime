; TypeScript imports query

; Import statements
(import_statement
  source: (string) @module) @import

; Import with specifiers
(import_statement
  (import_clause
    (named_imports
      (import_specifier) @item))) @import

; Default imports
(import_statement
  (import_clause
    (identifier) @item)) @import

; Namespace imports
(import_statement
  (import_clause
    (namespace_import
      (identifier) @item))) @import

; Dynamic imports
(call_expression
  function: (identifier) @callee
  (#eq? @callee "import")
  arguments: (arguments
    (string) @module)) @import

; Export statements
(export_statement) @export
; JavaScript imports query

(import_statement
  source: (string) @module) @import

(import_statement
  (import_clause
    (named_imports
      (import_specifier) @item))) @import

(import_statement
  (import_clause
    (identifier) @item)) @import

(import_statement
  (import_clause
    (namespace_import
      (identifier) @item))) @import

(call_expression
  function: (identifier) @callee
  (#eq? @callee "import")
  arguments: (arguments
    (string) @module)) @import

(export_statement) @export
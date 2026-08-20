; Python imports query

(import_statement
  name: (dotted_name) @module) @import

(import_from_statement
  module_name: (dotted_name) @module
  name: (dotted_name) @item) @import

(import_from_statement
  module_name: (dotted_name) @module
  name: (wildcard_import) @item) @import
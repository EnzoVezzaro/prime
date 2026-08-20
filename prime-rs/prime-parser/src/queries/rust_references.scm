; Rust references query
; Captures: @reference (the reference), @target (referenced symbol)

; Simple identifier references
(identifier) @reference

; Scoped references (Module::Type)
(scoped_identifier
  path: (_) @path
  name: (identifier) @reference)

; Field access
(field_expression
  field: (field_identifier) @reference)

; Use statements
(use_declaration
  argument: (use_tree
    (scoped_identifier) @reference))

; Attribute references
(attribute
  (identifier) @reference)

; Type references in annotations
(type_reference
  (scoped_identifier) @reference)
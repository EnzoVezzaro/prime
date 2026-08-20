; Rust types query
; Captures: @type (type annotation), @name (type name)

; Type ascriptions
(type_ascription
  type: (_) @type)

; Type parameters
(type_parameters
  (type_parameter
    name: (identifier) @name))

; Where clauses
(where_clause
  (where_predicate
    type: (_) @type))

; Return types
(function_signature
  return_type: (_) @type)

; Parameter types
(parameter
  type: (_) @type)

; Struct fields
(field_declaration
  type: (_) @type)

; Tuple struct fields
(tuple_struct_field
  type: (_) @type)

; Enum variants
(enum_variant
  (tuple_type) @type)

; Type aliases
(type_item
  type: (_) @type)

; Trait bounds
(type_bound
  (type_identifier) @type)

; Impl trait
(impl_trait
  (type_bound) @type)

; Dyn trait
(dyn_trait
  (type_bound) @type)
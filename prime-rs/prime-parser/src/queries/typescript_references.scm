; TypeScript references query

; Simple references
(identifier) @reference

; Member expressions
(member_expression
  property: (property_identifier) @reference)

; Type references
(type_reference
  (type_identifier) @reference)

; TSX JSX elements
(jsx_element
  name: (jsx_name
    name: (identifier) @reference))

; Import specifiers
(import_specifier
  name: (identifier) @reference)

; Export specifiers
(export_specifier
  name: (identifier) @reference)
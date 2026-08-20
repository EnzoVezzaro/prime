; TypeScript types query

; Type annotations
(type_annotation
  type: (_) @type)

; Type references
(type_reference
  (type_identifier) @type)

; Generic types
(type_arguments
  (type) @type)

; Union types
(union_type
  (type) @type)

; Intersection types
(intersection_type
  (type) @type)

; Array types
(array_type
  element: (_) @type)

; Function types
(function_type
  (type_parameters) @type
  (formal_parameters) @params
  return_type: (_) @return)

; Conditional types
(conditional_type
  check_type: (_) @check
  extends_type: (_) @extends
  true_type: (_) @true
  false_type: (_) @false)

; Mapped types
(mapped_type
  (type_parameter) @param
  (type) @type)

; Indexed access types
(indexed_access_type
  object_type: (_) @object
  index_type: (_) @index)

; Template literal types
(template_literal_type) @type

; Infer types
(infer_type
  (type_parameter) @param)
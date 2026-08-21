; C++ definitions query

(function_definition
  declarator: (identifier) @name) @definition

(class_specifier
  name: (type_identifier) @name) @definition

(struct_specifier
  name: (type_identifier) @name) @definition

(enum_specifier
  name: (type_identifier) @name) @definition

(namespace_definition
  name: (namespace_identifier) @name) @definition

(template_declaration
  (function_definition
    declarator: (identifier) @name)) @definition

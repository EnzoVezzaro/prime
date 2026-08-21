; C definitions query

(function_definition
  declarator: (identifier) @name) @definition

(struct_specifier
  name: (type_identifier) @name) @definition

(enum_specifier
  name: (type_identifier) @name) @definition

(type_definition
  type: (_) @name) @definition

(declaration
  declarator: (identifier) @name) @definition

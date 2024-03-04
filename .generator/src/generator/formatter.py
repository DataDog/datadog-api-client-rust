"""Data formatter."""
from functools import singledispatch
import json
import re

import dateutil.parser

from .utils import snake_case, camel_case, untitle_case, schema_name

PRIMITIVE_TYPES = ["string", "number", "boolean", "integer"]

KEYWORDS = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
}


def is_primitive(schema):
    _type = schema.get("type", "object")
    return _type in PRIMITIVE_TYPES and "enum" not in schema


def block_comment(comment, prefix="///", first_line=True):
    comment = re.sub(r"(\w+://[^\s\(\)]+)", r"<\1>", comment)
    lines = comment.split("\n")
    start = "" if first_line else lines[0] + "\n"
    return (start + "\n".join(f"{prefix} {line}".rstrip() for line in lines[(0 if first_line else 1) :])).rstrip()


def model_filename(name):
    return snake_case(name)


def escape_reserved_keyword(word):
    """
    Escape reserved language keywords like openapi generator does it
    :param word: Word to escape
    :return: The escaped word if it was a reserved keyword, the word unchanged otherwise
    """
    if word in KEYWORDS:
        return f"{word}_"
    return word


def attribute_name(attribute):
    return escape_reserved_keyword(camel_case(attribute))


def variable_name(attribute):
    return escape_reserved_keyword(snake_case(attribute))


def format_value(value, quotes='"', schema=None, version=None):
    if schema and "enum" in schema:
        index = schema["enum"].index(value)
        enum_varnames = schema["x-enum-varnames"][index]
        name = schema_name(schema)
        return f"crate::datadog{version.upper()}::model::{name}::{enum_varnames}"

    if isinstance(value, str):
        return f"{quotes}{value}{quotes}"
    elif isinstance(value, bool):
        return "true" if value else "false"
    elif value is None:
        return "nil"
    return value


def simple_type(schema, render_nullable=False, render_option=True, render_new=False):
    """Return the simple type of a schema.

    :param schema: The schema to extract the type from
    :return: The simple type name
    """
    type_name = schema.get("type")
    type_format = schema.get("format")
    nullable = render_nullable and schema.get("nullable", False)
    
    inner_type = None
    if type_name == "integer":
        inner_type = {
            "int32": "i32",
            "int64": "i64",
            None: "i32",
        }[type_format]

    if type_name == "number":
        inner_type = {
            "double": "f64",
            None: "f64",
        }[type_format]

    if type_name == "string":
        inner_type = {
            "date": "String",
            "date-time": "String",
            "email": "String",
            "binary": "Vec<u8>",
            None: "String",
        }[type_format]
    if type_name == "boolean":
        inner_type = "bool"
    
    if inner_type == None:
        return None

    simple_type = inner_type
    if nullable:
        simple_type = f"Option<{simple_type}>"
    if render_option:
        simple_type = f"Option<{simple_type}>"
    return simple_type


def is_reference(schema, attribute):
    """Check if an attribute is a reference."""
    is_required = attribute in schema.get("required", [])
    if is_required:
        return False

    attribute_schema = schema.get("properties", {}).get(attribute, {})

    is_nullable = attribute_schema.get("nullable", False)
    if is_nullable:
        return False

    is_anytype = attribute_schema.get("type", "object") == "object" and not (
        "properties" in attribute_schema
        or "oneOf" in attribute_schema
        or "anyOf" in attribute_schema
        or "allOf" in attribute_schema
    )
    if is_anytype:
        return False

    # no reference to arrays
    if attribute_schema.get("type", "object") == "array" or "items" in attribute_schema:
        return False

    return True


def attribute_path(attribute):
    return ".".join(attribute_name(a) for a in attribute.split("."))


def rust_name(name):
    """Convert key to Rust name.

    Example:

    >>> rust_name("DASHBOARD_ID")
    dashboard_id
    """
    return name.lower()


def reference_to_value(schema, value, print_nullable=True, **kwargs):
    """Return a reference to a value.

    :param schema: The schema to extract the type from
    :param value: The value to reference
    :return: The simple type name
    """
    type_name = schema.get("type")
    type_format = schema.get("format")
    nullable = schema.get("nullable", False)

    prefix = ""
    if type_name in PRIMITIVE_TYPES:
        prefix = ""
    else:
        prefix = f"datadog{kwargs.get('version', '')}::"

    if nullable and print_nullable:
        if value == "nil":
            formatter = "None"
        else:
            formatter = "Some({value})"
    else:
        formatter = "{value}"

    if type_name == "integer":
        function_name = {
            "int": "i32",
            "int32": "i32",
            "int64": "i64",
            None: "i64",
        }[type_format]
        return formatter.format(prefix=prefix, function_name=function_name, value=value)

    if type_name == "number":
        function_name = {
            "float": "f32",
            "double": "f64",
            None: "f32",
        }[type_format]
        return formatter.format(prefix=prefix, function_name=function_name, value=value)

    if type_name == "string":
        function_name = {
            "date": "Time",
            "date-time": "Time",
            "email": "String",
            None: "String",
        }[type_format]
        return formatter.format(prefix=prefix, function_name=function_name, value=value)

    if type_name == "boolean":
        return formatter.format(prefix=prefix, function_name="bool", value=value)

    if nullable:
        function_name = schema_name(schema)
        if function_name is None:
            raise NotImplementedError(f"nullable {schema} is not supported")
        return formatter.format(prefix=prefix, function_name=function_name, value=value)
    return f"&{value}"


def format_parameters(data, spec, replace_values=None, has_body=False, **kwargs):
    parameters_spec = {p["name"]: p for p in spec.get("parameters", [])}
    if "requestBody" in spec and "multipart/form-data" in spec["requestBody"]["content"]:
        parent = spec["requestBody"]["content"]["multipart/form-data"]["schema"]
        for name, schema in parent["properties"].items():
            parameters_spec[name] = {
                "in": "form",
                "schema": schema,
                "name": name,
                "description": schema.get("description"),
                "required": name in parent.get("required", []),
            }

    parameters = ""
    has_optional = False
    for p in parameters_spec.values():
        required = p.get("required", False)
        if required:
            k = p["name"]
            v = data.pop(k)  # otherwise there is a missing required parameters
            value = format_data_with_schema(
                v["value"],
                p["schema"],
                name_prefix=f"datadog{kwargs.get('version', '')}.",
                replace_values=replace_values,
                required=True,
                **kwargs,
            )
            parameters += f"{value}, "
        else:
            has_optional = True

    body_is_required = spec.get("requestBody", {"required": None}).get("required", False)

    if has_body and body_is_required:
        parameters += "body, "
    if has_optional or body_is_required is False:
        parameters += f"*datadog{kwargs.get('version', '')}.New{spec['operationId'][0].upper()}{spec['operationId'][1:]}OptionalParameters()"
        if has_body and not body_is_required:
            parameters += ".WithBody(body)"

        for k, v in data.items():
            value = format_data_with_schema(
                v["value"],
                parameters_spec[k]["schema"],
                name_prefix=f"datadog{kwargs.get('version', '')}.",
                replace_values=replace_values,
                required=True,
                **kwargs,
            )
            parameters += f".With{camel_case(k)}({value})"

        parameters += ", "

    return parameters


def _format_oneof(schema, data, name, name_prefix, replace_values, required, nullable, **kwargs):
    matched = 0
    one_of_schema = None
    for sub_schema in schema["oneOf"]:
        try:
            if "items" in sub_schema and not isinstance(data, list):
                continue
            if sub_schema.get("nullable") and data is None:
                # only one schema can be nullable
                formatted = "nil"
            else:
                sub_schema["nullable"] = False
                formatted = format_data_with_schema(
                    data,
                    sub_schema,
                    name_prefix=name_prefix,
                    replace_values=replace_values,
                    **kwargs,
                )
            if matched == 0:
                one_of_schema = sub_schema
                # NOTE we do not support mixed schemas with oneOf
                # parameters += formatted
                parameters = formatted
            matched += 1
        except (KeyError, ValueError, TypeError) as e:
            print(f"{e}")

    if matched != 1:
        raise ValueError(f"[{matched}] {data} is not valid for schema {name}")

    one_of_schema_name = schema_name(one_of_schema)
    if not one_of_schema_name:
        one_of_schema_name = simple_type(one_of_schema).title()

    if not is_primitive(one_of_schema):
        parameters = f"Box::new({parameters})"
    if name:
        return f"{name_prefix}{name}::{one_of_schema_name}({parameters})"
    return f"{parameters}"


@singledispatch
def format_data_with_schema(
    data,
    schema,
    name_prefix="",
    replace_values=None,
    default_name=None,
    required=False,
    in_list=False,
    **kwargs,
):
    if not schema:
        return ""

    nullable = schema.get("nullable", False)
    variables = kwargs.get("variables", set())

    name = schema_name(schema)

    if "enum" in schema:
        if nullable and data is None:
            pass
        elif data not in schema["enum"]:
            raise ValueError(f"{data} is not valid enum value {schema['enum']}")

    if replace_values and data in replace_values:
        parameters = replace_values[data]

        # Make sure that variables used in given statements are camelCase for Rust linter
        if parameters in variables:
            parameters = rust_name(parameters)

        simple_type_value = simple_type(schema)
        if isinstance(data, int) and simple_type_value in {
            "float",
            "float32",
            "float64",
        }:
            parameters = f"{simple_type_value}({parameters})"
    else:
        if nullable and data is None:
            parameters = "None"
        else:
            def format_string(x):
                if isinstance(x, bool):
                    raise TypeError(f"{x} is not supported type {schema}")
                if x and ("`" in x or '"' in x or "\n" in x):
                    x = f"r#\"{x}\"#.to_string()"
                    return x
                return f'"{x}".to_string()' if x else '"".to_string()'

            def format_datetime(x):
                d = dateutil.parser.isoparse(x)
                return f"time.Date({d.year}, {d.month}, {d.day}, {d.hour}, {d.minute}, {d.second}, {d.microsecond}, time.UTC)"

            def format_double(x):
                if isinstance(x, (bool, str)):
                    raise TypeError(f"{x} is not supported type {schema}")
                return str(x)

            def format_number(x):
                if isinstance(x, bool):
                    raise TypeError(f"{x} is not supported type {schema}")
                return str(x)

            def format_interface(x):
                if isinstance(x, (int, float)):
                    return str(x)
                if isinstance(x, str):
                    return format_string(x)
                raise TypeError(f"{x} is not supported type {schema}")

            def format_bool(x):
                if not isinstance(x, bool):
                    raise TypeError(f"{x} is not supported type {schema}")
                return "true" if x else "false"

            def open_file(x):
                return f"func() *os.File {{ fp, _ := os.Open({format_string(x)}); return fp }}()"

            formatter = {
                "int32": str,
                "int64": str,
                "double": format_double,
                "date-time": format_datetime,
                "number": format_number,
                "integer": str,
                "boolean": format_bool,
                "string": format_string,
                "email": format_string,
                "binary": open_file,
                None: format_interface,
            }[schema.get("format", schema.get("type"))]

            # TODO format date and datetime
            parameters = formatter(data)

    if "enum" in schema and name:
        if data is None and nullable:
            parameters = "None"
        else:
            # find schema index and get name from x-enum-varnames
            index = schema["enum"].index(data)
            enum_varnames = schema["x-enum-varnames"][index]
            parameters = f"{name_prefix}{name}::{enum_varnames}"

        if not required:
            if nullable:
                if data is None:
                    return "None"
                return f"Some({parameters})"
            parameters = f"{parameters}"
        return parameters

    if in_list and nullable:
        schema = schema.copy()
        schema["nullable"] = False

    if (not required or schema.get("nullable")) and schema.get("type") is not None:
        return reference_to_value(schema, parameters, print_nullable=not in_list, **kwargs)

    if "oneOf" in schema:
        return _format_oneof(schema, data, name, name_prefix, replace_values, required, nullable, **kwargs)

    return parameters


@format_data_with_schema.register(list)
def format_data_with_schema_list(
    data,
    schema,
    name_prefix="",
    replace_values=None,
    default_name=None,
    required=False,
    in_list=False,
    **kwargs,
):
    if not schema:
        return ""

    nullable = schema.get("nullable")

    if "oneOf" in schema:
        return _format_oneof(schema, data, schema_name(schema), name_prefix, replace_values, required, nullable, **kwargs)

    parameters = ""
    list_schema = schema["items"]
    depth = 1
    while list_schema.get("type") == "array":
        list_schema = list_schema["items"]
        depth += 1

    parameters = ""
    for d in data:
        value = format_data_with_schema(
            d,
            schema["items"],
            name_prefix=name_prefix,
            replace_values=replace_values,
            required=not schema["items"].get("nullable", False),
            in_list=True,
            **kwargs,
        )
        parameters += f"{value},"

    if nullable:
        return f"Some(vec![{parameters}])"
    return f"vec![{parameters}]"


@format_data_with_schema.register(dict)
def format_data_with_schema_dict(
    data,
    schema,
    name_prefix="",
    replace_values=None,
    default_name=None,
    required=False,
    in_list=False,
    **kwargs,
):
    if not schema:
        return ""

    nullable = schema.get("nullable", False)

    name = schema_name(schema) or default_name
    required = ""
    parameters = ""
    if "properties" in schema:
        required_properties = set(schema.get("required", []))
        missing = required_properties - set(data.keys())
        if missing:
            raise ValueError(f"missing required properties: {missing}")
        for k, v in schema["properties"].items():
            if k not in data:
                continue
            value = format_data_with_schema(
                data[k],
                v,
                name_prefix=name_prefix,
                replace_values=replace_values,
                default_name=name + camel_case(k) if name else None,
                required=k in required_properties,
                **kwargs,
            )
            if k in required_properties:
                required += f"{value}, "
            else:
                parameters += f".{variable_name(k)}({value})"

    if schema.get("additionalProperties"):
        nested_schema = schema["additionalProperties"]
        nested_schema_name = simple_type(nested_schema)
        if not nested_schema_name:
            nested_schema_name = schema_name(nested_schema)
            if nested_schema_name:
                nested_schema_name = name_prefix + nested_schema_name

        has_properties = schema.get("properties")
        add_parameters = ""
        for k, v in data.items():
            if has_properties and k in schema["properties"]:
                continue
            value = format_data_with_schema(
                v,
                schema["additionalProperties"],
                name_prefix=name_prefix,
                replace_values=replace_values,
                required=True,
                **kwargs,
            )
            add_parameters += f'("{k}".to_string(), {value}),'

            # IMPROVE: find a better way to get nested schema name
            if not nested_schema_name:
                nested_schema_name = value.split("{")[0]

        if has_properties:
            parameters += f".additional_properties(std::collections::BTreeMap::from([{add_parameters}]))"
        else:
            return f"std::collections::BTreeMap::from([{parameters}])"

    if "oneOf" in schema:
        return _format_oneof(schema, data, name, name_prefix, replace_values, required, nullable, **kwargs)

    if schema.get("type") == "object" and "properties" not in schema:
        if schema.get("additionalProperties") == {}:
            for k, v in data.items():
                parameters += f'("{k}".to_string(), serde_json::from_str("{v}").unwrap()),'
            return f"std::collections::BTreeMap::from([{parameters}])"
        else:
            return "std::collections::BTreeMap::new()"

    if not name:
        raise ValueError(f"Unnamed schema {schema} for {data}")

    if parameters == "" and schema.get("type") == "string":
        raise ValueError(f"No schema matched for {data}")

    if nullable:
        return f"Some({name_prefix}{name}::new({required}){parameters})"
    return f"{name_prefix}{name}::new({required}){parameters}"

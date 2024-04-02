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
    return escape_reserved_keyword(snake_case(attribute))


def variable_name(attribute):
    return escape_reserved_keyword(snake_case(attribute))


def format_value(value, quotes='"', schema=None, version=None, required=None):
    if schema and "enum" in schema:
        index = schema["enum"].index(value)
        enum_varnames = schema["x-enum-varnames"][index]
        name = schema_name(schema)
        return f"datadog_api_client::datadog{version.upper()}::model::{name}::{enum_varnames}"

    if isinstance(value, str):
        value = f"{quotes}{value}{quotes}"
    elif isinstance(value, bool):
        value = "true" if value else "false"
    elif value is None:
        value = "None"

    if required is False:
        value = f"Some({value})"

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
            "uuid": "String",
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
        if value == "None":
            return "None"
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
        return formatter.format(prefix=prefix, function_name=function_name, value=f"{value} as {function_name}")

    if type_name == "string":
        function_name = {
            "date": "Time",
            "date-time": "Time",
            "email": "String",
            None: "String",
        }[type_format]
        if function_name == "Time":
            return formatter.format(prefix=prefix, function_name=function_name, value=f"{value}")
        return formatter.format(prefix=prefix, function_name=function_name, value=value)

    if type_name == "boolean":
        return formatter.format(prefix=prefix, function_name="bool", value=value)

    return f"{value}"


def format_parameters(data, spec, replace_values=None, has_body=False, **kwargs):
    imports = set()
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
            value, extra_imports = format_data_with_schema(
                v["value"],
                p["schema"],
                replace_values=replace_values,
                required=True,
                imports=imports,
                **kwargs,
            )
            imports.update(extra_imports)
            parameters += f"{value}, "
        else:
            has_optional = True

    body_is_required = spec.get("requestBody", {"required": None}).get("required", False)

    if has_body and body_is_required:
        parameters += "body, "
    if has_optional or body_is_required is False:
        imports.add(f"datadog_api_client::datadog{kwargs.get('version', '')}::api_{kwargs.get('api')}::{spec['operationId']}OptionalParams")
        parameters += f"{spec['operationId']}OptionalParams::default()"
        if has_body and not body_is_required:
            parameters += ".body(body)"

        for k, v in data.items():
            value, extra_imports = format_data_with_schema(
                v["value"],
                parameters_spec[k]["schema"],
                replace_values=replace_values,
                required=True,
                imports=imports,
                **kwargs,
            )
            imports.update(extra_imports)
            parameters += f".{variable_name(k)}({value})"

        parameters += ", "

    return parameters, imports


def _format_oneof(schema, data, name, replace_values, imports, **kwargs):
    matched = 0
    one_of_schema = None
    for sub_schema in schema["oneOf"]:
        try:
            if "items" in sub_schema and not isinstance(data, list):
                continue
            if sub_schema.get("nullable") and data is None:
                # only one schema can be nullable
                formatted = "None"
            else:
                sub_schema["nullable"] = False
                formatted, extra_imports = format_data_with_schema(
                    data,
                    sub_schema,
                    replace_values=replace_values,
                    imports=set(),
                    **kwargs,
                )
                imports.update(extra_imports)
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

    if not is_primitive(one_of_schema) and one_of_schema.get("type") != "array":
        # TODO: revisit possibility of removing all boxes
        parameters = f"Box::new({parameters})"
    if name:
        imports.add(f"datadog_api_client::datadog{kwargs.get('version', '')}::model::{name}")
        return f"{name}::{one_of_schema_name}({parameters})", imports
    return f"{parameters}", imports


@singledispatch
def format_data_with_schema(
    data,
    schema,
    replace_values=None,
    default_name=None,
    required=False,
    imports=set(),
    **kwargs,
):
    if not schema:
        return "", imports

    nullable = schema.get("nullable", False)
    variables = kwargs.get("variables", set())
    extra_imports = set()

    name = schema_name(schema)

    if "enum" in schema:
        if nullable and data is None:
            pass
        elif data not in schema["enum"]:
            raise ValueError(f"{data} is not valid enum value {schema['enum']}")

    if replace_values and data in replace_values:
        parameters = replace_values[data]

        # Make sure that variables used in given statements are lowercase snake_case for Rust linter
        if parameters in variables:
            parameters = rust_name(parameters) + ".clone()"
    else:
        if nullable and data is None:
            parameters = "None"
        else:
            def format_string(x):
                if isinstance(x, bool):
                    raise TypeError(f"{x} is not supported type {schema}")
                if x and ("`" in x or '"' in x or "\n" in x):
                    return f"r#\"{x}\"#.to_string()", set()
                if x:
                    return f'"{x}".to_string()', set()
                return '"".to_string()', set()

            def format_datetime(x):
                # TODO: format date and datetime
                d = dateutil.parser.isoparse(x)
                return f'"{d.isoformat()}".to_string()', set()
                # if d.microsecond != 0:
                #     return f"(Utc.with_ymd_and_hms({d.year}, {d.month}, {d.day}, {d.hour}, {d.minute}, {d.second}).unwrap() + chrono::Duration::microseconds({d.microsecond})).to_string()"
                # return f"Utc.with_ymd_and_hms({d.year}, {d.month}, {d.day}, {d.hour}, {d.minute}, {d.second}).unwrap().to_string()"

            def format_double(x):
                if isinstance(x, (bool, str)):
                    raise TypeError(f"{x} is not supported type {schema}")
                return str(float(x)), set()

            def format_number(x):
                if isinstance(x, bool):
                    raise TypeError(f"{x} is not supported type {schema}")
                return str(x), set()

            def format_value(x):
                if isinstance(x, (int, float)):
                    return f"Value::from({x})", set(["serde_json::Value"])
                if isinstance(x, str):
                    return f"Value::from(\"{x}\")", set(["serde_json::Value"])
                raise TypeError(f"{x} is not supported type {schema}")

            def format_bool(x):
                if not isinstance(x, bool):
                    raise TypeError(f"{x} is not supported type {schema}")
                if x:
                    return "true", set()
                return "false", set()
            # create a set with a single string element

            def open_file(x):
                return f"fs::read(\"{x}\").unwrap()", set(["std::fs"])

            formatter = {
                "int32": format_number,
                "int64": format_number,
                "double": format_double,
                "date-time": format_datetime,
                "number": format_number,
                "integer": format_number,
                "boolean": format_bool,
                "string": format_string,
                "email": format_string,
                "binary": open_file,
                None: format_value,
            }[schema.get("format", schema.get("type"))]

            # TODO format date and datetime
            parameters, extra_imports = formatter(data)

    if "enum" in schema and name:
        if data is None and nullable:
            parameters = "None"
        else:
            # find schema index and get name from x-enum-varnames
            index = schema["enum"].index(data)
            enum_varnames = schema["x-enum-varnames"][index]
            imports.add(f"datadog_api_client::datadog{kwargs.get('version', '')}::model::{name}")
            parameters = f"{name}::{enum_varnames}" 

        if not required:
            if nullable:
                if data is None:
                    return "None", imports
                return f"Some({parameters})", imports
            parameters = f"{parameters}"
        return parameters, imports

    
    if (not required or schema.get("nullable")) and schema.get("type") is not None:
        imports.update(extra_imports)
        return reference_to_value(schema, parameters, print_nullable=True, **kwargs), imports

    if "oneOf" in schema:
        if default_name and schema_name(schema) is None:
            return _format_oneof(schema, data, default_name+"Item", replace_values, imports, **kwargs)
        return _format_oneof(schema, data, schema_name(schema), replace_values, imports, **kwargs)
    
    imports.update(extra_imports)
    return parameters, imports


@format_data_with_schema.register(list)
def format_data_with_schema_list(
    data,
    schema,
    replace_values=None,
    default_name=None,
    required=False,
    imports=None,
    **kwargs,
):
    if not schema:
        return "", imports

    nullable = schema.get("nullable")

    if "oneOf" in schema:
        if default_name and schema_name(schema) is None:
            return _format_oneof(schema, data, default_name+"Item", replace_values, imports, **kwargs)
        return _format_oneof(schema, data, schema_name(schema), replace_values, imports, **kwargs)

    parameters = ""
    list_schema = schema["items"]
    depth = 1
    while list_schema.get("type") == "array":
        list_schema = list_schema["items"]
        depth += 1

    parameters = ""
    for d in data:
        value, extra_imports = format_data_with_schema(
            d,
            schema["items"],
            replace_values=replace_values,
            default_name=schema_name(schema),
            required=not schema["items"].get("nullable", False),
            imports=set(),
            **kwargs,
        )
        imports.update(extra_imports)
        parameters += f"{value},"

    if nullable:
        return f"Some(vec![{parameters}])", imports
    return f"vec![{parameters}]", imports


@format_data_with_schema.register(dict)
def format_data_with_schema_dict(
    data,
    schema,
    replace_values=None,
    default_name=None,
    required=False,
    imports=None,
    **kwargs,
):
    if not schema:
        return "", imports

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
            value, extra_imports = format_data_with_schema(
                data[k],
                v,
                replace_values=replace_values,
                default_name=name if name else None,
                required=k in required_properties,
                imports=set(),
                **kwargs,
            )
            imports.update(extra_imports)
            if k in required_properties:
                required += f"{value}, "
            else:
                parameters += f".{variable_name(k)}({value})"

    if schema.get("additionalProperties"):
        has_properties = schema.get("properties")
        add_parameters = ""
        for k, v in data.items():
            if has_properties and k in schema["properties"]:
                continue
            value, extra_imports = format_data_with_schema(
                v,
                schema["additionalProperties"],
                replace_values=replace_values,
                required=True,
                imports=set(),
                **kwargs,
            )
            imports.update(extra_imports)
            add_parameters += f'("{k}".to_string(), {value}),'

        imports.add("std::collections::BTreeMap")
        if has_properties:
            parameters += f".additional_properties(BTreeMap::from([{add_parameters}]))"
        else:
            return f"BTreeMap::from([{add_parameters}])", imports

    if "oneOf" in schema:
        return _format_oneof(schema, data, name, replace_values, imports, **kwargs)

    if schema.get("type") == "object" and "properties" not in schema:
        imports.add("std::collections::BTreeMap")
        if schema.get("additionalProperties") == {}:
            for k, v in data.items():
                if isinstance(v, (int, float)):
                    imports.add("serde_json::Value")
                    parameters += f'("{k}".to_string(), Value::from({v})),'
                if isinstance(v, str):
                    imports.add("serde_json::Value")
                    parameters += f'("{k}".to_string(), Value::from(\"{v}\")),'
            return f"BTreeMap::from([{parameters}])", imports
        else:
            return "BTreeMap::new()", imports

    if not name:
        raise ValueError(f"Unnamed schema {schema} for {data}")

    if parameters == "" and schema.get("type") == "string":
        raise ValueError(f"No schema matched for {data}")

    imports.add(f"datadog_api_client::datadog{kwargs.get('version', '')}::model::{name}")
    if nullable:
        return f"Some({name}::new({required}){parameters})", imports
    return f"{name}::new({required}){parameters}", imports

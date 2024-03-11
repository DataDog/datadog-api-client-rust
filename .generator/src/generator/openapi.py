import hashlib
import json
import pathlib
import random
import uuid
import warnings
import yaml

from jsonref import JsonRef
from urllib.parse import urlparse
from yaml import CSafeLoader

from . import formatter


def load(filename):
    path = pathlib.Path(filename)
    with path.open() as fp:
        return JsonRef.replace_refs(yaml.load(fp, Loader=CSafeLoader))


def get_name(schema, version=None):
    name = None
    if hasattr(schema, "__reference__"):
        name = schema.__reference__["$ref"].split("/")[-1]

    return f"crate::datadog{version.upper()}::model::{name}" if version and name else name

def option_wrapper(name, option, nullable):
    if option:
        name = f"Option<{name}>"
    if nullable:
        name = f"Option<{name}>"
    return name

def type_to_rust(schema, alternative_name=None, render_nullable=False, render_option=True, render_box=False, version=None):
    """Return Rust type name for the type."""
    # special case for additionalProperties: true
    if schema is True or schema == {}:
        return "serde_json::Value"

    if "enum" not in schema:
        name = formatter.simple_type(schema, render_nullable=render_nullable, render_option=render_option)
        if name is not None:
            return name

    name = get_name(schema, version)
    if name:
        if "enum" in schema:
            return option_wrapper(name, render_option, render_nullable)
        if not (schema.get("additionalProperties") and not schema.get("properties")) and schema.get("type", "object") == "object":
            name = f"Box<{name}>" if render_box else name
            return option_wrapper(name, render_option, render_nullable)

    type_ = schema.get("type")
    if type_ is None:
        if "items" in schema:
            type_ = "array"
        elif "properties" in schema:
            type_ = "object"
        elif "oneOf" in schema:
            return alternative_name
        else:
            type_ = "object"
            warnings.warn(f"Unknown type for schema: {schema} ({name or alternative_name})")
            return option_wrapper(f"serde_json::Value", render_option, render_nullable)

    if type_ == "array":
        if name or alternative_name:
            alternative_name = (name or alternative_name) + "Item"
        nullable_item = schema["items"].get("nullable")
        name = type_to_rust(schema["items"], alternative_name=alternative_name, render_nullable=nullable_item, render_option=False, version=version)
        return option_wrapper(f"Vec<{name}>", render_option, render_nullable)
    elif type_ == "object":
        name = "serde_json::Value"
        if "additionalProperties" in schema:
            name = type_to_rust(schema["additionalProperties"], render_nullable=render_nullable, render_option=False, version=version)
        return option_wrapper(f"std::collections::BTreeMap<String, {name}>", render_option, render_nullable)

    raise ValueError(f"Unknown type {type_}")


def get_schema_for_attribute(schema, attribute, current_name=None):
    """Return schema for the attribute."""
    return schema.get("properties", {}).get(attribute)


def get_type_for_attribute(schema, attribute, current_name=None):
    """Return Rust type name for the attribute."""
    child_schema = schema.get("properties", {}).get(attribute)
    alternative_name = current_name + formatter.camel_case(attribute) if current_name else None
    return type_to_rust(child_schema, alternative_name=alternative_name)


def get_type_for_parameter(parameter, version=None, render_option=None):
    """Return Rust type name for the parameter."""
    if render_option is None:
        render_option = not parameter.get("required")
    if "content" in parameter:
        assert "in" not in parameter
        for content in parameter["content"].values():
            return type_to_rust(content["schema"], version=version, render_option=render_option)
    return type_to_rust(parameter.get("schema"), version=version, render_option=render_option)

def has_optional_parameter(operation):
    for _, parameter in parameters(operation):
        if not parameter.get("required"):
            return True
    return False

def get_type_for_response(response, version):
    """Return Rust type name for the response."""
    if "content" in response:
        for content in response["content"].values():
            if "schema" in content:
                return type_to_rust(content["schema"], version=version)


def responses_by_types(operation, version):
    result = {}
    for response_code, response in operation["responses"].items():
        if int(response_code) < 300:
            continue
        response_type = get_type_for_response(response, version)
        if response_type in result:
            result[response_type][1].append(response_code)
        else:
            result[response_type] = [response, [response_code]]
    return result.items()


def get_apis_and_versions(all_apis):
    result = {}
    for version, api in all_apis.items():
        for name, _ in api.items():
            if name not in result:
                result[name] = []
            result[name].append(version)
    return result.items()


def child_models(schema, alternative_name=None, seen=None, parent=None):
    seen = seen or set()
    current_name = get_name(schema)
    name = current_name or alternative_name

    # schema["name"] = name

    if parent is not None:
        schema["parent"] = parent

    has_sub_models = False
    if "allOf" in schema:
        has_sub_models = True
        for index in range(len(schema["allOf"])):
            yield from child_models(schema["allOf"][index], seen=seen, parent=schema)
    if "oneOf" in schema:
        has_sub_models = True
        for index in range(len(schema["oneOf"])):
            yield from child_models(schema["oneOf"][index], seen=seen, parent=schema)
    if "anyOf" in schema:
        has_sub_models = True
        for index in range(len(schema["anyOf"])):
            yield from child_models(schema["anyOf"][index], seen=seen, parent=schema)

    if "items" in schema:
        yield from child_models(
            schema["items"],
            alternative_name=name + "Item" if name is not None else None,
            seen=seen,
            parent=schema,
        )

    if (schema.get("type") == "object" or "properties" in schema or has_sub_models) and (
        not (schema.get("additionalProperties") and not schema.get("properties"))
    ):
        if not has_sub_models and name is None:
            # this is a basic map object so we don't need a type
            return

        if name is None:
            raise ValueError(f"Schema {schema} has no name")

        if name in seen:
            return

        if "properties" in schema or has_sub_models:
            seen.add(name)
            yield name, schema

        for key in schema.get("properties", {}):
            yield from child_models(
                schema["properties"][key],
                alternative_name=name + formatter.camel_case(key),
                seen=seen,
                # parent=schema,
            )

    if "enum" in schema:
        if name is None:
            raise ValueError(f"Schema {schema} has no name")

        if name in seen:
            return

        seen.add(name)
        yield name, schema

    if "additionalProperties" in schema:
        nested_name = get_name(schema["additionalProperties"])
        if nested_name:
            yield from child_models(
                schema["additionalProperties"],
                seen=seen,
                # parent=schema,
            )


def models(spec):
    name_to_schema = {}

    for path in spec["paths"]:
        for method in spec["paths"][path]:
            operation = spec["paths"][path][method]

            for content in operation.get("parameters", []):
                if "schema" in content:
                    name_to_schema.update(dict(child_models(content["schema"])))

            for content in operation.get("requestBody", {}).get("content", {}).values():
                if "schema" in content:
                    name_to_schema.update(dict(child_models(content["schema"])))

            for response in operation.get("responses", {}).values():
                for content in response.get("content", {}).values():
                    if "schema" in content:
                        name_to_schema.update(dict(child_models(content["schema"])))

    return name_to_schema


def apis(spec):
    operations = {}

    for path in spec["paths"]:
        for method in spec["paths"][path]:
            operation = spec["paths"][path][method]
            tag = operation.get("tags", [None])[0]
            operations.setdefault(tag, []).append((path, method, operation))

    return operations


def operation(spec, operation_id):
    for path in spec["paths"]:
        for method in spec["paths"][path]:
            operation = spec["paths"][path][method]
            if operation["operationId"] == operation_id:
                return operation
    return None


def parameters(operation):
    for content in operation.get("parameters", []):
        if "schema" in content and content.get("required"):
            yield content["name"], content

    if "requestBody" in operation:
        if "multipart/form-data" in operation["requestBody"]["content"]:
            parent = operation["requestBody"]["content"]["multipart/form-data"]["schema"]
            for name, schema in parent["properties"].items():
                yield name, {
                    "in": "form",
                    "schema": schema,
                    "name": name,
                    "description": schema.get("description"),
                    "required": name in parent.get("required", []),
                }
        else:
            name = operation.get("x-codegen-request-body-name", "body")
            yield name, operation["requestBody"]

    for content in operation.get("parameters", []):
        if "schema" in content and not content.get("required"):
            yield content["name"], content


def form_parameter(operation):
    if "requestBody" in operation and "multipart/form-data" in operation["requestBody"]["content"]:
        parent = operation["requestBody"]["content"]["multipart/form-data"]["schema"]
        [(name, schema)] = list(parent["properties"].items())
        return {
            "schema": schema,
            "name": name,
            "description": schema.get("description"),
            "required": name in parent.get("required", []),
        }


def parameter_schema(parameter):
    if "schema" in parameter:
        return parameter["schema"]
    if "content" in parameter:
        for content in parameter.get("content", {}).values():
            if "schema" in content:
                return content["schema"]

    raise ValueError(f"Unknown schema for parameter {parameter}")


def return_type(operation, version):
    for response in operation.get("responses", {}).values():
        for content in response.get("content", {}).values():
            if "schema" in content:
                return_type = type_to_rust(content["schema"], version=version, render_option=False)
                return return_type, content["schema"]
        return None, None


def accept_headers(operation):
    any_type = "*/*"
    seen = []
    for response in operation.get("responses", {}).values():
        if "content" in response:
            for media_type in response["content"].keys():
                if media_type not in seen:
                    seen.append(media_type)
        else:
            return [any_type]
    return seen


def collection_format(parameter):
    in_to_style = {
        "query": "form",
        "path": "simple",
        "header": "simple",
        "cookie": "form",
    }
    schema = parameter_schema(parameter)
    matrix = {
        ("form", False): "csv",
        ("form", True): "multi",
        # TODO add more cases from https://swagger.io/specification/#parameter-style
    }
    if schema.get("type") == "array" or "items" in schema:
        in_ = parameter.get("in", "query")
        style = parameter.get("style", in_to_style[in_])
        explode = parameter.get("explode", True if style == "form" else False)
        return matrix.get((style, explode), "multi")
    return ""


def format_server(server, server_variables=None, path=""):
    url = server["url"] + path
    # replace potential path variables
    for variable, value in (server_variables or {}).items():
        url = url.replace("{" + variable + "}", value)
    # replace server variables if they were not replace before
    for variable in server["variables"]:
        if server_variables and variable in server_variables:
            continue
        url = url.replace("{" + variable + "}", server["variables"][variable]["default"])
    return urlparse(url)


def server_url_and_method(spec, operation_id, server_index=0, server_variables=None):
    for path in spec["paths"]:
        for method in spec["paths"][path]:
            operation = spec["paths"][path][method]
            if operation["operationId"] == operation_id:
                if "servers" in operation:
                    server = operation["servers"][server_index]
                else:
                    server = spec["servers"][server_index]
                return (
                    format_server(server, server_variables=server_variables, path=path).geturl(),
                    method,
                )

    raise ValueError(f"Operation {operation_id} not found")


def response_code_and_accept_type(operation, status_code=None):
    for response in operation["responses"]:
        if status_code is None:
            return int(response), next(iter(operation["responses"][response].get("content", {None: None})))
        if response == str(status_code):
            return status_code, next(iter(operation["responses"][response].get("content", {None: None})))
    return status_code, None


def request_content_type(operation, status_code=None):
    return next(iter(operation.get("requestBody", {}).get("content", {None: None})))


def response(operation, status_code=None):
    for response in operation["responses"]:
        if status_code is None or response == str(status_code):
            return list(operation["responses"][response]["content"].values())[0]["schema"]
    return None


def get_default(operation, attribute_path):
    attrs = attribute_path.split(".")

    for name, parameter in parameters(operation):
        if name == attrs[0]:
            break
    if name == attribute_path:
        # We found a top level attribute matching the full path, let's use the default
        return formatter.format_value(parameter["schema"]["default"])

    if name == "body":
        parameter = next(iter(parameter["content"].values()))["schema"]
    for attr in attrs[1:]:
        parameter = parameter["properties"][attr]

    return formatter.format_value(parameter["default"])


def get_accessors(operation, attribute_path, container_name="params"):
    param_path = attribute_path.split(".")
    for name, parameter in parameters(operation):
        if name == param_path[0]:
            break

    required = parameter["required"]
    getter = setter = container_name
    if name == "body":
        parameter = next(iter(parameter["content"].values()))
        if required:
            param_path = param_path[1:]
            getter = setter = f"body"

    schema = parameter["schema"]
    for attr in param_path:
        required = True if attr in schema.get("required", []) else False

        name = formatter.attribute_name(attr)
        if required or attr == param_path[0]:
            getter = setter = f"{getter}.{name}"
        else:
            getter = f"{getter}.as_ref().unwrap().{name}"
            setter = f"{setter}.as_mut().unwrap().{name}"

        if attr != param_path[-1] and attr != "body":
            required = attr in schema.get("required", [])
            schema = schema["properties"][attr]

    return getter, setter, required, schema


def get_container(operation, attribute_path, container_name="params"):
    attribute_name = attribute_path.split(".")[0]
    for name, parameter in parameters(operation):
        if name == attribute_name and parameter["required"]:
            return '{}.{}'.format(name, ".".join(formatter.attribute_name(a) for a in attribute_path.split(".")[1:]))

    return f'{container_name}.{formatter.attribute_path(attribute_path)}'


def get_deprecated(schema):
    if "properties" in schema:
        for property in schema["properties"].values():
            if property.get("deprecated", False):
                return True
    return False


def get_container_type(operation, attribute_path, stop=None, **kwargs):
    attrs = attribute_path.split(".")[:stop]
    for name, parameter in parameters(operation):
        if name == attrs[0]:
            break

    if attrs[0] == "body":
        parameter = next(iter(parameter["content"].values()))
        
    if name == attrs[0] and len(attrs) == 1:
        return type_to_rust(parameter["schema"], **kwargs)

    parameter = parameter["schema"]
    for attr in attrs[1:]:
        parameter = parameter["properties"][attr]
    return type_to_rust(parameter, **kwargs)


def get_type_at_path(operation, attribute_path, version=None):
    content = None
    for code, response in operation.get("responses", {}).items():
        if int(code) >= 300:
            continue
        for content in response.get("content", {}).values():
            if "schema" in content:
                break
    if content is None:
        raise RuntimeError("Default response not found")
    content = content["schema"]
    if attribute_path:
        for attr in attribute_path.split("."):
            content = content["properties"][attr]

    return get_name(content.get("items"), version=version)


def generate_value(schema, use_random=False, prefix=None):
    spec = schema.spec
    if not use_random:
        if "example" in spec:
            return spec["example"]
        if "default" in spec:
            return spec["default"]

    if spec["type"] == "string":
        if use_random:
            return str(
                uuid.UUID(
                    bytes=hashlib.sha256(
                        str(prefix or schema.keys).encode("utf-8"),
                    ).digest()[:16]
                )
            )
        return "string"
    elif spec["type"] == "integer":
        return random.randint(0, 32000) if use_random else len(str(prefix or schema.keys))
    elif spec["type"] == "number":
        return random.random() if use_random else 1.0 / len(str(prefix or schema.keys))
    elif spec["type"] == "boolean":
        return True
    elif spec["type"] == "array":
        return [generate_value(schema[0], use_random=use_random)]
    elif spec["type"] == "object":
        return {key: generate_value(schema[key], use_random=use_random) for key in spec["properties"]}
    else:
        raise TypeError(f"Unknown type: {spec['type']}")


class Schema:
    def __init__(self, spec, value=None, keys=None):
        self.spec = spec
        self.value = value if value is not None else generate_value
        self.keys = keys or tuple()

    def __getattr__(self, key):
        return self[key]

    def __getitem__(self, key):
        type_ = self.spec.get("type", "object")
        if type_ == "object":
            try:
                return self.__class__(
                    self.spec["properties"][key],
                    value=self.value,
                    keys=self.keys + (key,),
                )
            except KeyError:
                if "oneOf" in self.spec:
                    for schema in self.spec["oneOf"]:
                        if schema.get("type", "object") == "object":
                            try:
                                return self.__class__(
                                    schema["properties"][key],
                                    value=self.value,
                                    keys=self.keys + (key,),
                                )
                            except KeyError:
                                pass
            raise KeyError(f"{key} not found in {self.spec.get('properties', {}).keys()}: {self.spec}")
        if type_ == "array":
            return self.__class__(self.spec["items"], value=self.value, keys=self.keys + (key,))

        raise KeyError(f"{key} not found in {self.spec}")

    def __repr__(self):
        value = self.value(self)
        if isinstance(value, (dict, list)):
            return json.dumps(value, indent=2)
        return str(value)


class Operation:
    def __init__(self, name, spec, method, path):
        self.name = name
        self.spec = spec
        self.method = method
        self.path = path

    def server_url_and_method(self, spec, server_index=0, server_variables=None):
        def format_server(server, path):
            url = server["url"] + path
            # replace potential path variables
            for variable, value in server_variables.items():
                url = url.replace("{" + variable + "}", value)
            # replace server variables if they were not replace before
            for variable in server["variables"]:
                if variable in server_variables:
                    continue
                url = url.replace(
                    "{" + variable + "}",
                    server["variables"][variable]["default"],
                )
            return url

        server_variables = server_variables or {}
        if "servers" in self.spec:
            server = self.spec["servers"][server_index]
        else:
            server = spec["servers"][server_index]
        return format_server(server, self.path), self.method

    def response_code_and_accept_type(self):
        for response in self.spec["responses"]:
            return int(response), next(iter(self.spec["responses"][response].get("content", {None: None})))
        return None, None

    def request_content_type(self):
        return next(iter(self.spec.get("requestBody", {}).get("content", {None: None})))

    def response(self):
        for response in self.spec["responses"]:
            return Schema(next(iter((self.spec["responses"][response]["content"].values())))["schema"])

    def request(self):
        return Schema(next(iter(self.spec["requestBody"]["content"].values()))["schema"])

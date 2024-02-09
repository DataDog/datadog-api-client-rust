import pathlib

import click
from jinja2 import Environment, FileSystemLoader

from . import openapi
from . import formatter
from . import utils

MODULE = "datadog-api-client-rust"
COMMON_PACKAGE_NAME = "datadog"


@click.command()
@click.argument(
    "specs",
    nargs=-1,
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=pathlib.Path),
)
@click.option(
    "-o",
    "--output",
    type=click.Path(path_type=pathlib.Path),
)
def cli(specs, output):
    """
    Generate a Rust code snippet from OpenAPI specification.
    """
    env = Environment(loader=FileSystemLoader(str(pathlib.Path(__file__).parent / "templates")))

    env.filters["accept_headers"] = openapi.accept_headers
    env.filters["attribute_name"] = formatter.attribute_name
    env.filters["block_comment"] = formatter.block_comment
    env.filters["camel_case"] = formatter.camel_case
    env.filters["collection_format"] = openapi.collection_format
    env.filters["format_server"] = openapi.format_server
    env.filters["format_value"] = formatter.format_value
    env.filters["is_reference"] = formatter.is_reference
    env.filters["is_primitive"] = formatter.is_primitive
    env.filters["parameter_schema"] = openapi.parameter_schema
    env.filters["parameters"] = openapi.parameters
    env.filters["form_parameter"] = openapi.form_parameter
    env.filters["response_type"] = openapi.get_type_for_response
    env.filters["return_type"] = openapi.return_type
    env.filters["simple_type"] = formatter.simple_type
    env.filters["snake_case"] = formatter.snake_case
    env.filters["untitle_case"] = formatter.untitle_case
    env.filters["upperfirst"] = utils.upperfirst
    env.filters["variable_name"] = formatter.variable_name
    env.filters["has_optional_parameter"] = openapi.has_optional_parameter

    env.globals["enumerate"] = enumerate
    env.globals["responses_by_types"] = openapi.responses_by_types
    env.globals["get_name"] = openapi.get_name
    env.globals["get_type_for_attribute"] = openapi.get_type_for_attribute
    env.globals["get_type_for_response"] = openapi.get_type_for_response
    env.globals["get_type_for_parameter"] = openapi.get_type_for_parameter
    env.globals["get_apis_and_versions"] = openapi.get_apis_and_versions
    env.globals["get_type"] = openapi.type_to_rust
    env.globals["get_default"] = openapi.get_default
    env.globals["get_deprecated"] = openapi.get_deprecated
    env.globals["get_container"] = openapi.get_container
    env.globals["get_container_type"] = openapi.get_container_type
    env.globals["get_type_at_path"] = openapi.get_type_at_path
    env.globals["common_package_name"] = COMMON_PACKAGE_NAME
    env.globals["module"] = MODULE

    api_j2 = env.get_template("api.j2")
    model_j2 = env.get_template("model.j2")
    mod_j2 = env.get_template("mod.j2")
    mod_api_j2 = env.get_template("mod_api.j2")
    mod_model_j2 = env.get_template("mod_model.j2")
    # doc_j2 = env.get_template("doc.j2")

    common_files = {
        "configuration.rs": env.get_template("configuration.j2"),
        "mod.rs": env.get_template("common_mod.j2"),
    }
    librs = env.get_template("lib.j2")

    test_fixtures = env.get_template("function_mappings.j2")

    output.mkdir(parents=True, exist_ok=True)

    all_specs = {}
    all_apis = {}
    for spec_path in specs:
        spec = openapi.load(spec_path)
        version = spec_path.parent.name
        all_specs[version] = spec

        apis = openapi.apis(spec)
        all_apis[version] = apis
        models = openapi.models(spec)

        env.globals["openapi"] = spec
        env.globals["version"] = version
        env.globals["package_name"] = f"datadog{version.upper()}"

        resources_dir = output / env.globals["package_name"]
        resources_dir.mkdir(parents=True, exist_ok=True)

        for name, model in models.items():
            filename = "model_" + formatter.model_filename(name) + ".rs"
            model_path = resources_dir / "model" / filename
            model_path.parent.mkdir(parents=True, exist_ok=True)
            with model_path.open("w") as fp:
                fp.write(model_j2.render(name=name, model=model, models=models))

        mod_path = resources_dir / "model" / "mod.rs"
        with mod_path.open("w") as fp:
            fp.write(mod_model_j2.render(apis=apis, models=models))

        all_operations = []

        for name, operations in apis.items():
            filename = "api_" + formatter.snake_case(name) + ".rs"
            api_path = resources_dir / "api" / filename
            api_path.parent.mkdir(parents=True, exist_ok=True)
            with api_path.open("w") as fp:
                fp.write(api_j2.render(name=name, operations=operations))
            all_operations.append((name, operations))

        mod_path = resources_dir / "api" / "mod.rs"
        with mod_path.open("w") as fp:
            fp.write(mod_api_j2.render(apis=apis, models=models))

        mod_path = resources_dir / "mod.rs"
        with mod_path.open("w") as fp:
            fp.write(mod_j2.render(apis=apis, models=models))


        # doc_path = resources_dir / "doc.rs"
        # with doc_path.open("w") as fp:
        #     fp.write(doc_j2.render(all_operations=all_operations))

    common_package_output = pathlib.Path(f"../src/{COMMON_PACKAGE_NAME}")
    common_package_output.mkdir(parents=True, exist_ok=True)
    for name, template in common_files.items():
        filename = common_package_output / name
        with filename.open("w") as fp:
            fp.write(template.render(apis=all_apis, all_specs=all_specs))

    lib_name = output / "lib.rs"
    with lib_name.open("w") as fp:
        fp.write(librs.render())

    test_fixture_output = pathlib.Path("../tests/scenarios/")
    filename = test_fixture_output / "function_mappings.rs"
    with filename.open("w") as fp:
        fp.write(test_fixtures.render(all_apis=all_apis, all_specs=all_specs))

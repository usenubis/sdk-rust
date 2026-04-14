from __future__ import annotations

import pathlib
import re
from dataclasses import dataclass


ROOT = pathlib.Path(__file__).resolve().parents[2]
SDK_ROOT = pathlib.Path(__file__).resolve().parents[1]
MAIN_RS = ROOT / "services" / "api-gateway" / "src" / "main.rs"
OUTPUT = SDK_ROOT / "src" / "generated" / "endpoints.rs"

HTTP_METHODS = ("get", "post", "put", "patch", "delete", "options", "head")


@dataclass(frozen=True)
class Operation:
    method: str
    path: str
    function_name: str
    path_params: tuple[str, ...]


def strip_comments(text: str) -> str:
    without_block = re.sub(r"/\*[\s\S]*?\*/", "", text)
    return re.sub(r"//.*", "", without_block)


def find_matching_paren(text: str, open_index: int) -> int:
    depth = 0
    for index in range(open_index, len(text)):
        char = text[index]
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index
    raise ValueError("could not find matching parenthesis for .route(...)")


def sanitize_identifier(raw: str) -> str:
    normalized = re.sub(r"[^a-zA-Z0-9]+", "_", raw.strip().lower())
    normalized = re.sub(r"_+", "_", normalized).strip("_")
    if not normalized:
        return "root"
    if normalized[0].isdigit():
        return f"_{normalized}"
    return normalized


def function_name_for(method: str, path: str) -> str:
    parts: list[str] = []
    for segment in path.strip("/").split("/"):
        if not segment:
            continue
        if segment.startswith(":"):
            parts.append(f"by_{sanitize_identifier(segment[1:])}")
        else:
            parts.append(sanitize_identifier(segment))
    if not parts:
        parts = ["root"]
    return sanitize_identifier("_".join([method.lower(), *parts]))


def path_params_for(path: str) -> tuple[str, ...]:
    params: list[str] = []
    for segment in path.strip("/").split("/"):
        if segment.startswith(":"):
            params.append(sanitize_identifier(segment[1:]))
    return tuple(params)


def extract_operations(text: str) -> list[tuple[str, str]]:
    operations: list[tuple[str, str]] = []
    cursor = 0

    while True:
        route_index = text.find(".route(", cursor)
        if route_index == -1:
            break

        open_paren = text.find("(", route_index)
        close_paren = find_matching_paren(text, open_paren)
        route_body = text[open_paren + 1 : close_paren]

        match = re.match(r'\s*"([^"]+)"\s*,([\s\S]+)\Z', route_body)
        if match:
            path = match.group(1).strip()
            route_expr = match.group(2)
            methods = {
                method_match.group(1).upper()
                for method_match in re.finditer(r"\b(get|post|put|patch|delete|options|head)\s*\(", route_expr)
            }
            for method in sorted(methods):
                operations.append((method, path))

        cursor = close_paren + 1

    return operations


def dedupe_operations(raw_operations: list[tuple[str, str]]) -> list[Operation]:
    seen_pairs: set[tuple[str, str]] = set()
    used_names: dict[str, int] = {}
    operations: list[Operation] = []

    for method, path in raw_operations:
        pair = (method, path)
        if pair in seen_pairs:
            continue
        seen_pairs.add(pair)

        base_name = function_name_for(method, path)
        count = used_names.get(base_name, 0)
        used_names[base_name] = count + 1
        function_name = base_name if count == 0 else f"{base_name}_{count + 1}"

        operations.append(
            Operation(
                method=method,
                path=path,
                function_name=function_name,
                path_params=path_params_for(path),
            )
        )

    operations.sort(key=lambda item: (item.function_name, item.method, item.path))
    return operations


def render_path_params_array(path_params: tuple[str, ...]) -> str:
    if not path_params:
        return "&[]"
    parts = ", ".join(f"(\"{param}\", {param})" for param in path_params)
    return f"&[{parts}]"


def render_signature_params(path_params: tuple[str, ...]) -> str:
    if not path_params:
        return ""
    return "".join(f", {param}: &str" for param in path_params)


def render_method(operation: Operation) -> str:
    signature_params = render_signature_params(operation.path_params)
    path_params_array = render_path_params_array(operation.path_params)
    doc = f"/// `{operation.method} {operation.path}`"

    if operation.method in {"GET", "HEAD", "OPTIONS"}:
        return f"""{doc}
    pub async fn {operation.function_name}(
        &self{signature_params},
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {{
        self.request_value(
            Method::{operation.method},
            "{operation.path}",
            {path_params_array},
            query,
            Option::<&Value>::None,
        )
        .await
    }}"""

    return f"""{doc}
    pub async fn {operation.function_name}<B: Serialize + ?Sized>(
        &self{signature_params},
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<Value, NubisError> {{
        self.request_value(
            Method::{operation.method},
            "{operation.path}",
            {path_params_array},
            query,
            body,
        )
        .await
    }}"""


def generate_contents(operations: list[Operation]) -> str:
    methods = "\n\n".join(render_method(operation) for operation in operations)
    return f"""// GENERATED FILE - DO NOT EDIT BY HAND
// Source: services/api-gateway/src/main.rs routes

use reqwest::Method;
use serde::Serialize;
use serde_json::Value;

use crate::{{NubisClient, NubisError}};

impl NubisClient {{
{methods}
}}
"""


def main() -> None:
    source = MAIN_RS.read_text(encoding="utf-8")
    stripped = strip_comments(source)
    raw_operations = extract_operations(stripped)
    operations = dedupe_operations(raw_operations)
    contents = generate_contents(operations)
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(contents, encoding="utf-8")
    print(f"generated {len(operations)} operations to {OUTPUT}")


if __name__ == "__main__":
    main()


# `<st-var>`

The `<st-var>` node is used to initialize state variables when the application starts. These variables are evaluated immediately, making them available from the very beginning of the application lifecycle, though their values can be modified later.

## Attributes

| Attribute | Description |
| :--- | :--- |
| `path` | **Required.** The JSON pointer path where the value will be stored. |
| `value` | **Required.** The value to store in the state. |
| `type` | **Optional.** The data type of the value (`number`, `str`, `bool`, `json`). Defaults to `str`. |

### Path Normalization
The `path` attribute uses the standard JSON Pointer syntax (similar to `serde_json::value::Value::pointer`). The path is automatically normalized. For example, `path`, `/path`, `/path/`, and `path/` will all be resolved and stored internally as `/path`.

### Type Casting
The application will attempt to convert the provided `value` into the specified `type`. If the content is not compatible with the requested type (e.g., trying to parse "hello" as a `number`), the conversion fails and the value will not be stored.

## Example

```xml
<state>
    <st-var path="/greeting" value="Hello World" type="str" />
    <st-var path="/settings/max_items" value="10" type="number" />
    <st-var path="/is_logged_in" value="false" type="bool" />
</state>
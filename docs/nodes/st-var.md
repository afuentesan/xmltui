# `<st-var>`

The `<st-var>` node is used to initialize state variables when the application starts. These variables are evaluated immediately, making them available from the very beginning of the application lifecycle, though their values can be modified later.

## Attributes

| Attribute | Description |
| :--- | :--- |
| `path` | **Required.** The JSON pointer path where the value will be stored. |
| `value` | **Required.** The value to store in the state. |
| `id` | **Optional.** A unique identifier that allows event handlers to re-apply this specific value to its `path` later in the application lifecycle. |
| `type` | **Optional.** The data type of the value (`number`, `str`, `bool`, `json`). Defaults to `str`. |

### Path Normalization
The `path` attribute uses the standard JSON Pointer syntax (similar to `serde_json::value::Value::pointer`). The path is automatically normalized. For example, `path`, `/path`, `/path/`, and `path/` will all be resolved and stored internally as `/path`.

### Type Casting
The application will attempt to convert the provided `value` into the specified `type`. If the content is not compatible with the requested type (e.g., trying to parse "hello" as a `number`), the conversion fails and the value will not be stored.

### Multiple Initializers & Event Re-use
You can define multiple `<st-var>` nodes targeting the same `path` as long as they have different `id` attributes. 

* **Startup Precedence:** During initialization, all `<st-var>` nodes are processed in order. If multiple nodes share the same `path`, the **last one defined** will dictate the initial state value.
* **Event Triggers:** By assigning an `id` to each `<st-var>`, you can trigger them individually from user events later on to update the state to that specific value.

## Example

```xml
<state>
    <!-- Basic static variables -->
    <st-var path="/greeting" value="Hello World" type="str" />
    <st-var path="/settings/max_items" value="10" type="number" />

    <!-- Multiple state initializers targeting the same path -->
    <!-- At startup, '/running' will be set to 'false' because 'stop' is defined last -->
    <st-var id="start" path="/running" value="true" type="bool" />
    <st-var id="stop" path="/running" value="false" type="bool" />
</state>
```
# `<st-command>`

The `<st-command>` node dynamically modifies the state by executing one or more system commands (defined in `<code>` nodes) and storing their output.

## Attributes

| Attribute | Description |
| :--- | :--- |
| `exec` | **Required.** The `name` of the `<code>` node to execute. Commands can be chained using `\|`. |
| `path` | **Required.** The JSON pointer path where the command's output will be stored (normalized exactly like `<st-var>`). |
| `id` | **Optional.** A unique identifier to trigger this command later. If omitted, the command cannot be called manually. |
| `on-init` | **Optional.** Boolean (`true` or `false`). Defaults to `false`. If `true`, the command runs automatically when the app starts. |
| `type` | **Optional.** The data type to cast the output into (`number`, `str`, `bool`, `json`). Defaults to `str`. |
| `args` | **Optional.** Maps state values to `<arg-var>` names required by the command. |
| `envs` | **Optional.** Maps state values to `<env-var>` names required by the command. |

> **Note:** If a `<st-command>` lacks both an `id` and `on-init="true"`, it will never be executed.

### Execution & Timing
Commands with `on-init="true"` are executed in parallel **after** the initial UI has been rendered. The state is updated asynchronously as each command finishes. For instance, if one command takes 1 second and another takes 3 seconds, the UI will reflect the first command's data after 1 second, and the second command's data 2 seconds later.

### Command Chaining
You can pipe commands together using the `|` character in the `exec` attribute. The standard output (stdout) of the first command is passed to the standard input (stdin) of the next. The final command's output is saved to the `path`. 
If any command in the chain fails, the entire execution aborts and no state is saved.

```xml
<!-- Executes 'actions', pipes to 'values', then pipes to 'other' -->
<st-command exec="actions|values|other" path="/processed_data" />

```

### Passing State to Arguments and Environments

You can inject current state values into the variables (`<arg-var>` and `<env-var>`) expected by the executed `<code>`.

The syntax for `args` and `envs` is `variable_name:/path/to/state`. Multiple mappings are separated by commas.

```xml
<st-command exec="fetch-user" path="/user/data" args="id:/user/id,role:/user/role" />

```

**Shorthand Syntax:**
If the variable name expected by the command exactly matches the state path root name (e.g., an `<arg-var name="age" />` fetching from `{"age": 47}`), you can omit the colon and the path.

```xml
<!-- Equivalent to args="name:/name,age:/age" -->
<st-command exec="update-profile" path="/status" args="name,age" envs="token" />

```

## Example

```xml
<st-command 
    id="refresh-data" 
    path="/user/actions" 
    exec="actions" 
    type="json" 
    envs="count" 
    args="name:/user/name" 
    on-init="true" 
/>

```
# `<st-command>`

The `<st-command>` node dynamically modifies the state by executing one or more system commands (defined in `<code>` nodes) and storing their output. It also supports continuous execution (looping) and displaying temporary success or error messages to the user.

## Attributes

### Core Attributes
| Attribute | Description |
| :--- | :--- |
| `exec` | **Required.** The `name` of the `<code>` node to execute. Commands can be chained using `\|`. |
| `path` | **Required.** The JSON pointer path where the command's output will be stored (normalized exactly like `<st-var>`). |
| `id` | **Optional.** A unique identifier to trigger this command later. If omitted, the command cannot be called manually via events. |
| `type` | **Optional.** The data type to cast the output into (`number`, `str`, `bool`, `json`). Defaults to `str`. |
| `args` | **Optional.** Maps state values to `<arg-var>` names required by the command. |
| `envs` | **Optional.** Maps state values to `<env-var>` names required by the command. |

### Execution & Timing Attributes
| Attribute | Description |
| :--- | :--- |
| `on-init` | **Optional.** Boolean (`true` or `false`). Defaults to `false`. If `true`, the command runs automatically when the app starts. |
| `refresh-sec` | **Optional.** Unsigned integer. Re-runs the command continuously, waiting this many **seconds** between executions. |
| `refresh-ms` | **Optional.** Unsigned integer. Re-runs the command continuously, waiting this many **milliseconds** between executions. |

### Notification Attributes
| Attribute | Description |
| :--- | :--- |
| `success-title`<br>`err-title` | The title of the message shown on success or error. Can contain inline Jinja `{{ }}` templates. Newlines are ignored (single line only). |
| `success-title-template`<br>`err-title-template` | The `name` of a `<template>` node to use for the message title. |
| `success-message`<br>`err-message` | The body of the message shown on success or error. Can contain inline Jinja `{{ }}` templates. Supports multiline text. |
| `success-message-template`<br>`err-message-template` | The `name` of a `<template>` node to use for the message body. |
| `success-dur`<br>`err-dur` | Unsigned integer. How many seconds the message stays on screen. Defaults to `5`. |

> **Note:** If a `<st-command>` lacks an `id`, `on-init="true"`, `refresh-sec`, and `refresh-ms`, it will never be executed.

---

## Execution & Timing

Commands triggered on initialization are executed in parallel **after** the initial UI has been rendered. The state is updated asynchronously as each command finishes. 

### Looping (`refresh-sec` / `refresh-ms`)
If you provide `refresh-sec` or `refresh-ms`, the command will run automatically when the app starts (making `on-init` unnecessary) and will continue to loop indefinitely until the user navigates to another screen. 

The wait time applies **after** the command finishes executing. For example, if a command takes 4 seconds to run and you set `refresh-sec="1"`, a new execution will start every 5 seconds total (4s running + 1s waiting).

---

## Command Chaining

You can pipe commands together using the `|` character in the `exec` attribute. The standard output (stdout) of the first command is passed to the standard input (stdin) of the next. The final command's output is saved to the `path`. 
If any command in the chain fails, the entire execution aborts and no state is saved.

```xml
<!-- Executes 'actions', pipes to 'values', then pipes to 'other' -->
<st-command exec="actions|values|other" path="/processed_data" />

```

---

## Notifications & Messaging

You can display temporary pop-up messages to the user when a command finishes using the `success-*` and `err-*` attributes.

* **Display Duration:** Messages stay on screen for the amount of seconds defined in `success-dur` or `err-dur` (defaults to 5 seconds). The user can dismiss them early by pressing the `Esc` key.
* **Inline Templates vs. External Templates:** You can write plain text or inline Jinja directly in the `-title` and `-message` attributes (e.g., `success-message="Done: {{ ctx }}"`). Alternatively, you can point to a defined template name using the `-template` variants.

### Context Variables in Notifications

Inside notification templates, you have access to two variables:

* `st`: The global application state.
* `ctx`: The output of the command.
* **On Success:** `ctx` contains the standard output (stdout), cast to the format defined in the `type` attribute (e.g., parsed as JSON if `type="json"`).
* **On Error:** `ctx` contains the raw standard error (stderr) string. It is **not** cast to the `type` attribute, guaranteeing you can read the raw error text.



---

## Passing State to Arguments and Environments

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

---

## Example

```xml
<st-command 
    id="refresh-data" 
    path="/user/actions" 
    exec="actions" 
    type="json" 
    envs="count" 
    args="name:/user/name" 
    refresh-sec="10"
    success-title="Update Successful"
    success-message="Fetched {{ ctx.length }} new actions."
    err-title="Sync Failed"
    err-message="{{ ctx }}"
    err-dur="10"
/>

```
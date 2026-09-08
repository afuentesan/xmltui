# `<template>`

The `<template>` node defines reusable UI fragments. Templates use Jinja2 syntax, allowing for dynamic content generation based on application state or command outputs.

## Importing External Templates

You can load templates from an external file using the `src` attribute. The path must be relative to the application's root directory.

```xml
<template src="templates/common.xml" />

```

**External File Format:**
The external file must have `<rtml>` as its root node and contain the `<template>` nodes you want to import.

```xml
<!-- /home/user/examples/templates/common.xml -->
<rtml>
    <template name="greeting">
        <line>Hello {{ st.name }}</line>
    </template>
</rtml>

```

## Inline Definition

To define a template directly inside the `<head>`, you must provide the `name` attribute and the UI structure inside the node.

| Attribute | Description |
| --- | --- |
| `name` | A unique identifier for the template. If multiple templates share the same name, the last one defined takes precedence. |

### Dynamic Context (Jinja2)

Inside any template, you can use `{{ ... }}` to inject dynamic data:

* `st`: You always have access to the global application state via the `st` variable.
* `ctx`: If the template is used to render the result of a `<command>`, the command's raw output or parsed JSON response will be available in the `ctx` variable.

```xml
<template name="user-greeting">
    <!-- Uses the global state 'username' -->
    <line>Welcome back, <span class="highlight">{{ st.username }}</span></line>
</template>

```
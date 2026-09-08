# `<code>`

The `<code>` node defines an external system command that your application can execute. You can define commands directly inline or import them from external files.

## Importing External Commands

If you provide the `src` attribute, the node will load external command definitions. When using `src`, any other attributes or child nodes are ignored. 

The path provided in `src` must be relative to the application's root directory (the `--root` parameter passed to `xmltui`).

```xml
<code src="bin/common.xml" />

```

**External File Format:**
The external file must have `<rtml>` as its root node and contain the `<code>` nodes you want to import.

```xml
<!-- /home/user/examples/bin/common.xml -->
<rtml>
    <code command="date" name="date-command" />
    <code command="ls" name="ls-command" />
</rtml>

```

## Inline Definition

To define a command directly in the `<head>`, you must provide at least the `command` and `name` attributes.

| Attribute | Description |
| --- | --- |
| `command` | The executable to run (e.g., `date`, `/usr/bin/head`). |
| `name` | A unique identifier used to refer to this command later. If multiple `<code>` nodes share the same `name`, the last one defined takes precedence. |

### Arguments (`<args>`)

You can pass arguments to your command using the `<args>` wrapper node, which can contain:

* `<arg>`: A constant argument. The content of the node is the argument's value.
* `<arg-var>`: A variable argument. It requires a `name` attribute. The caller executing the command must provide the actual value for this variable.

```xml
<code command="/usr/bin/head" name="head-command">
    <args>
        <arg>-3</arg>
        <arg-var name="file" />
    </args>
</code>

```

### Environment Variables (`<envs>`)

You can set environment variables for the command execution using the `<envs>` wrapper node, which can contain:

* `<env>`: A constant environment variable. Requires a `name` attribute. The content of the node is the value.
* `<env-var>`: A variable environment variable. Requires a `name` attribute. The caller executing the command must provide the actual value.

### Complex Example

Here is an example using `bash`, setting environment variables, and utilizing `<![CDATA[ ... ]]>` to safely pass a script containing special characters:

```xml
<code command="/usr/bin/bash" name="greet">
    <envs>
        <env name="greeting">Hello</env>
        <env-var name="name" />
    </envs>
    <args>
        <arg>-c</arg>
        <arg>
            <![CDATA[ 
                echo "$greeting $name"
            ]]>
        </arg>
    </args>
</code>

```
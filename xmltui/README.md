# xmltui

**xmltui** is a framework for building Terminal User Interfaces (TUIs) using a web-like approach, allowing you to design terminal apps similarly to how you would build a webpage.

Under the hood, it leverages **Ratatui** for rendering the UI in the terminal, **Tokio** for asynchronous command execution, and **MiniJinja** for parsing and rendering Jinja2 templates.

## Example

Let's build a simple application that displays the current date and time in the center of the screen.
*Note: This example requires the `date` command to be installed on your system.*

```xml
<rtml xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" 
      xsi:noNamespaceSchemaLocation="schema.xsd">

    <!--
    The <head> node is where you define commands, styles, and templates. 
    These can also be defined in external files and imported from the head. 
    -->
    <head>

        <!-- 
        The <code> node is used to define the commands we want to execute. 
        The 'command' attribute specifies the executable, and 'name' provides an identifier 
        to reference this command later. 
        Arguments and environment variables can also be defined here, but since this 
        example doesn't need them, we leave the <code> node self-closed.
        -->
        <code command="date" name="date-command" />

        <!-- 
        The <style> node is used to define the layout and styling of the application. 
        Styles are written in JSON format. The outer opening and closing braces can be 
        omitted, but the rest must be valid JSON.
        To target specific nodes, you can use tag names, class names (prefixed with a dot '.'), 
        and IDs (prefixed with a hash '#'), exactly like CSS.
        -->
        <style>
            "body" : {
                "flex" : "center",
                "dir" : "vertical"
            },
            "command" : {
                "length" : "1"
            },
            "line" : {
                "align" : "center"
            }
        </style>
    </head>

    <!-- 
    The <body> node contains the visible elements that will be rendered on the screen.
    -->
    <body>
        <!-- 
        The <command> node executes a defined command and renders a template with its output.
        The template can be defined inline (inside the node), or referenced via the 'template' 
        attribute if it was defined in the <head> or an external file.
        The 'exec' attribute must point to the 'name' of a <code> node.
        By setting 'refresh-sec="1"', we tell the app to re-run the command every second. 
        By default, commands run only once on startup, though they can be triggered by events.
        -->
        <command exec="date-command" refresh-sec="1"> 
            <template>
                <!-- 
                <line> corresponds to Ratatui's Line widget. It can contain plain text and/or <span> nodes. 
                The 'ctx' variable holds the output of the executed command. 
                By default, command output is treated as raw text, but it can be parsed as JSON by 
                adding 'output="json"' if the command returns valid JSON data.
                The double curly braces are Jinja2 syntax. 
                Since our command outputs plain text, we simply use {{ ctx }} to render it directly.
                -->
                <line>{{ ctx }}</line>
            </template>
        </command>
    </body>
</rtml>
```

## Installation

You can install `xmltui` using Cargo:

```bash
cargo install xmltui

```

`xmltui` acts as the engine that interprets and renders your XML application—conceptually similar to how a web browser renders web pages.

## Usage

Assuming you saved the previous example at `/home/user/examples/clock.xml`, you can run it with the following command:

```bash
xmltui --root="/home/user/examples" --path="clock"

```

### CLI Arguments

* `--root`: The base directory path of your application.
* `--path`: The entry point file to render (the `.xml` extension is optional).

### Navigation & Default Entry Point

* **Default Entry Point:** If the `--path` argument is omitted, `xmltui` will automatically look for an `index.xml` file inside the specified `--root` directory.
* **Multi-file Navigation:** You can split your application across multiple XML files within the root folder and navigate between them using anchor tags, for example: `<a src="another.xml">`.
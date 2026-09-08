# `<rtml>`

The `<rtml>` node is the root element of any `xmltui` document. Every valid `xmltui` application must start and end with this tag.

## Attributes

Unlike UI nodes, the `<rtml>` node does not use the common styling attributes. Instead, it is typically used to define the XML namespaces and the schema location. Providing the schema URL enables validation and syntax autocompletion in most modern code editors (like VS Code or Neovim).

| Attribute | Description |
| :--- | :--- |
| `xmlns:xsi` | Standard XML namespace declaration (usually `http://www.w3.org/2001/XMLSchema-instance`). |
| `xsi:noNamespaceSchemaLocation` | URL pointing to the `xmltui` XSD schema (e.g., the raw GitHub URL) to enable editor support. |

## Allowed Children

The `<rtml>` node expects up to three specific structural child nodes. They should be defined in the following order:

* **[`<head>`](./head.md)** *(Optional)*: Used to define application metadata, import external files, and declare `<style>`, `<code>` (commands), and `<template>` nodes.
* **[`<state>`](./state.md)** *(Optional)*: Used to declare the initial state variables of the application. These variables can later be accessed in Jinja2 templates using the `st` object.
* **[`<body>`](./body.md)** *(Mandatory)*: Contains the visible layout and UI components (such as `<line>`, `<command>`, etc.) that will be rendered on the terminal screen.

## Example

Here is a standard boilerplate for an `xmltui` application:

```xml
<rtml xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" 
      xsi:noNamespaceSchemaLocation="https://raw.githubusercontent.com/afuentesan/xmltui/refs/heads/main/schema.xsd">

    <!-- Optional: Defines styles, commands, and templates -->
    <head>
        
    </head>

    <!-- Optional: Defines initial application state -->
    <state>

    </state>

    <!-- Mandatory: Defines the user interface -->
    <body>

    </body>
</rtml>
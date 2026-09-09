# `<state>`

The `<state>` node is an optional structural element used to declare and initialize the application's state. The state acts as a global key-value store (accessible via the `st` object in templates) that can be dynamically updated during the application's lifecycle.

## Allowed Children

Inside the `<state>` node, you can define:

* **[`<st-var>`](./st-var.md)**: Initializes static state variables when the application starts.
* **[`<st-command>`](./st-command.md)**: Executes system commands to populate or update the state dynamically.
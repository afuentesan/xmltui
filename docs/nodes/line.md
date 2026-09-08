# `<line>`

The `<line>` node represents a single line of text on the screen. It is a wrapper around Ratatui's `Line` widget.

## Attributes

The `<line>` node accepts all the [Common Attributes](../common-attributes.md) (like `fg`, `bg`, `id`, `class`), plus the following specific alignment and spacing properties:

| Attribute | Values | Description |
| :--- | :--- | :--- |
| `align` | `left`, `center`, `right` | Aligns the text within its container. |
| `padding-left` | Unsigned Integer | Adds empty spaces to the left of the content. |
| `padding-right`| Unsigned Integer | Adds empty spaces to the right of the content. |
| `padding` | Unsigned Integer | Applies the same amount of empty spaces to both the left and right. |

## Allowed Children

A `<line>` node can contain:
* Plain text.
* `<span>` nodes (for styling specific parts of the text).
* A `<command>` node.

### Example 1: Text and Spans
You can mix plain text and styled spans inside a single line:

```xml
<line align="center" padding="2">
    Normal text and <span class="red-text">red text</span>
</line>

```

### Example 2: Dynamic Content via `<command>`

If you place a `<command>` inside a `<line>`, the command's output will replace any other content inside the line.
*Note: The command must return only text and/or `<span>` nodes. Any other returned nodes will be ignored.*

```xml
<line align="right">
    <command exec="date-command" refresh-sec="1">
        <template>
            {{ ctx }}
        </template>
    </command>
</line>
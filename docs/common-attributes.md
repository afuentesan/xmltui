# Common Attributes

Most UI nodes in `xmltui` share a standard set of attributes for styling and identification. Instead of repeating these in every node's documentation, they are compiled here.

## Identifiers

| Attribute | Description |
| :--- | :--- |
| `id` | A unique identifier for the node. |
| `class` | Space-separated class names used to target the node in the `<style>` section. |

## Styling & Layout

For colors (`fg`, `bg`, `uc`), you can use **Ratatui named colors** (e.g., `Red`, `LightBlue`) or **Hexadecimal colors** (e.g., `#ffffff`).

| Attribute | Values | Description |
| :--- | :--- | :--- |
| `fg` | Color | The text (foreground) color. |
| `bg` | Color | The background color. |
| `uc` | Color | Underline color (only visible if `text-decoration` includes underline). |
| `font-weight` | `normal`, `bold` | Sets the text weight. |
| `font-style` | `normal`, `italic` | Sets the text style. |
| `dim` | `true`, `false` | Applies Ratatui's `Dim` modifier, which lowers the brightness/intensity of the foreground color. |
| `text-decoration`| `none`, `underline`, `line-through` | Adds decorative lines to the text. |
| `blink` | `normal`, `slow`, `rapid` | Applies a blinking effect to the text (support depends on the terminal emulator). |
| `invert` | `true`, `false` | Inverts the foreground and background colors. |
| `visibility` | `visible`, `hidden` | Controls whether the node is rendered on the screen. |

---

## Dynamic Attributes & Templating

Except for `id` and `class`, all the attributes listed above can be dynamically calculated using templates or state variables.

### The `-template` suffix
You can append `-template` to any style attribute to compute its value using a predefined template (located in the `<head>` or an external file).

```xml
<line fg-template="my-color-template">Text</line>

```

### Inline Jinja2 Templating

If an attribute's value starts and ends with double curly braces `{{ }}`, it will be evaluated as a Jinja2 expression. Inside these templates, you have access to the application state via the `st` variable.

```xml
<!-- The background color will change based on the 'bgline' state variable -->
<line bg="{{ st.bgline }}">Text</line>

```
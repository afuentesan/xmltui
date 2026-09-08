# `<style>`

The `<style>` node is used to define the visual appearance (colors, alignment, modifiers) of your application's components. 

## Importing External Styles

You can load styles from an external JSON file using the `src` attribute. The path must be relative to the application's root directory.

```xml
<style src="styles/common.json" />

```

## Inline Definition

If you define styles directly inside the `<style>` node, the content must be written in JSON format. For convenience, the outermost opening and closing curly braces `{ }` can be omitted, but the rest of the content must be valid JSON.

You can target nodes by their tag name, by `class` (prefixing a `.`), or by `id` (prefixing a `#`).

```xml
<style>
    ".red" : { 
        "fg" : "Red",
        "font-weight" : "bold"
    },
    "#main-title" : {
        "align": "center"
    },
    "line" : {
        "bg" : "#333333"
    }
</style>

```

You can define as many `<style>` nodes as needed inside the `<head>`.
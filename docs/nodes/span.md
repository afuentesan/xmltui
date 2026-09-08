# `<span>`

The `<span>` node is an inline text wrapper used to apply specific styles to a portion of text within a parent element (such as a [`<line>`](./line.md)).

## Attributes

The `<span>` node accepts all [Common Attributes](../common-attributes.md) (such as `fg`, `bg`, `font-weight`, `text-decoration`, `class`, `id`, etc.). It does not introduce any node-specific attributes.

## Allowed Content

A `<span>` node can **only contain plain text**.

> ⚠️ **Constraint:** Nested `<span>` nodes are not supported. A `<span>` cannot contain other HTML/XML tags or child `<span>` elements inside it.

## Example

```xml
<line>
    This is normal text, 
    <span fg="Red" font-weight="bold">bold red text</span>, 
    and <span bg="#333333" text-decoration="underline">underlined text with a background</span>.
</line>
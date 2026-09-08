# `<head>`

The `<head>` node is an optional structural element used to define the configuration and resources of your `xmltui` application. It does not render anything directly on the screen. Instead, it acts as a container for commands, styles, and templates.

## Allowed Children

Inside the `<head>` node, you can define as many of the following nodes as you need:

* **[`<code>`](./code.md)**: Defines system commands and their arguments/environment variables so they can be executed later by the application.
* **[`<style>`](./style.md)**: Defines the visual layout and styling rules (using JSON) for the application's UI nodes.
* **[`<template>`](./template.md)**: Defines reusable UI components and layouts using Jinja2 syntax.

You can mix and match these nodes freely within the `<head>`.
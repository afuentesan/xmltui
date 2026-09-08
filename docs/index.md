# xmltui Documentation

**xmltui** is a framework for building Terminal User Interfaces (TUIs) using a web-like approach. It leverages Ratatui for rendering, Tokio for async execution, and MiniJinja for templating, allowing you to design terminal applications using XML.

Welcome to the official documentation. Here you will find detailed references for all available nodes and their attributes.

## Reference Guide

### Core Concepts
* [Common Attributes](./common-attributes.md) - Shared identifiers, styling, and templating rules that apply to most nodes.

### Node Reference
* [`<line>`](./nodes/line.md) - A wrapper for Ratatui's Line widget. Represents a single line of text or spans.
* [`<span>`](./nodes/span.md) - Used to apply specific styles to inline text segments.
* [`<command>`](./nodes/command.md) - Executes background processes and renders their output using templates.

### Additional Guides
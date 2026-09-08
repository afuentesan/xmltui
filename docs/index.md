# xmltui Documentation

**xmltui** is a framework for building Terminal User Interfaces (TUIs) using a web-like approach. It leverages Ratatui for rendering, Tokio for async execution, and MiniJinja for templating, allowing you to design terminal applications using XML.

Welcome to the official documentation. Here you will find detailed references for all available nodes and their attributes.

## Reference Guide

### Core Concepts
* [Common Attributes](./common-attributes.md) - Shared identifiers, styling, and templating rules that apply to most nodes.

### Node Reference

**Structural Nodes**
* [`<rtml>`](./nodes/rtml.md) - The root element of any `xmltui` document.
* [`<head>`](./nodes/head.md) - Container for configuration, commands, styles, and templates.
* [`<state>`](./nodes/state.md) - Defines the initial state variables of the application.
* [`<body>`](./nodes/body.md) - Contains the visible layout and UI components.

**Configuration Nodes (inside `<head>`)**
* [`<code>`](./nodes/code.md) - Defines external system commands, arguments, and environment variables.
* [`<style>`](./nodes/style.md) - Defines visual styling rules for the application using JSON format.
* [`<template>`](./nodes/template.md) - Defines reusable UI fragments and layouts using Jinja2 syntax.

**UI Nodes (inside `<body>`)**
* [`<line>`](./nodes/line.md) - A wrapper for Ratatui's Line widget. Represents a single line of text or spans.
* [`<span>`](./nodes/span.md) - Used to apply specific styles to inline text segments.
* [`<command>`](./nodes/command.md) - Executes background processes and renders their output using templates.

### Additional Guides
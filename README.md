# Tsui

A data-driven, multi-backend GUI environment.

Tsui is a GUI environment that includes both a framework for building data-driven interfaces and a set of general-purpose applications. It generates user interfaces from data definitions, with sensible defaults and the ability to customize specific views. Multiple GUI backends (e.g. Bevy, Dioxus, gpui, egui) are supported through a common abstraction, so the same data model can drive desktop, web, or other rendering targets.

Tsui works with any application — including wrapping CLI programs — but integrates most deeply with moco apps, where the plugin architecture and lifecycle management enable a seamless experience.

## The Problem

Building GUIs is repetitive. Most application data follows patterns (lists, forms, trees, dashboards) that get re-implemented from scratch in every project, locked to a single toolkit. And every developer environment ends up needing the same set of utility apps, rebuilt from nothing each time.

## Tsui's Approach

Define your data, get a working UI. Customize where it matters, accept smart defaults everywhere else. Swap backends without changing your data model. Ship with general-purpose apps so the environment is useful from day one.

## What Tsui Includes

- **Framework** — The core abstraction for turning data definitions into UIs, with a provider model for pluggable backends.
- **Apps** — A set of general-purpose GUI applications, similar to what a desktop environment provides out of the box.

## Principles

- **Data first, pixels second** — The UI is a projection of your data. Change the data model, the UI updates. No manual layout for common patterns.

- **Defaults that work, overrides that compose** — Auto-generated views cover the common case. Custom views slot in for specific needs without replacing the whole system.

- **Backend-agnostic** — A shared abstraction layer lets GUI providers (Bevy, Dioxus, gpui, egui, etc.) plug in beneath the same data definitions.

- **Standalone but moco-native** — Tsui can wrap any application, even CLI tools. With moco apps, it gains automatic plugin discovery, lifecycle integration, and richer data binding.

- **Environment, not just a toolkit** — Tsui ships with ready-made apps, not just building blocks.

## Part of Kudo

Tsui serves as the GUI layer in the [Kudo](https://github.com/edger-dev/kudo) platform. Kudo-specific applications (kinora viewer, hub dashboard, agent monitor) are built in the Kudo repo on top of tsui's framework.


# Trait Usage Audit

Audit of core trait implementations (`Focusable`, `Clickable`, `Container`, `PopupContainer`) across all components in `src/components/`.

**Date:** 2026-02-25

---

## Trait Definitions

| Trait | File | Purpose |
|-------|------|---------|
| `Focusable` | `src/traits/focusable.rs` | Components that receive keyboard focus and participate in Tab navigation |
| `Clickable` | `src/traits/clickable.rs` | Components that respond to mouse clicks via registered click regions |
| `Container` | `src/traits/container.rs` | Components that manage child components (render, handle_key, handle_mouse) |
| `PopupContainer` | `src/traits/container.rs` | Extension of Container for popup/modal behavior (centering, close-on-escape) |

---

## Implementation Matrix

### Interactive Components

| Component | `focused` field | `FocusId` | `Focusable` impl | `ClickRegion` use | `Clickable` impl | `handle_key` fn | `handle_mouse` fn | `Container` impl |
|-----------|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| **Button** | yes | yes | **no** | yes (Registry) | **no** | no | no | no |
| **CheckBox** | yes | yes | **no** | yes | **no** | no | no | no |
| **Input** | yes | yes | **no** | yes | **no** | no | no | no |
| **TextArea** | yes | yes | **no** | yes | **no** | no | no | no |
| **Select** | yes | yes | **no** | yes | **no** | yes | yes | no |
| **ContextMenu** | no | no | no | yes | **no** | yes | yes | no |
| **MenuBar** | yes | no | **no** | yes | **no** | yes | yes | no |
| **PopupDialog** | via FocusManager | no | no | yes (Registry) | **no** | yes (internal) | yes (internal) | **no** |
| **HotkeyDialog** | yes (custom) | no | **no** | yes | **no** | yes | yes | no |

### Navigation Components

| Component | `focused` field | `FocusId` | `Focusable` impl | `ClickRegion` use | `Clickable` impl | `handle_key` fn | `handle_mouse` fn | `Container` impl |
|-----------|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| **ListPicker** | no | no | no | no | no | no | no | no |
| **TreeView** | no | no | no | no | no | no | no | no |
| **FileExplorer** | no | no | no | no | no | no | no | no |
| **Accordion** | no | no | no | no | no | yes | yes | no |
| **Breadcrumb** | yes | no | **no** | yes | **no** | yes | yes | no |
| **ScrollableContent** | yes (private) | no | **no** | no | no | yes | yes | no |
| **DiffViewer** | no | no | no | no | no | yes | yes | no |
| **LogViewer** | no | no | no | no | no | no | no | no |

### Layout Components

| Component | `focused` field | `FocusId` | `Focusable` impl | `ClickRegion` use | `Clickable` impl | `handle_key` fn | `handle_mouse` fn | `Container` impl |
|-----------|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| **TabView** | yes | yes | **yes** | yes (Registry) | **no** | yes | yes | no |
| **SplitPane** | yes | yes | **yes** | yes (Registry) | **no** | yes | yes | no |

### Display Components (no interaction expected)

| Component | Traits needed |
|-----------|:---:|
| AnimatedText | none |
| ParagraphExt | none |
| Toast | none |
| Progress | none |
| MarqueeText | none |
| Spinner | none |
| StatusLine | none |
| HotkeyFooter | none |
| StepDisplay | none |
| MousePointer | none |

---

## Summary Counts

| Trait | Implementations | Components that *should* implement |
|-------|:-:|:-:|
| `Focusable` | **2** (TabView, SplitPane) | **11** (+ Button, CheckBox, Input, TextArea, Select, MenuBar, Breadcrumb, ScrollableContent, PopupDialog) |
| `Clickable` | **0** | **10** (Button, CheckBox, Input, TextArea, Select, TabView, SplitPane, ContextMenu, MenuBar, Breadcrumb) |
| `Container` | **0** | **3+** (PopupDialog, HotkeyDialog, ContextMenu) |
| `PopupContainer` | **0** | **3+** (PopupDialog, HotkeyDialog, Select dropdown) |

---

## Gap Analysis

### Gap 1: Focusable trait adoption (critical)

**9 components** have ad-hoc `focused: bool` fields and/or `FocusId` usage but do not implement `Focusable`:

- **Button** -- has `focused: bool`, `FocusId`, `set_focused()` method
- **CheckBox** -- has `focused: bool`, `FocusId`, `set_focused()` method
- **Input** -- has `focused: bool`, `FocusId`
- **TextArea** -- has `focused: bool`, `FocusId`
- **Select** -- has `focused: bool`, `FocusId`
- **MenuBar** -- has `focused: bool` (no FocusId)
- **Breadcrumb** -- has `focused: bool` (no FocusId)
- **ScrollableContent** -- has `focused: bool`, `is_focused()`, `set_focused()` (duplicates the trait API exactly)
- **PopupDialog** -- manages focus via `FocusManager` internally

**Impact:** The `Focusable` trait provides `current_style()`, `can_focus()`, and `tab_order()` methods that would enable generic focus management. Without trait adoption, `FocusManager` cannot query components polymorphically.

### Gap 2: Clickable trait is dead code (critical)

**Zero components** implement `Clickable`. Instead, components use `ClickRegion` and `ClickRegionRegistry` directly as data structures, bypassing the trait entirely.

Components using click infrastructure ad-hoc:
- Button, CheckBox, Input, TextArea, Select (use `ClickRegion`)
- TabView, SplitPane, PopupDialog (use `ClickRegionRegistry`)
- ContextMenu, MenuBar, Breadcrumb (use `ClickRegion`)

**Root cause:** The `Clickable` trait requires `click_regions() -> &[ClickRegion<Self::ClickAction>]`, but most components store click regions in their widget struct (not state), or use `ClickRegionRegistry` which has its own `handle_click()`. The trait doesn't integrate with the registry pattern that components actually use.

### Gap 3: Container/PopupContainer traits are unused (major)

**Zero components** implement `Container` or `PopupContainer`, despite:

- `PopupDialog` (components/container.rs) uses `EventResult` and `ContainerAction` from the trait module but implements its own render/handle pattern
- `HotkeyDialog` acts as a full popup dialog with key/mouse handling
- `ContextMenu` is a popup overlay with its own positioning logic
- `Select` has a dropdown popup overlay

**Root cause:** The `Container` trait requires `type State` associated type and `render(&self, frame, area, state)`, but most components use a widget+state pattern where the widget borrows state during rendering (ratatui `StatefulWidget` style). The trait's API doesn't match the codebase's actual architecture.

---

## Inconsistencies

### 1. Dual focus systems

Two unintegrated focus systems exist:

- **`Focusable` trait** (per-component) -- provides `focus_id()`, `is_focused()`, `set_focused()`, `can_focus()`, `tab_order()`
- **`FocusManager<T>`** (in `src/state/focus.rs`) -- manages focus across components using arbitrary `T: Clone + Eq + Hash`

`FocusManager` does not consume or produce `Focusable` implementors. It tracks focus state independently via its own `elements: Vec<T>` and `focused: Option<usize>`. This means implementing `Focusable` on a component doesn't automatically integrate with `FocusManager`.

### 2. Event handler signature inconsistency

Free functions that handle keyboard events use inconsistent parameter ordering and return types:

| Pattern | Components |
|---------|-----------|
| `fn handle_*_key(state, key, ...) -> Option<Action>` | scrollable_content, split_pane, tab_view, accordion |
| `fn handle_*_key(key, state, ...) -> Option<Action>` | breadcrumb, context_menu, menu_bar |
| `fn handle_*_key(key, state) -> Option<Action>` | select |
| `fn handle_*_key(state, key) -> bool` | diff_viewer |
| `fn handle_*_key(key, state, items) -> Option<Action>` | hotkey_dialog |

The `Container` trait defines `handle_key(&self, key, state) -> EventResult` but no component follows this signature.

### 3. Click region ownership split

Click regions live in different places across components:

- **In widget struct:** Button stores `ClickRegion` data in the widget
- **In state struct:** PopupDialog stores `ClickRegionRegistry` in `DialogState`
- **In free function:** TabView, SplitPane pass `&mut ClickRegionRegistry` to render functions
- **Inline:** ContextMenu, MenuBar construct `ClickRegion` during event handling

This makes a unified `Clickable` trait difficult because there's no consistent owner.

---

## Recommendations

### Short-term (low risk)

1. **Implement `Focusable` on all components with `focused: bool` + `FocusId`:**
   Button, CheckBox, Input, TextArea, Select. These already have all the data; they just need the trait impl. `ScrollableContent` is the most egregious -- it literally duplicates the trait API.

2. **Standardize event handler signatures** to `(state, key, ...) -> Option<Action>` (state-first, matching the majority pattern).

### Medium-term (design changes)

3. **Redesign `Clickable` trait** to work with `ClickRegionRegistry` pattern:
   ```rust
   trait Clickable {
       type ClickAction: Clone;
       fn click_registry(&self) -> &ClickRegionRegistry<Self::ClickAction>;
   }
   ```
   Or deprecate the trait and standardize on `ClickRegionRegistry` as the sole click API.

4. **Bridge `Focusable` and `FocusManager`**: Add a method to `FocusManager` that can query `Focusable` trait objects, e.g.:
   ```rust
   impl FocusManager {
       fn register_focusable(&mut self, component: &dyn Focusable) { ... }
   }
   ```

### Long-term (architectural)

5. **Decide on Container pattern**: Either:
   - Redesign `Container`/`PopupContainer` to match the widget+state+free-function pattern the codebase actually uses, or
   - Deprecate the traits and standardize the free-function approach with consistent signatures.

6. **Add `FocusId` to all interactive components** that currently lack it (MenuBar, Breadcrumb, ScrollableContent, Accordion, DiffViewer) to enable full Tab navigation coverage.

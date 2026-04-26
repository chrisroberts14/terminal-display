# terminal-display

A Rust crate for building terminal UIs.

## Quick start

```toml
[dependencies]
terminal-display = { path = "." }
```

```rust
use std::{thread, time::Duration};
use terminal_display::{
    Block, Bordered, Color, HStack, Terminal, Text, VStack, WidgetExt, span, style,
};

fn main() {
    let terminal = Terminal::new().expect("failed to init terminal");
    let handle = terminal.run();

    let mut cpu = 0u32;
    let mut mem = 100u32;

    loop {
        handle.render(move |frame| {
            let area = frame.area();

            // Outer border with title
            let outer = Block::new().title("Stats");
            let inner = outer.inner(area);
            frame.render(outer, area);

            // Two bordered columns side by side
            frame.render(
                HStack::new(vec![
                    Bordered {
                        block: Block::new().title("Core 0"),
                        child: VStack::new(vec![
                            Text::raw(format!("CPU: {}%", cpu)).fixed(1),
                            Text::from(vec![
                                span!("MEM: "),
                                span!(format!("{}%", mem), style!(fg = Color::Red)),
                            ])
                            .fixed(1),
                        ]),
                    }
                    .fill(),
                    Bordered {
                        block: Block::new().title("Core 1"),
                        child: VStack::new(vec![
                            Text::raw(format!("CPU: {}%", cpu)).fixed(1),
                            Text::from(vec![
                                span!("MEM: "),
                                span!(format!("{}%", mem), style!(fg = Color::Blue)),
                            ])
                            .fixed(1),
                        ]),
                    }
                    .fill(),
                ]),
                inner,
            );
        });

        cpu += 1;
        mem = mem.saturating_sub(1);
        thread::sleep(Duration::from_millis(500));
    }
}
```

## Concepts

### Widgets

| Widget | Description |
|--------|-------------|
| `Text::raw(s)` | Single line of plain text |
| `Text::from(vec![span!(...)])` | Line built from styled spans |
| `VStack::new(children)` | Vertical stack, children sized by constraints |
| `HStack::new(children)` | Horizontal stack, children sized by constraints |
| `Block::new().title("…")` | Border box with optional title |
| `Bordered { block, child }` | Any widget wrapped in a `Block` border |
| `Centered { child }` | Any widget centred in its allocated area |
| `Spinner::new(style, style!)` | Animated loading indicator |
| `ProgressBar::new(value, total)` | Filled progress bar with percentage |
| `Gauge::new(value)` | Elliptical ring gauge filling clockwise from the top |
| `Divider` | Horizontal or vertical separator line |
| `Padding::all(n, child)` | Insets a child widget's render area uniformly |
| `Padding::axes(h, v, child)` | Insets with separate horizontal / vertical amounts |
| `Padding::new(t, r, b, l, child)` | Insets with per-side control |
| `Popup::new(background, overlay)` | Renders a background with an overlay centred on top |
| `Table::new(constraints, rows)` | Column grid with optional headers and selected-row highlight |

`Centered` and `Popup` work with any widget that implements `natural_size()` — `Spinner`, `Text`, `Bordered`, and `Padding` all do. Composition is additive:

```rust
// A 3×3 bordered spinner centred in the terminal
Centered {
    child: Bordered {
        block: Block::new().title("Loading"),
        child: Spinner::new(SpinnerStyle::Dots, style!(bold)),
    },
}
```

### Layout constraints

Attach a constraint to any widget using `WidgetExt`:

```rust
widget.fixed(3)       // exactly 3 rows / columns
widget.fill()         // share remaining space equally with other Fill siblings
widget.ratio(1, 3)    // one third of the available space
```

### Styled text

```rust
span!("plain text")
span!("coloured", style!(fg = Color::Red))
span!("bold red", style!(fg = Color::Red, bold))
```

### Terminal resize

Resize events are handled automatically. The render thread clears the screen and repaints on the next `render` call.

### Shutdown

```rust
handle.shutdown(); // restores the terminal and exits the render thread
```
# Task: M5 - Error Handling, Dark Mode, and Polish

**Created:** 2025-10-07  
**Assignee:** TBD  
**Priority:** Medium  
**Status:** Backlog

---

## Description

Polish the MVP with comprehensive error handling, dark mode toggle, accessibility improvements, and visual refinements. Ensure the app handles edge cases gracefully and provides helpful feedback to users.

---

## Category

- [ ] **Safety & Performance**
- [ ] **Privacy & Security**
- [x] **Protocol Compliance** (Error messages reference ATproto issues)
- [x] **UI/UX** (Dark mode, accessibility, polish)
- [ ] **Community & Docs**
- [ ] **Infrastructure**

---

## Acceptance Criteria

- [ ] Dark mode toggle functional (switches between light/dark themes)
- [ ] System preference detected (prefers-color-scheme)
- [ ] Error messages user-friendly with remediation suggestions
- [ ] Empty states (no records, loading, error) properly designed
- [ ] Keyboard navigation fully functional (tab, enter, arrow keys)
- [ ] Screen reader tested (NVDA or VoiceOver)
- [ ] WCAG 2.1 AA compliance verified (contrast ratios, ARIA labels)
- [ ] Loading skeletons or spinners for async operations
- [ ] Animations smooth (CSS transitions < 300 ms)
- [ ] Mobile responsive (tested on iOS Safari, Android Chrome)

---

## Constitution Check

Does this task impact any constitutional principles? If yes, document:

- **Principle(s) affected**: 
  - Clarity & Discoverability in UI (core UX improvements)
- **Compliance notes**: 
  - Error messages explain ATproto errors in plain language
  - Accessibility ensures inclusive access (blind/low-vision users)
  - Dark mode reduces eye strain for long sessions

---

## Technical Details

### Dark Mode Implementation

```rust
// Detect system preference
let prefers_dark = window()
    .match_media("(prefers-color-scheme: dark)")
    .ok()
    .flatten()
    .map(|mq| mq.matches())
    .unwrap_or(false);

// Toggle function
let dark_mode = create_rw_signal(prefers_dark);
let toggle_dark_mode = move |_| {
    dark_mode.update(|mode| *mode = !*mode);
    // Update localStorage
    let mut prefs = storage.load_preferences().unwrap();
    prefs.dark_mode = dark_mode.get();
    storage.save_preferences(&prefs).ok();
    // Update DOM class
    document().body().class_list().toggle("dark").ok();
};
```

### CSS Dark Mode

```css
/* Light mode (default) */
:root {
  --bg-color: #ffffff;
  --text-color: #1a1a1a;
  --border-color: #e5e5e5;
}

/* Dark mode */
body.dark {
  --bg-color: #1a1a1a;
  --text-color: #e5e5e5;
  --border-color: #333333;
}

body {
  background-color: var(--bg-color);
  color: var(--text-color);
  transition: background-color 0.2s, color 0.2s;
}
```

### Error Handling Strategy

| Error | User-Friendly Message | Remediation |
|-------|----------------------|-------------|
| Handle not found | "Handle 'xyz' not found. Check spelling or try a DID instead." | Suggest DID format |
| Network timeout | "Request timed out. Check your internet connection." | Retry button |
| Invalid DID format | "Invalid DID format. Expected 'did:plc:...' or 'did:web:...'" | Link to DID docs |
| 429 Rate Limited | "Too many requests. Please wait {retry_after} seconds." | Show countdown timer |
| 500 Server Error | "ATproto server error. Try again later or contact support." | Link to status page |

### Empty States

```rust
#[component]
pub fn EmptyState(message: String, icon: String) -> impl IntoView {
    view! {
        <div class="empty-state">
            <div class="icon">{icon}</div>
            <p>{message}</p>
        </div>
    }
}

// Usage
<Show when=move || records().is_empty() fallback=|| view! { <RecordList /> }>
    <EmptyState message="No records found.".to_string() icon="📭".to_string() />
</Show>
```

### Accessibility Checklist

- [ ] All interactive elements focusable (keyboard nav)
- [ ] Focus indicators visible (outline or ring)
- [ ] ARIA labels on icon-only buttons (e.g., `aria-label="Toggle dark mode"`)
- [ ] `aria-expanded` on collapsible tree nodes
- [ ] `role="status"` on loading spinners (screen reader announces)
- [ ] Color contrast ratios:
  - Normal text: 4.5:1 minimum
  - Large text: 3:1 minimum
  - Interactive elements: 3:1 minimum
- [ ] Skip links for screen readers (skip to main content)

### Mobile Responsiveness

- Breakpoints: 640px (mobile), 768px (tablet), 1024px (desktop)
- Use flexbox/grid for fluid layouts
- Touch targets ≥ 44x44 px (iOS/Android guidelines)
- Test on real devices (not just browser DevTools)

### Testing approach

- **Dark Mode**: Toggle, reload page, verify persistence
- **Accessibility**: Run axe-core DevTools extension
- **Keyboard Nav**: Unplug mouse, navigate entire app with keyboard
- **Screen Reader**: Test with VoiceOver (macOS) or NVDA (Windows)
- **Mobile**: Test on iPhone SE (small screen), iPad (tablet), Android phone

---

## Estimates

- **Effort**: Medium (2-8h)
- **Risk**: Low (mostly CSS and UX tweaks)

---

## Notes

- **Animation Performance**: Use `transform` and `opacity` for animations (GPU-accelerated). Avoid animating `width` or `height`.
- **Dark Mode Persistence**: Save to localStorage; apply on page load before first render to avoid flash.
- **Error Boundaries**: Wrap components in Leptos `ErrorBoundary` to catch panics.
- **Loading Skeletons**: Use placeholder boxes with shimmer animation (better UX than spinners).



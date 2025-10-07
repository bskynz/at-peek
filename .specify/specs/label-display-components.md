# Specification: Label Display Components

**Version:** 0.1.0  
**Author(s):** Project Maintainers  
**Created:** 2025-10-07  
**Status:** Draft

---

## Summary

Defines the UI components for displaying content moderation labels in at-peek. Labels are organized by category (adult content, violence, spam, etc.) and displayed as visual badges with color-coding, tooltips, and metadata (source, timestamp).

---

## Constitution Alignment

| Principle | Compliance Notes |
|-----------|------------------|
| Rust Safety & Performance | Leptos components with reactive signals; efficient rendering |
| User Privacy by Design | No tracking of which labels users view |
| Protocol Fidelity & Data Accuracy | Labels displayed exactly as returned from labeler services |
| Clarity & Discoverability in UI | Clear categorization, tooltips, color-coding for quick understanding |
| Open Source Transparency & Community | Component code is auditable and well-documented |

---

## Label Categories

Labels are grouped into semantic categories for easier understanding:

### 1. Adult Content
- `porn` - Pornographic content
- `sexual` - Sexually suggestive
- `nudity` - Nudity (artistic or otherwise)

**Color:** Red/Pink  
**Icon:** 🔞

### 2. Violence & Gore
- `graphic-media` - Graphic violence
- `gore` - Gore or extreme violence

**Color:** Orange/Red  
**Icon:** ⚠️

### 3. Spam & Low Quality
- `spam` - Spam content

**Color:** Yellow  
**Icon:** 🚫

### 4. Hate & Harassment
- `hate` - Hateful or discriminatory content

**Color:** Dark Red  
**Icon:** 🛑

### 5. Moderation Actions
- `!hide` - Hidden from feeds
- `!warn` - Warning required before viewing
- `!blur` - Blurred in feeds

**Color:** Gray/Blue  
**Icon:** 👁️

### 6. Other/Custom
- Any labels not in the above categories

**Color:** Gray  
**Icon:** 🏷️

---

## Component Specifications

### LabelBadge

Visual badge for a single label.

```rust
#[component]
pub fn LabelBadge(label: Label) -> impl IntoView {
    let category = get_label_category(&label.val);
    let color_class = category.color_class();
    let tooltip = get_label_description(&label.val);
    
    view! {
        <div 
            class=format!("label-badge {}", color_class)
            title=tooltip
        >
            <span class="label-icon">{category.icon()}</span>
            <span class="label-value">{&label.val}</span>
            <span class="label-meta">
                "from " {shorten_did(&label.src)}
                " • " {format_timestamp(&label.cts)}
            </span>
        </div>
    }
}
```

**Visual Design:**
```
┌─────────────────────────────────────────┐
│ 🔞 porn                                 │
│    from did:plc:abc...xyz               │
│    • 2024-10-07 12:34 UTC               │
└─────────────────────────────────────────┘
  ↑         ↑              ↑
 icon    label value   metadata
```

---

### LabelCategoryGroup

Groups labels by category with collapsible sections.

```rust
#[component]
pub fn LabelCategoryGroup(
    category: LabelCategory,
    labels: Vec<Label>
) -> impl IntoView {
    let expanded = create_rw_signal(true); // Start expanded
    
    view! {
        <div class="label-category-group">
            <div 
                class="category-header"
                on:click=move |_| expanded.update(|e| *e = !*e)
            >
                <span class="toggle-icon">
                    {move || if expanded() { "▼" } else { "▶" }}
                </span>
                <span class="category-name">{category.name()}</span>
                <span class="label-count">{labels.len()}</span>
            </div>
            
            <Show when=move || expanded()>
                <div class="category-labels">
                    <For
                        each=move || labels.clone()
                        key=|label| format!("{}:{}", label.val, label.cts)
                        let:label
                    >
                        <LabelBadge label=label />
                    </For>
                </div>
            </Show>
        </div>
    }
}
```

---

### LabelViewer

Main component that displays all labels with categories.

```rust
#[component]
pub fn LabelViewer(state: AppState) -> impl IntoView {
    let categorized_labels = create_memo(move |_| {
        state.labels.get().map(|collection| {
            categorize_labels(&collection.labels)
        })
    });
    
    view! {
        <div class="label-viewer">
            <Show
                when=move || state.labels.get().is_some()
                fallback=|| view! { <EmptyState /> }
            >
                {move || categorized_labels.get().map(|categories| {
                    view! {
                        <div class="label-categories">
                            <For
                                each=move || categories.iter()
                                key=|(cat, _)| cat.clone()
                                let:item
                            >
                                <LabelCategoryGroup 
                                    category=item.0.clone()
                                    labels=item.1.clone()
                                />
                            </For>
                        </div>
                    }
                })}
            </Show>
        </div>
    }
}
```

---

### EmptyState

Displayed when no labels are found.

```rust
#[component]
pub fn EmptyState() -> impl IntoView {
    view! {
        <div class="empty-state">
            <div class="empty-icon">✅</div>
            <h3>"No moderation labels found"</h3>
            <p>"This user or post has no labels applied by moderation services."</p>
        </div>
    }
}
```

---

## Helper Functions

### Label Categorization

```rust
pub enum LabelCategory {
    AdultContent,
    Violence,
    Spam,
    Hate,
    ModerationAction,
    Other,
}

impl LabelCategory {
    pub fn color_class(&self) -> &str {
        match self {
            Self::AdultContent => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
            Self::Violence => "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200",
            Self::Spam => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
            Self::Hate => "bg-red-200 text-red-900 dark:bg-red-800 dark:text-red-100",
            Self::ModerationAction => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
            Self::Other => "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200",
        }
    }
    
    pub fn icon(&self) -> &str {
        match self {
            Self::AdultContent => "🔞",
            Self::Violence => "⚠️",
            Self::Spam => "🚫",
            Self::Hate => "🛑",
            Self::ModerationAction => "👁️",
            Self::Other => "🏷️",
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            Self::AdultContent => "Adult Content",
            Self::Violence => "Violence & Gore",
            Self::Spam => "Spam",
            Self::Hate => "Hate & Harassment",
            Self::ModerationAction => "Moderation Actions",
            Self::Other => "Other Labels",
        }
    }
}

pub fn get_label_category(val: &str) -> LabelCategory {
    match val {
        "porn" | "sexual" | "nudity" => LabelCategory::AdultContent,
        "graphic-media" | "gore" => LabelCategory::Violence,
        "spam" => LabelCategory::Spam,
        "hate" => LabelCategory::Hate,
        s if s.starts_with('!') => LabelCategory::ModerationAction,
        _ => LabelCategory::Other,
    }
}

pub fn get_label_description(val: &str) -> &str {
    match val {
        "porn" => "Pornographic content",
        "sexual" => "Sexually suggestive content",
        "nudity" => "Nudity (artistic or otherwise)",
        "graphic-media" => "Graphic violence or disturbing imagery",
        "gore" => "Extreme violence or gore",
        "spam" => "Spam or low-quality content",
        "hate" => "Hateful or discriminatory content",
        "!hide" => "Hidden from feeds by moderators",
        "!warn" => "Warning required before viewing",
        "!blur" => "Blurred in feeds",
        _ => "Custom moderation label",
    }
}

pub fn shorten_did(did: &str) -> String {
    if did.len() > 20 {
        format!("{}...{}", &did[..12], &did[did.len()-5..])
    } else {
        did.to_string()
    }
}

pub fn format_timestamp(iso8601: &str) -> String {
    // Parse and format as human-readable
    // E.g., "2024-10-07 12:34 UTC"
    chrono::DateTime::parse_from_rfc3339(iso8601)
        .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string())
        .unwrap_or_else(|_| iso8601.to_string())
}
```

---

## Styling (TailwindCSS)

```css
.label-badge {
  @apply flex flex-col gap-1 p-3 rounded-lg border-2 transition-all;
}

.label-badge:hover {
  @apply shadow-md scale-105;
}

.label-icon {
  @apply text-2xl;
}

.label-value {
  @apply font-semibold text-lg;
}

.label-meta {
  @apply text-sm opacity-70;
}

.category-header {
  @apply flex items-center gap-2 p-2 cursor-pointer rounded hover:bg-gray-100 dark:hover:bg-gray-800;
}

.toggle-icon {
  @apply text-gray-500;
}

.category-name {
  @apply font-semibold text-lg flex-1;
}

.label-count {
  @apply bg-gray-200 dark:bg-gray-700 px-2 py-1 rounded-full text-sm;
}

.empty-state {
  @apply flex flex-col items-center justify-center gap-4 p-12 text-center;
}

.empty-icon {
  @apply text-6xl;
}
```

---

## Accessibility

- All interactive elements (category headers, badges) are keyboard-accessible
- `aria-expanded` on category headers
- `title` attributes on badges for screen readers
- Color is not the only indicator (icons + text labels)
- Sufficient contrast ratios (WCAG 2.1 AA)

---

## Testing Plan

- [ ] Unit tests for label categorization logic
- [ ] Visual regression tests for label badges
- [ ] Accessibility audit (axe-core)
- [ ] Manual test: keyboard navigation through categories
- [ ] Screen reader test: VoiceOver/NVDA
- [ ] Test with various label combinations (empty, single category, all categories)

---

## References

- [Bluesky Label Definitions](https://docs.bsky.app/docs/advanced-guides/moderation)
- [ATproto Label Spec](https://atproto.com/specs/label)



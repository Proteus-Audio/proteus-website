# Proteus Website Style Guide

## Objective
Build a marketing/documentation site that visually aligns with the Proteus Author frontend while staying clean, fast, and easy to extend in Dioxus.

## Visual Direction
- Mood: modern studio tool, calm and technical.
- Contrast model: light surfaces with clear borders/shadows.
- Accent strategy: ocean-blue primary with warm analog-gold highlight.

## Core Tokens
- Primary action color: `#1978A4`
- Primary hover/deep: `#125F82`
- Analog accent: `#E0C25C`
- Background gradient: `#EDF3F7` to `#D4E6F1`
- Main text: `#1F2B36`
- Secondary text: `#5D7282`
- Border: `#D5DEE3`
- Radius: `14px`
- Shadow: `0 16px 40px rgba(20, 40, 56, 0.12)`

## Typography
- Primary family: `Inter` (400/600/800).
- Accent/display family: `Silkscreen` for brand kicker/title tags.
- Text style:
  - Body: highly readable neutral sans
  - Kicker/meta labels: uppercase pixel-style accent

## Components
- Header:
  - Frosted light panel
  - Left-aligned icon lockup + brand text
  - Right-aligned nav links in rounded pill style
- Hero:
  - Two-column panel on desktop, stacked on mobile
  - Left: product statement + CTA buttons
  - Right: Proteus icon showcase card
- Cards:
  - White surfaces with subtle gradient and strong border edges
  - Concise heading + one supporting paragraph
- Content Panel:
  - Shared container for About/Downloads prose sections
- Downloads Cards:
  - One card per tool/repository, disabled CTA until links are available

## Motion
- Keep animation sparse and meaningful.
- Use short, gentle entrance transitions (`~340ms`) for hero/panel load.
- Avoid decorative perpetual animation.

## Layout Rules
- Max content width: `1120px` container.
- Primary gap rhythm: `1rem`.
- Responsive breakpoints:
  - `900px`: stack multi-column panels
  - `680px`: stack header regions, tighten spacing

## Asset Usage
- Use `/assets/images/icon.png` (provided Proteus wave icon) in:
  - Header brand lockup
  - Landing page hero artwork
- Use local font assets under `/assets/fonts/`.

## Writing Tone
- Technical but approachable.
- Keep claims concrete and tied to procedural audio use cases.
- Prefer short paragraphs over long blocks.

## Future Expansion
When adding new sections (Docs, FAQ, Contact), preserve this token set and component language to maintain visual continuity.

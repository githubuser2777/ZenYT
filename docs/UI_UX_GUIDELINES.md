# UI/UX Design System

ZenYT aims to look like a premium, paid application. We achieve this through meticulous attention to detail in our Vanilla CSS.

## 1. Design Philosophy
- **Minimalism & Focus**: The main screen should have a single, prominent input field for the URL. No clutter.
- **Dark Mode Aesthetic**: Deep, rich dark backgrounds reduce eye strain and look modern.
- **Glassmorphism**: Modals and sidebars should use semi-transparent backgrounds with background blur.

## 2. CSS Variables (The Palette)
Create a `global.css` file defining these tokens:
```css
:root {
  /* Backgrounds */
  --bg-primary: #0F1115;      /* Deep dark background */
  --bg-secondary: #1A1D24;    /* Slightly lighter for cards/panels */
  --bg-glass: rgba(26, 29, 36, 0.6);

  /* Accents */
  --accent-primary: #6C5CE7;  /* Premium Purple */
  --accent-hover: #8172F0;
  --accent-success: #00B894;  /* Green for completed downloads */
  --accent-danger: #D63031;   /* Red for errors/cancels */

  /* Text */
  --text-primary: #FFFFFF;
  --text-secondary: #A0AAB2;
  --text-muted: #6B7280;

  /* Geometry */
  --border-radius-sm: 6px;
  --border-radius-md: 12px;
  --border-radius-lg: 20px;
}
```

## 3. Typography
- **Primary Font**: `Inter`, `Roboto`, or system default sans-serif.
- **Weights**: Use 400 for regular text, 500 for buttons, 600/700 for headings.
- **Line Height**: 1.5 for readability.

## 4. Micro-Interactions
- **Buttons**: Every clickable element must have a `transition: all 0.2s ease;`. On hover, buttons should slightly lighten and lift (`transform: translateY(-1px)`). On click (active state), they should press down.
- **Inputs**: The URL input should have a subtle glowing box-shadow when focused: `box-shadow: 0 0 0 2px var(--accent-primary);`
- **Progress Bars**: The width of the progress bar must transition smoothly. Use `transition: width 0.3s linear;` to prevent jerky updates from yt-dlp parsing.

## 5. Layout Structure
- **Top Bar**: Custom window dragging area (Tauri `data-tauri-drag-region`) to eliminate the default OS title bar, creating a seamless app window.
- **Main Area**: Centered URL input.
- **Queue Panel**: A slide-up or side panel showing active and past downloads, styled as individual cards.
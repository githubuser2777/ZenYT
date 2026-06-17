# UI/UX Guidelines

ZenYT must feature a stunning, premium interface. We want users to be wowed at first glance.

## 1. Styling Strategy
- **Strictly Vanilla CSS**: We use standard CSS (`.css` files) to maintain absolute control over styling. No Tailwind.
- **CSS Variables (Custom Properties)**: Extensively use CSS variables for colors, spacing, and typography to maintain a cohesive design system.

## 2. Aesthetics
- **Dark Mode by Default**: Implement a sleek, modern dark mode using deep grays/blacks and vibrant accent colors (e.g., neon purples, electric blues).
- **Glassmorphism**: Use subtle translucent backgrounds with `backdrop-filter: blur()` for overlays, sidebars, and modals.
- **Gradients**: Use smooth, well-blended linear or radial gradients. Avoid harsh or generic colors.

## 3. Typography
- Use modern sans-serif fonts from Google Fonts (e.g., Inter, Roboto, or Outfit).
- Maintain a clear hierarchy (H1 -> H6).

## 4. Interactivity & Motion
- **Micro-animations**: Elements should respond to hover, active, and focus states.
- Smooth transitions for layout changes and state updates.
- Real-time visual feedback for downloads (progress bars, speed metrics).

# AI Assistant & Developer Guidelines

This document provides context and rules for AI coding assistants (like GitHub Copilot, Cursor, or Gemini) and human developers working on ZenYT.

## 1. "Thoáng" (Flexible) Mindset
- **Pragmatism over Dogma**: We prefer code that works reliably, reads easily, and performs well over code that strictly follows complex design patterns.
- **Refactoring**: If you see a messy function, you are encouraged to refactor it, provided you do not break the core functionality. Do not ask for permission for minor cleanups.
- **Creative Freedom**: If a feature can be implemented in a better, more visually appealing way than requested, implement the better version.

## 2. Code Quality & Standards
- **Vanilla CSS Priority**: **DO NOT** use TailwindCSS, Bootstrap, or Material UI. We rely on handcrafted Vanilla CSS. Use CSS variables in a `root.css` file to maintain a design system.
- **Rust Backend**: Keep the Rust code safe. Handle `Result` and `Option` gracefully. Do not use `.unwrap()` in production paths; use proper error handling to pass error messages back to the UI.
- **React Frontend**: Use functional components and Hooks. Keep components small and focused.

## 3. UI/UX Focus
- The user experience is paramount. When writing frontend code, always consider:
  - Is there a loading state?
  - Are errors displayed gracefully (e.g., Toast notifications instead of `alert()`)?
  - Do buttons have hover and active states?

## 4. Working with `yt-dlp`
- Always assume `yt-dlp` commands might fail (e.g., due to geographic restrictions, deleted videos). Ensure the Rust backend catches standard error output (`stderr`) and sends it to the frontend for the user to see.
- When parsing `yt-dlp` JSON output, be aware that not all fields are guaranteed to exist. Use optional chaining in JS or `Option<T>` in Rust.
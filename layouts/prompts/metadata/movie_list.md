PAGE TYPE:
Movie list

LAYOUT:
- top nav via include
- Left sidebar nav via include
- Main content fills remaining space
- Max width inside main

STYLE:
- theme toggle (light and dark mode)
- Compact, media-center UI
- Tailwind CSS v4
- askama templates

COMPONENTS:
- Movie cards (grid)  
- Poster image, title, alt title
- add clickable buttons for genres
- MPAA rating, runtime
- Star rating + inline action icons
- Add clickable icons for favorite, watched, good, bad and trash after the star rating
- Availability after the runtime, add a green/yellow/red text for available, requested, unavailable

INTERACTIVITY:
- Alpine.js for icon toggles
- debounce
- use x-data for the state logic
- No pagination bar
- async image loading along with skeleton loaders

CONSTRAINTS:
- Semantic HTML
- Accessible (aria where appropriate)
- svg instead of emoji

TECHSTACK:
- Rust
- Askama
- HTML5
- Tailwindcss v4
- Axum
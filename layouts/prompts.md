# metadata list for movie
design and generate html5 and tailwindcss v4 code for list of movies showing the poster with title, alternate title, mpaa rating, runtime.
Add clickable icons for favorite, watched, good, terrible after the star rating.
after the runtime, add a green/yellow/red text (text-right) for available or not.
svg instead of emoji.

# movie detail page - metadata
design and generate html5 and tailwindcss v4 code for movie detail page incluing cast carousel and crew carousel with left/right arrows.
these carousels will be link clickable.
Allow for tagline and alternate title, mpaa rating, runtime.
Clickable genre buttons.
Use alpinejs for interactivity.
Do not include pagination bar.
Add accordian review section that is collapsed by default and expands on click to show user reviews.
Add clickable icons for favorite, watched, good, terrible after the star rating. 
svg instead of emoji.
Backend-ready Alpine store.

# metadata list for tv show
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity.
Do not include pagination bar.
svg instead of emoji.

# tv show detail page - metadata
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.

# book detail page - metadata
design and generate html5 and tailwindcss v4 code for books and magazines showing cover with title, alternate title, pages.
Add clickable icons for favorite, watched, good, terrible after the star rating. add a green/yellow/red text (text-right) for available or not.
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.

# music detail page - metadata
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.

# sports detail page - metadata
# prob have to have many breakdowns
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.

# game detail page - metadata
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.

# game system detail page - metadata
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.

# videoplayback form for WEB playback
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.\

# movie collection - metadata
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.

# music video detail page - metadata
design and generate html5 and tailwindcss v4 code for **********
Use alpinejs for interactivity. Do not include pagination bar.
svg instead of emoji.

# main user landing page
design and generate html5 and tailwindss v4 code for multimedia landing page showing movies, tv show, music in different sections. Use alpinejs for interactivity. Do not include pagination bar. svg instead of emoji. no upper navbar. no explore new media.

# to convert html templates
convert following to alpinejs, tailwindcss v4 and html5.  svg instead of emoji.






Lazy Loading + Skeleton Loaders = 🧠💥
Best combo:
Skeleton shows instantly
Image loads when needed
No layout jump
Feels instant
This is exactly how Netflix / TMDB / Prime do it.


Optimize your movie page images for Core Web Vitals
Add smart eager/lazy logic to your carousels
Show a perfect LCP-friendly poster setup

<img
  src="poster.jpg"
  loading="eager"
  fetchpriority="high"
  decoding="async"
>

PAGE TYPE:
Movie list

LAYOUT:
- Left sidebar (nav)
- Main content fills remaining space
- Max width inside main

STYLE:
- Dark theme
- Compact, media-center UI
- Tailwind CSS v4

COMPONENTS:
- Movie cards (grid)
- Poster, title, alt title
- MPAA rating, runtime
- Star rating + inline action icons
- Availability text (right-aligned)

INTERACTIVITY:
- Alpine.js for icon toggles
- No pagination bar

CONSTRAINTS:
- Semantic HTML
- Accessible (aria where appropriate)



I assume:
Mobile-first
Flexbox / Grid only (no hacks)
Semantic HTML
Tailwind utilities only (no inline CSS)
Alpine = light behavior, not heavy state

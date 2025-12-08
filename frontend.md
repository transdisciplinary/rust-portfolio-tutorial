# 🎨 Frontend Guide (Askama + HTMX)


We avoid heavy frameworks like React/Next.js. Instead, we use **Server-Side Rendering (SSR)** mixed with **HTMX** for interactivity.

## 1. Templates (Askama)

Files in `templates/` are compiled into the Rust binary. This makes them extremely fast.

```html
<!-- dashboard.html -->
{% extends "base.html" %}
{% block content %}
    <h1>{{ project.title }}</h1>
{% endblock %}
```

When Rust builds, it checks if `project.title` actually exists. No more runtime "undefined" errors!

## 2. Interactivity (HTMX)

For things like reordering content blocks or deleting items, we use **HTMX**. It allows us to do AJAX requests directly in HTML.

```html
<!-- Example: Delete a block -->
<form method="POST" action="/admin/blocks/delete/123" 
      hx-boost="false" class="confirm-delete">
    <button type="submit">Delete</button>
</form>
```

We primarily use standard form submissions (`POST`) for simplicity, but HTMX is available for dynamic updates (like drag-and-drop reordering).

## 3. Styling (Vanilla CSS)

We use a CSS Layer architecture in `static/css/styles.css`:

1.  **Base**: Resets and variables (`--clr-bg`, `--clr-text`).
2.  **Admin**: Specific styles for the CMS.
3.  **Components**: Buttons, Inputs, Modals.

## 4. Client-Side Image Optimization

To save bandwidth, we use `browser-image-compression` in `templates/base.html`.

1.  User selects a 5MB JPEG.
2.  JS intercepts the upload.
3.  Compresses it to a <1MB WebP.
4.  Sends the WebP to the server.

# Module 04: Building the Frontend

We are building a "No-Framework" frontend. That means no React, no Vue, no massive JS bundles. Just HTML and CSS.

## 1. Askama Templates
We write HTML files in `templates/` with special syntax.
Rust compiles these into the code.

```html
<!-- templates/project.html -->
{% extends "base.html" %}

{% block content %}
    <h1>{{ project.title }}</h1>
    <div class="gallery">
        {% for block in blocks %}
            {% if block.kind == "image" %}
                <img src="{{ block.content }}" />
            {% endif %}
        {% endfor %}
    </div>
{% endblock %}
```

## 2. HTMX for Interactivity
Instead of writing complex fetch logic, we use HTMX attributes.

**Example: Reordering Blocks**
```html
<div class="blocks-list" hx-post="/admin/blocks/reorder" hx-trigger="end">
    <!-- draggable items -->
</div>
```
When you drag and drop an item, HTMX automatically sends a POST request to the server with the new order.

## 3. CSS Architecture
We use modern CSS Layers in `static/css/styles.css`.
*   `@layer base`: Resets and fonts.
*   `@layer admin`: Specific layout for the CMS.
*   `@layer components`: Buttons (`.btn`), Cards, Inputs.

## 4. Client-Side Image Optimization
This is a cool trick. We want to save server bandwidth.
In `templates/base.html`, we intercept the form upload:

```javascript
async function uploadFile(file) {
    // 1. Compress Image in Browser
    const compressed = await imageCompression(file, { maxSizeMB: 1 });
    
    // 2. Send compressed file to Server
    formData.append('file', compressed);
    xhr.send(formData);
}
```
This forces all uploads to be efficient WebP images before they even leave the user's computer.


to run the server and automatically update whena a change is made

`cargo watch -x run`

better divide the 2 css files

# Mauro's Portfolio Website

A personal portfolio website built with Rust

## ✨ Features

- **Dark Theme**: Ocean-inspired design with turquoise accents
- **Blog System**: Markdown-powered blog with status tags (draft/complete)
- **Experience Timeline**: Professional background showcase
- **Responsive Design**: Mobile-friendly layout
- **Fast Performance**: Rust server

## 🛠 Tech Stack

- **Backend**: Rust with html/template
- **Styling**: CSS with Grid and Flexbox

## 📁 Project Structure

```
Mauro-website/
├── content/
│   └── blog/           # Markdown blog posts
├── static/
│   ├── css/           # Stylesheets
│   └── assets/        # Static images, CV
├── templates/         # HTML templates
├── parser/            # Markdown parsing logic
│    src/
│   └── main.rs        # Rust server
├── Cargo.toml         # Rust dependencies
└── Dockerfile         # Container configuration
```

## 🚀 Local Development

### Prerequisites

- Go 1.24+
- Git

### Setup

3. **Run the server**
   ```bash
   make dev
   ```

4. **Visit the site**
   Open [http://localhost:8080](http://localhost:8080)

## 📝 Adding Blog Posts

Create a new markdown file in `content/blog/` with the following format:

```markdown
# Your Post Title

*Published: July 1, 2025*
*Category: Technology*
*Status: draft*

Your blog post content here...
```

**Supported fields:**
- `Published`: Date in "Month Day, Year" format
- `Category`: Post category
- `Status`: `draft` or `complete` (shows colored tag)

## 🎨 Design System

### Colors
- **Background**: `#0d1b2a` (dark ocean blue)
- **Primary**: `#20b2aa` (turquoise)
- **Text**: `#eceff1` (light gray)
- **Secondary**: `#707070` (medium gray)

### Typography
- **Headers**: Lora (serif)
- **Body**: Lora (serif)
- **Code/Nav**: Inconsolata (monospace)
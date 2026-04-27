Portfolio project inspired by https://github.com/cfereday

# Mauro's Portfolio Website

A personal portfolio website built with Rust

deployes on leapcell available here: https://mauro-busso-portfolio.leapcell.app

## ✨ Features

<!-- - **Blog System**: Markdown-powered blog with status tags (draft/complete) -->

- **Experience Timeline**: Professional background showcase
- **Responsive Design**: Mobile-friendly layout
- **Fast Performance**: Rust server

## 🛠 Tech Stack

- **Backend**: Rust with html/template
- **Styling**: CSS with Grid and Flexbox

## 📁 Project Structure

```
Portfolio/
├── content/
│   └── blog/          # Markdown blog posts
├── static/
│   ├── css/           # Stylesheets
│   └── assets/        # Static images, CV
├── templates/         # HTML templates
├── parser/            # Markdown parsing logic for blogs
│    src/
│   └── main.rs        # Rust server
├── Cargo.toml         # Rust dependencies
```

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

_Published: July 1, 2025_
_Category: Technology_
_Status: draft_

Your blog post content here...
```

**Supported fields:**

- `Published`: Date in "Month Day, Year" format
- `Category`: Post category
- `Status`: `draft` or `complete` (shows colored tag)

## 🛠 Adding Projects

The portfolio page loads projects from GitHub using the repository topic `portfolio` for the user `maurobusso`.

To add a new project:

1. Go to the GitHub repository you want to add.
2. Open the repository settings and add the topic `portfolio`.
3. Make sure the repository belongs to the GitHub user `maurobusso`.

If the repo has the `portfolio` topic and is in the right user account, it will appear on the `/projects` page automatically.

> Note: The site searches GitHub with `user:maurobusso topic:portfolio fork:true`, so the repo must be owned by `maurobusso` and tagged with `portfolio`.

During development, I encountered an issue where repository data would intermittently fail to load. After investigation, I identified that unauthenticated requests to the GitHub Search API are strictly rate-limited to 60 requests per hour. To resolve this, I implemented an asynchronous caching layer using the moka crate. By storing API responses in memory with a 60-minute Time-To-Live (TTL), I significantly reduced the number of outbound requests, improved page load speeds, and ensured the site remains stable even under frequent refreshes.

To ensure reliability against GitHub's API rate limits, this project utilizes an in-memory cache. This ensures that the portfolio remains performant and avoids "blank" states caused by hitting API thresholds during high traffic or active development.

## Tech Stack:

Runtime: Tokio (Async Rust)

HTTP Client: Reqwest

Caching: Moka (High-performance concurrent cache)

Testing: Wiremock (HTTP mocking)

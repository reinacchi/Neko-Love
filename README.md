# Neko-Love API V4 (Rust / Axum Rewrite)

> A community-driven reimplementation of the original Neko-Love API, now written in Rust using the Axum web framework.

---

## 🌟 What is this?

This project is a **modern reimplementation of the original Neko-Love API**, which was once hosted at `neko-love.xyz`. It served random anime-style images like "neko", "hug", "kiss", and more — often used in Discord bots, anime projects, and other community tools.

The original API was written in Node.js using Koa. This version is a **fresh and solid base built with Rust and the Axum framework**, designed for others to easily clone, customize, and host themselves.

---

## 🔧 Goals

- Provide a **lightweight and fast** REST API to serve random images.
- Offer a **clean modular structure** for adding new routes, categories, and assets.
- **SFW only**: This version contains strictly Safe For Work content.

---

## ⚠️ Please Note

This API **is not hosted by the original author**.

> It is provided as an open-source base only. You are free to clone, modify, self-host, and integrate it into your own projects.

---

## 🚀 Quick Overview

- Each route returns a random image from a local folder (e.g. `/assets/neko/`)
- Example JSON response:

```json
{
  "id": "01",
  "url": "http://localhost:3030/images/neko/01.png"
}
```

- Images are served at `/images/<category>/<image>`

---

## 🚩 Why a new version?

> After the original API was shut down, several community members asked if they could bring it back. Unfortunately, the original source code was lost. This rewrite aims to provide a fresh, modern foundation.

- ✅ Easier to maintain
- ✅ Fast and lightweight (Rust + Axum)
- ✅ Clean structure for contributions

---

## 💻 Run Locally

### Requirements

Make sure you have **Rust installed** (version 1.76+ recommended):  
→ [Download Rust](https://www.rust-lang.org/tools/install)

---

### Installation

1. Clone the repository:

```bash
git clone https://github.com/Otaku17/Neko-Love.git
cd Neko-Love
```

2. Run the API:

```bash
cargo run
```

4. Add your images in the corresponding folders inside the `assets/` directory (e.g. `assets/neko/`, `assets/hug/`, etc.)

---

## 🤝 Contributing

Want to add a new image category?

1. Create a folder inside `assets/<name>`
2. Add your image files there

No extra routes need to be added to the code since they are managed automatically.

---

Thanks to everyone who used the original Neko-Love, and to all those who want to bring it back with a new twist ✨

## 🔍 Example API Call

To get a random image (returns `{ "url": "/images/neko/01.png" }`):

```sh
GET http://localhost:3030/api/v4/neko
```

To access the image directly (after receiving the URL from the JSON response):

```
http://localhost:3030/images/neko/01.png
```

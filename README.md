# ⚡ Satisfactory Calculator

A production-chain planner for [Satisfactory](https://www.satisfactorygame.com/).
Pick a target item, set your desired output rate, and get the full breakdown —
every machine, the total power, the raw resources you need to mine, any
byproducts you'll produce, and an alternate-recipe picker to optimize the chain.

The app runs **entirely in the browser** — the recipe solver is a JavaScript
port bundled into the frontend, so there's no backend to run. It deploys as a
static site to GitHub Pages.

### 🔗 Live demo

**https://jay37mack37.github.io/satisfactory-calculator/**

## Features

- **Full production-tree solver** — recursively walks the recipe tree from your
  target item all the way down to raw ore, returning every production step.
- **Alternate recipes** — ~90 alternate recipes included; swap any step to an
  alternate and re-run to see the updated chain.
- **Conveyor belt & pipeline presets** — one-click rate buttons for Mk.1–6 belts
  and Mk.1–3 pipes.
- **Visual production tree** — an indented, depth-aware view of the dependency
  chain alongside the classic flat table.
- **Summary stats** — total power (MW), production-step count, and raw-resource
  count at a glance.
- **Byproduct tracking** — byproducts are surfaced with their rate and source.
- **Item icons** — 66 item icons scraped from the Satisfactory wiki.
- **Responsive & polished** — glassmorphism dark UI that works on mobile.

## Tech stack

| Layer | Tech |
| --- | --- |
| Frontend | [Svelte 5](https://svelte.dev/) (runes) + [Vite 8](https://vite.dev/) |
| Recipe engine (in-browser) | Vanilla JS port in `frontend/src/lib/engine.js` |
| Recipe engine (reference) | Rust + [Axum](https://github.com/tokio-rs/axum) in `backend/` |
| Data | `recipes.json` (159 recipes) + `icon_map.json`, bundled into the frontend |
| Tests | [Vitest](https://vitest.dev/) (JS engine) · `cargo test` (Rust) |
| Deploy | GitHub Actions → GitHub Pages |

The frontend is self-contained and uses the bundled JS engine. The Rust backend
in `backend/` is the original reference implementation of the solver — it embeds
the same data and exposes the same calculation via a small REST API. You don't
need it to run the app, but it's handy for cross-checking and local development.

## Getting started

### Run the frontend (recommended)

```bash
cd frontend
npm install
npm run dev
```

Then open the URL Vite prints (default <http://localhost:5173>).

There are also convenience scripts at the repo root:

- `./start.sh` (Linux/macOS) or `start.bat` (Windows) — installs deps and
  launches the Vite dev server.

### Run the Rust backend (optional)

```bash
cd backend
cargo run
```

The server listens on <http://localhost:3000> and exposes:

- `GET /api/items` — sorted list of craftable items
- `GET /api/icons` — item → icon path map
- `GET /api/alternates` — items with more than one recipe
- `POST /api/calculate` — `{ item, rate, recipe_overrides }` → full result

## Testing

```bash
# JS engine unit tests (verifies the port mirrors the Rust solver)
cd frontend && npm test

# Rust backend tests
cd backend && cargo test
```

The Vitest suite hand-traces a `Rotor @ 10/min` chain (nodes, total power, raw
ores), an alternate-recipe override (`Alternate: Cast Screws`), and byproduct
accumulation (`Heavy Oil Residue` → `Polymer Resin`).

## Project structure

```
satisfactory-calculator/
├── .github/workflows/deploy.yml   # Build + deploy to GitHub Pages
├── frontend/
│   ├── src/
│   │   ├── App.svelte             # App shell + state
│   │   ├── app.css                # Design system (dark glassmorphism)
│   │   └── lib/
│   │       ├── engine.js          # JS port of the recipe solver
│   │       ├── engine.test.js     # Vitest unit tests
│   │       ├── data/              # Bundled recipes.json + iconMap.json
│   │       └── components/         # Header, InputPanel, SummaryCards, …
│   ├── public/icons/              # Item icon PNGs
│   └── vite.config.js             # base path set for Pages subpath
├── backend/                       # Rust/Axum reference solver
│   └── src/{main.rs, recipe_engine.rs, recipes.json, icon_map.json}
└── scripts/download_icons.py      # Icon scraper (satisfactory.wiki.gg)
```

## Data sources

- Recipe data is curated in `backend/src/recipes.json` (and mirrored into the
  frontend at build time).
- Item icons are scraped from the [Satisfactory wiki](https://satisfactory.wiki.gg/)
  via `scripts/download_icons.py`.

## License

[MIT](LICENSE) — Satisfactory is a trademark of Coffee Stain Studios; this
project is a fan-made planning tool and is not affiliated with or endorsed by
Coffee Stain Studios. Item icons are sourced from the community wiki.
<script>
  let { repoUrl = "https://github.com/jay37mack37/satisfactory-calculator" } = $props();
</script>

<header class="hero">
  <div class="badge">⚡ Production Chain Planner</div>
  <h1>Satisfactory<span class="grad">Calculator</span></h1>
  <p class="subtitle">
    Pick a target item, set your desired output rate, and get the full
    breakdown — machines, power, raw resources, byproducts, and alternate
    recipes.
  </p>
  <a class="repo-link" href={repoUrl} target="_blank" rel="noreferrer">
    <svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
      <path
        fill="currentColor"
        d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0016 8c0-4.42-3.58-8-8-8z"
      />
    </svg>
    View on GitHub
  </a>
</header>

<style>
  .hero {
    text-align: center;
    margin-bottom: 40px;
    padding: 16px 0 8px;
  }

  .badge {
    display: inline-block;
    font-size: 0.72rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--accent);
    background: var(--accent-glow);
    border: 1px solid var(--accent-glow);
    padding: 5px 12px;
    border-radius: 999px;
    margin-bottom: 18px;
  }

  h1 {
    font-size: clamp(2.2rem, 6vw, 3.4rem);
    font-weight: 800;
    letter-spacing: -0.02em;
    line-height: 1.05;
    margin-bottom: 14px;
  }

  .grad {
    background: var(--accent-grad);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
    margin-left: 0.25em;
  }

  .subtitle {
    color: var(--text-secondary);
    font-size: 1.05rem;
    max-width: 620px;
    margin: 0 auto 20px;
    line-height: 1.5;
  }

  .repo-link {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-secondary);
    text-decoration: none;
    padding: 8px 16px;
    border: 1px solid var(--border);
    border-radius: 999px;
    background: var(--bg-card);
    backdrop-filter: blur(10px);
    transition: all 0.2s;
  }

  .repo-link:hover {
    color: var(--text-primary);
    border-color: var(--border-strong);
    transform: translateY(-1px);
  }
</style>
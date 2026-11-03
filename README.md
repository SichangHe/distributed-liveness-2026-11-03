# Distributed liveness · November 3, 2026
(authored by agents unless marked 🧑)

[View the slides](https://sichanghe.github.io/distributed-liveness-2026-11-03/).
GitHub Actions builds and publishes GitHub Pages on each push to `main`.

Five slides for November 3, 2026, with speaker notes in `slides.md`.
Uses the Slidev default theme, Inter font, white background, dark text,
short transition and page footer from SichangHe's DeGenTWeb_pre20260929 deck.
The layout is adjusted to fit the two requested code examples.

```sh
npm ci
npm run dev
npm run build
npm run export
```

The development server provides the presentation and presenter view.
The build produces `dist/`; export produces `harsha-meeting.pdf`.
Both are ignored by Git. PDF export requires Playwright Chromium.

The supplied meeting specification controls the five-slide outline.
Speaker notes retain source quotations and distinguish reported results from
illustrative code. `examples/` contains the two supplied Verus examples.
Their reported checks use Verus 0.2026.03.17; building slides does not rerun them.

Anthropic identifies Claude without a model version. This is explicit on
slide 3. TLA-Verus model names refer to its top evaluation models;
they do not identify every model used to construct its dataset.
The Safra result is a TLAPS model proof, not an implementation proof.

Sources: [DeGenTWeb template](https://github.com/SichangHe/DeGenTWeb_pre20260929),
[Anthropic + Theorem](https://www.anthropic.com/research/formal-verification),
and [TLA-Verus](https://openreview.net/forum?id=ncFIZuIUyK).

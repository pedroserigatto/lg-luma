# Fork plan: the first changes

Written 2026-11-02, so the work can continue from any session (cloud or local). Each change is its own PR, merged in this order. Every step is committed and pushed as it is done, so a PR's commits show how far it got. Background and decisions: [FORK-FEATURES.md](FORK-FEATURES.md).

## Decisions (2026-11-02)

- Keep: weather, shortcut tiles (Home Assistant), Delete app, launch-splash sync, autostart (for now), the webOS 5/6 takeover code (for a later attempt), self-update (repointed to this fork).
- Remove: 11 of the 13 interface languages (keep English and Portuguese (Brazil)); the Korean and Chinese fonts; the webOS 10/11 and 24 takeover code.
- HDMI input status: confirmed correct on the TV.
- Takeover on this TV (checked on the rootfs image and the TV, 2026-11-02): LG's app manager has no `blockSystemApp`/`releaseBlockSystemApp`/`forceBlockSystemApp` and no `profile.blockedAppList` (only `registerNativeApp`); the Home role is tied to the executable path (`exeName /usr/bin/com.webos.app.home`, exact `allowedNames`). So only the webOS 5/6-style bind mount over the executable can apply.
- Releases: keep upstream's release workflow (Changesets "Version Packages" PR, then a GitHub release with the IPK and the Homebrew feed). The fork's Actions may create PRs (setting enabled). Versions continue from 2.16.2.

## How to check a change (every PR)

```sh
pnpm install --frozen-lockfile
pnpm format:check        # cargo fmt + oxfmt
pnpm check               # clippy, lints, version sync
pnpm test                # unit tests (node + cargo)
pnpm assets              # needs woff2_decompress and Python fontTools (nix develop has them)
xvfb-run -a pnpm test:smoke
```

CI runs all of this on each PR, and **PR Test IPK** builds an installable package (download it from the PR's bot comment, a ZIP, kept 14 days). Add a changeset to code PRs: `pnpm changeset` (patch for removals), committed under `.changeset/`.

## PR A: languages (keep English and Portuguese (Brazil))

Goal: the language list shows only English and Português (Brasil); a saved preference for a removed language loads as English; nothing else changes.

1. `src/i18n.rs`: drop `De, Zh, ZhTw, Es, It, Ru, Pl, Fr, Vi, Nl, Sv` from `Language`, `ALL` (becomes 2 entries), `code()`, `name()`, the date/clock/greeting formatting arms, geocoding codes, and the system-locale mapping (around line 490). A saved unknown code must fall back to `En` (check the `Deserialize` impl: an unknown string must not make the whole preferences file invalid). Remove the tests for the removed languages (Chinese, etc.); keep and adapt the English/Portuguese ones.
2. `assets/messages.json`: keep only the `en` and `pt` strings of each entry (a small script; keep key order).
3. `tests/translations-smoke.mjs`: limit to `en` and `pt`.
4. `readme.md`/`development.md`: the language list (development.md, "Storage, weather and languages").
5. Check that the TV's language (pt-BR) selects Portuguese at first start: find where the system locale is read (`i18n.rs`, `platform.rs`).
6. Changeset (patch), checks, push, PR.

## PR B: Korean and Chinese fonts (after PR A)

Goal: the package no longer carries the CJK and Hangul fallback fonts (`font-cjk-400/500.ttf`, `font-kr-400/500.ttf`, generated from Noto Sans SC/KR), and `font-metrics.json` no longer holds their glyph advances (it may be large and is likely loaded at startup: measure before/after). Text in those scripts (an app or city name) would then show as missing glyphs; no installed app on this TV has such a title.

1. `tools/font-weights.py`: remove `fallback()` and its two calls; write `font-metrics.json` without `fallback-*` entries.
2. `tools/assets.mjs`: stop requiring and passing `noto-sans-sc.ttf`/`noto-sans-kr.ttf`; stop copying `font-cjk-license.txt`.
3. `tools/font-assets.mjs` and `pnpm assets:fonts`: stop downloading them; delete `assets/fonts/noto-sans-sc.ttf`, `noto-sans-kr.ttf`, `noto-ofl.txt` (27 MB in the repo).
4. `src/renderer.rs` (around lines 750-780 and 1705-1725, `is_hangul`, font ids 2000/3000): drop the fallback fonts; characters missing from Inter take the existing missing-glyph path. Check `src/font.rs` and `src/text_rendering.rs` for `fallback-` metrics lookups.
5. `tools/package.mjs`: remove any check or copy of the CJK files and licence.
6. `src/i18n.rs` test `bundled_cjk_font_covers_every_traditional_chinese_message` (already gone with PR A).
7. `assets.md`, `development.md` (vector rendering: "CJK fonts open only when needed").
8. Record the package size before/after (the IPK in the PR Test IPK artifact; 2.16.2 upstream: 35.6 MB).
9. Changeset (patch), checks, push, PR.

## PR C: webOS 10/11 and 24 takeover code

Goal: `home.sh`/`luma-helper home` keep only the webOS 5/6 executable method (bind mount over `/usr/bin/com.webos.app.home` or `fullhome`), and their version check stays as it is for now (refusing webOS 4) until a later, careful attempt.

1. `tv-helper/src/home.rs`: remove the manifest method (staged manifest, `blockSystemApp`/`releaseBlockSystemApp`, webOS 24 handling) and the webOS 10/11 lifecycle registration (`luna-send -a com.webos.app.home`, `registerNativeApp` holder); keep the executable method, status, disable/restore, boot hook.
2. `tv/home.sh`: the same.
3. `src/main.rs`/`src/platform.rs`: anything only the manifest method uses (staged Home identity, lifecycle holder).
4. Tests: `tests/home-wrapper.test.mjs`, the `luma-tv-helper` cargo tests: drop the manifest/10-11/24 cases.
5. `readme.md` ("Replacing LG Home": methods for 25/26), `development.md` ("Optional native Home integration", "Home lifecycle on webOS 10/11").
6. Changeset (patch), checks, push, PR.

## PR D: self-update from this fork (after the first release)

Goal: Luma checks `https://github.com/pedroserigatto/lg-luma/releases/latest/download/repo.json` instead of upstream's, and installs only this fork's release assets.

1. `src/update.rs`: the feed URL and the accepted asset URL prefix (`ecklf/lg-luma` -> `pedroserigatto/lg-luma`); `tv/self-update.sh` if it checks URLs.
2. Tests in `src/update.rs`/`tests/self-update.test.mjs`.
3. Docs (development.md, "Update availability").
4. Only after the fork has published a release (merge PRs A-C, then the "Version Packages" PR).
5. Then on the TV: add the fork's feed to the Homebrew Channel (launch it with `{"launchMode":"addRepository","url":"https://github.com/pedroserigatto/lg-luma/releases/latest/download/repo.json"}`) and remove upstream's (`https://github.com/ecklf/lg-luma/releases/latest/download/repo.json`, click it in Homebrew Channel Settings): both use the same app id.

## Later (not in these PRs)

- The TV round ([FORK-FEATURES.md](FORK-FEATURES.md), "TV checks"): Recent apps, upload from a phone, Delete app, startup log, memory.
- Staying resident and hidden so Home opens instantly; the Home key; the memory diet.

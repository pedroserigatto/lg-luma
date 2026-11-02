# Feature inventory for this TV

Every Luma feature (version 2.16.2), whether it works on this TV (LG 50UM7360PSA, webOS 4.10.2-29, LCD), what it costs, and what to do with it. Written 2026-11-02 from the code, `readme.md`, `development.md` and a first run of Luma 2.14.0 on the TV. Nothing has been changed yet: the **Decision** column is filled in with the owner.

**On this TV:** ✅ seen working on the TV · ❔ unknown, needs a TV check (listed in [the TV checks](#tv-checks)) · ❌ does not work here.

**Cost:** what it uses while it is not in use: _none_ means code only, nothing loaded or running until used.

## Core

| Feature                        | What it does                                                                | On this TV                                                                                                 | Cost when idle                                 | Proposal | Decision |
| ------------------------------ | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ---------------------------------------------- | -------- | -------- |
| Launcher, rendering            | Native window drawn with the GPU (EGL, GLES2 via SDL-webOS and wayland-egl) | ✅ smooth, no flashing                                                                                     | The window's buffers                           | Keep     | Keep     |
| App inventory and launching    | Lists installed apps, opens them                                            | ✅ all apps, video plays after launch                                                                      | Small                                          | Keep     | Keep     |
| Installed app icons            | Reads each app's own icon, decoded off the main thread                      | ✅ all icons shown                                                                                         | Icon caches (see [Memory](#memory))            | Keep     | Keep     |
| Curated brand cards            | Vector logos for 38 known apps (YouTube, Netflix, ...)                      | ✅                                                                                                         | Meshes loaded when shown                       | Keep     |          |
| Home favourites row, Apps grid | One favourites strip, a six-column app grid, reordering (hold OK)           | ✅                                                                                                         | Small                                          | Keep     |          |
| Inputs (HDMI) widget           | Lists inputs; polls HDMI status every 2 s while Home is showing             | ❔ status fields (`hdmiPlugIn`, `hdmiSignalExist`) may be missing on webOS 4; it falls back to `connected` | One read every 2 s, only while Home is visible | Keep     |          |
| Date, clock, greeting widgets  | Date formats, 12/24 h, seconds optional, "Good evening, name"               | ✅                                                                                                         | Small; seconds redraw once a second            | Keep     |          |
| Weather widget                 | Open-Meteo forecast for a chosen city, cached                               | ✅ (owner uses it)                                                                                         | A request every 15 min while Home is visible   | Keep     | Keep     |
| Settings                       | General, Widgets, Background, Languages, System                             | ✅                                                                                                         | Built when first opened (or prewarmed, below)  | Keep     | Keep     |
| Quit app (Settings → System)   | Ends Luma                                                                   | ✅                                                                                                         | None                                           | Keep     |          |
| Remote and pointer input       | Arrows, OK, hold OK, Back, Menu, pointer, wheel                             | ✅                                                                                                         | None                                           | Keep     |          |

## Backgrounds

| Feature               | What it does                                                                                                         | On this TV                                     | Cost when idle                   | Proposal                                                         | Decision |
| --------------------- | -------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- | -------------------------------- | ---------------------------------------------------------------- | -------- |
| Bundled wallpapers    | Six photos and three drawn backgrounds in the package (6.1 MB)                                                       | ✅                                             | The current one as a GPU texture | Keep                                                             |          |
| Gradient toggle       | Readability shading on or off                                                                                        | ✅                                             | None                             | Keep                                                             |          |
| Rotation              | Next background every 15 minutes, with a fade                                                                        | ✅ (rendering)                                 | A timer                          | Keep; later: by time of day                                      |          |
| Upload from a phone   | QR code; a temporary local web page (only while its dialog is open, 10 min max) to send a photo                      | ❔ the TV app's network permissions on webOS 4 | None while closed                | Keep                                                             |          |
| LG launch splash sync | Copies the chosen background into the splash LG shows while Luma starts, through the Homebrew Channel's root service | ❔                                             | A file write after saves         | Remove once Luma stays resident (the splash is then rarely seen) |          |

## Shortcuts and integrations

| Feature                    | What it does                                                              | On this TV                      | Cost when idle    | Proposal                | Decision |
| -------------------------- | ------------------------------------------------------------------------- | ------------------------------- | ----------------- | ----------------------- | -------- |
| Shortcut tiles             | Home tiles that send a web request (for example a Home Assistant webhook) | ✅ expected (plain HTTP)        | None              | Keep (owner may use it) | Keep     |
| Shortcut editor on a phone | QR code to a temporary page to create and edit tiles                      | ❔ same network check as upload | None while closed | Keep                    |          |

## System and lifecycle

| Feature                                   | What it does                                                               | On this TV                                                                                 | Cost when idle                  | Proposal                                                    | Decision |
| ----------------------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ | ------------------------------- | ----------------------------------------------------------- | -------- |
| Recent apps (Back on Home, or hold Back)  | Card switcher of recent apps; close one with Up                            | ❔ uses `com.webos.service.recent`, captured on webOS 26                                   | None until opened               | Keep if the service exists on webOS 4, else remove          |          |
| Delete app                                | Uninstalls an app from its options popup (LG's own apps are protected)     | ❔ `appInstallService/remove` on webOS 4                                                   | None                            | Decide: useful, but destructive                             |          |
| Self-update                               | Checks the **upstream** releases at every start and offers to install them | Works, but **wrong for this fork**: it would replace this version with upstream's          | A network request at each start | Remove (later maybe: updates from this fork's own releases) | Remove   |
| Autostart (25 s after boot)               | Opens Luma once after a cold boot                                          | ✅ on webOS 4 (upstream issue #98)                                                         | A boot hook                     | Remove: replaced by staying resident and the Home key       |          |
| Home takeover, webOS 10/11 and 24 methods | Mounts Luma over LG's Home manifest; app-lifecycle tricks                  | ❌ refused on webOS 4 by a version check; LG's mechanisms differ                           | Code only                       | Remove                                                      |          |
| Home takeover, webOS 5/6 method           | Bind-mounts Luma's program over LG's Home program                          | ❔ refused on webOS 4 by a version check only; this TV's Home apps have the same structure | Code only                       | Keep the code for a later, careful attempt                  |          |
| Startup diagnostics, profiling            | `--profile-frames`, `--diagnostics`, `LUMA_*` switches                     | ✅                                                                                         | None unless asked               | Keep: needed to measure                                     | Keep     |

## Text, languages, rendering

| Feature                          | What it does                                                                 | On this TV                                    | Cost when idle                                                   | Proposal                                              | Decision |
| -------------------------------- | ---------------------------------------------------------------------------- | --------------------------------------------- | ---------------------------------------------------------------- | ----------------------------------------------------- | -------- |
| Latin font (Inter)               | The interface text                                                           | ✅                                            | Glyph caches                                                     | Keep                                                  | Keep     |
| Korean and Chinese fonts         | Noto Sans KR and SC, opened only when such text appears                      | Not needed: no installed app has such a title | **27 MB of the 35 MB package** (disk); no RAM unless used        | Remove                                                |          |
| 13 interface languages           | Translations, date formats                                                   | ✅                                            | 128 KB file                                                      | Keep Portuguese (Brazil) and English; remove the rest |          |
| Text rendering setting           | 4x MSAA (default) or analytic edges                                          | ✅ MSAA in use                                | MSAA buffers, size unknown on this GPU                           | Keep; measure MSAA's memory                           |          |
| Partial redraw                   | Redraws only what changed (needs EGL buffer-age support)                     | ❔ the startup log says which mode            | Saves GPU work                                                   | Keep                                                  |          |
| Idle prewarm                     | Builds the other pages in the background after 1.5 s idle, so they open fast | ✅                                            | Fills caches ahead of time: likely part of the 57 → 95 MB growth | Keep; measure, maybe limit                            |          |
| 4K rendering request             | Experimental `--resolution 4k`                                               | Not useful: the TV's graphics plane is 1080p  | Code only                                                        | Leave (command-line only)                             |          |
| Desktop preview and test scripts | Runs on a computer, not in the TV package                                    | Not on the TV                                 | None                                                             | Keep (development)                                    | Keep     |

## Memory

Budgets in the code, not measured on the TV yet (the [TV checks](#tv-checks) measure them):

- Text: glyph geometry 8 MiB CPU, text runs 32 MiB GPU.
- Images: icons up to 16 MiB CPU (256 icons), raster artwork 48 MiB GPU.
- MSAA: 4x on a 1920x1080 window.

Seen on the TV with 2.14.0: 57 MB just started, 95 MB after browsing, peak 116 MB. LG's basic Home uses 71-74 MB. The diet starts from these budgets and the measurements.

## TV checks

Questions only the TV can answer, for the next testing round:

1. Recent apps: does Back on Home open the switcher with real apps, and do Open and Close (Up) work?
2. Inputs: do the HDMI tiles show the right connected or not-connected state?
3. Upload from a phone and the shortcut editor: does the QR page open on a phone on the same network, and does an upload arrive?
4. Delete app: only if kept. Test it on an app that can be reinstalled.
5. The startup log: partial redraw mode, MSAA samples, first-frame time.
6. Memory: just started, after opening each page, after an hour; with MSAA and with analytic text.

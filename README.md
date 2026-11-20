# Tavru Sheets

> **Product name:** Tavru Sheets. Earlier public names were CellTrellis and Nanshe. Repository: [github.com/tomblanchard312/Tavru-Sheets](https://github.com/tomblanchard312/Tavru-Sheets).

A Rust spreadsheet with a Developer workspace for **VBA, Python and R**, and **Scribe**, a local
assistant powered by [Oqelith](https://github.com/tomblanchard312/Oqelith).

Tavru Sheets is a fork of [GridCraft](https://github.com/storytold/gridcraft) by the ArtCraft team.
The window title is Tavru Sheets. Crates, the `nanshe` binary, `NANSHE_*` variables, command
names and settings directories stay on the Nanshe names so upstream improvements remain easy
to merge. About lists the upstream contributors and licenses. See [product identity](docs/product-name.md).

## Run

```sh
cargo run -p tavru-sheets -- --sample sales
```

Open **Developer → Code Editor**, choose VBA, Python or R, edit the script, acknowledge that you
trust it, and choose **Run preview**. Review the engine calls and output, then choose
**Apply script changes**. The changes apply together and can be undone together. A changed or
closed workbook invalidates its preview. Save source in the workbook with **Save in workbook**;
ordinary file saves and close prompts also include edited script drafts.

- VBA needs the optional .NET helper built from [`vba/`](vba/README.md).
- Python needs a local Python 3 interpreter (`TAVRU_SHEETS_PYTHON` selects a virtual environment).
- R needs a local R installation and the `jsonlite` package (`NANSHE_RSCRIPT` selects Rscript).
- The Scribe pane requires a local Oqelith runtime and a registered chat model. Open Scribe and
  send a request. Responses can explain a selection,
  propose a formula, or draft any of the three script languages. Applying or running a suggestion
  always requires your action.

Python and R execute trusted local code with your account's permissions. The workbook preview is
not an operating-system sandbox. Read the [setup, examples and limits](docs/nanshe-developer.md).

## Checks

```sh
cargo xtask ci
dotnet build vba/NansheVba.sln -m:1
dotnet run --project vba/tests/Nanshe.Vba.Tests --no-build
```

The inherited architecture and format documentation is in [`docs/`](docs/). Packaged
installer identifiers and artwork still follow upstream; this branch is a source-build port,
not a signed Tavru Sheets release. The upstream self-update feed is disabled for this fork.

## License and credits

MIT OR Apache-2.0. Upstream copyright remains with the ArtCraft Team and the GridCraft contributors.
Fork changes are copyright (c) 2026 Tom Blanchard; NOTICE still records those changes under the
earlier Nanshe and Nisaba names. See [NOTICE](NOTICE) and [ATTRIBUTION.md](ATTRIBUTION.md).
Tavru Sheets is not sponsored or endorsed by the ArtCraft team.

Voice accessibility preview: see [dictation, transcription, translation and safe voice controls](docs/VOICE.md).

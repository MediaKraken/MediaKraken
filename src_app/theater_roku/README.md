# theater_roku

A Roku SceneGraph channel for browsing a MediaKraken server from a Roku
device. Mirrors the category layout of `theater_tizen` (Movies, Music, TV,
Sports, Live TV, Image Gallery, Games, Books, iRadio, Home Movies, 3D,
Internet) and queries the MediaKraken HTTP API for catalog listings.

## Layout

```
manifest                     # Roku channel manifest
bsconfig.json                # rokucommunity / VS Code BrightScript config
Makefile                     # `make` builds the sideload zip; `make sideload` pushes it
source/
  Main.brs                   # entry point, launches HomeScene
  Registry.brs               # roRegistrySection helpers (server URL, etc.)
components/
  HomeScene.{xml,brs}        # top-level scene with category grid
  CategoryTile.{xml,brs}     # grid tile renderer
  CatalogTask.{xml,brs}      # roUrlTransfer-based HTTP task node
images/                      # channel art (see images/README.md for required PNGs)
```

## Build

```
make            # produces build/theater_roku.zip
make sideload ROKU_HOST=192.168.1.50 ROKU_PASS=mypass
```

The default sideload credentials are `rokudev` / `rokudev`; override via env
or `make` arguments.

## Configuring the server URL

On first launch the channel reads the `ServerUrl` value from the
`MediaKraken` registry section. If unset, it falls back to the value returned
by `MK_DefaultServerUrl()` in `source/Registry.brs`
(`http://mediakraken.local:8080`).

To change the server URL at runtime press the `*` (Options/Info) button on
the home screen and enter a new URL in the keyboard dialog. The value is
persisted to the registry.

## API contract

The HTTP task currently calls:

```
GET <server>/api/titlesearch/<category>
Accept: application/json
```

It expects either a JSON array of title objects or
`{ "items": [...] }`. The response is logged to the status label; rendering
a per-category result grid is the next step.

## Required image assets

The manifest and category grid reference PNGs under `images/`. They are not
checked in; see `images/README.md` for the list and recommended dimensions.
The channel will sideload without them but tiles and splash will be blank.

## Status

This is a working channel skeleton, not a full client. Things still to do:

- Result grid for catalog responses (currently only the count is shown).
- Playback via the Roku Video node, against whatever stream URL the
  MediaKraken server returns.
- Deep linking handler beyond the placeholder in `Main.brs`.
- Channel art (see `images/README.md`).

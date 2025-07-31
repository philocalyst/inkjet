#!/usr/bin/env just

# Fail on error, verbose output, pipefail
set shell := ["bash", "-eux", "-o", "pipefail", "-c"]

# Common build command
BUILD := "cargo build --all-features -vv"

[doc("Show usage if no target is provided.")]
must_specify:
    @echo "Must specify one of: redownload [languages], regenerate [languages], features, themes"

[doc("Redownload language modules.")]
redownload:
    INKJET_REDOWNLOAD_LANGS=true {{BUILD}}

[doc("Regenerate language modules.")]
regenerate:
    INKJET_REBUILD_LANGS_MODULE=true {{BUILD}}

[doc("Rebuild features.")]
features:
    INKJET_REBUILD_FEATURES=true {{BUILD}}

[doc("Rebuild themes.")]
themes:
    INKJET_REBUILD_THEMES=true {{BUILD}}

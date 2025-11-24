# Hi

## What is this

Well Nixpkgs sucks at packaging Rust packages currently as it is incapable of detecting
when dependencies are insecure. There's probably packages that the upstream hasn't bump their
lock file in 500 years with insecure rustls versions.

## The idea

1. Add a new package set to Nixpkgs, something like `pkgs.rustCrates`
2. Add a builder for individual crates that just takes the source, and places it in a convinent location for symlinking to Cargo Vendor directories
3. Add another builder that takes a list of crate sources, and symlinks them all together
4. Change `buildRustPackage` to accept a filtering predicate that selects dependencies upon `pkgs.rustCrates`
   - Note that this will require a way to add git deps
5. When building a crate do the standard thing of making a cargo-vendor manifest like is done now, just with all the crates above

This is a prototype tool for creating the `rustCrates` manifest. It would just output a big JSON file. Each crate would also
have an adjacency list of all its dependencies as well, and depend on them like standard Nix packages.

This avoids the problem of `npmPackages` since the file would not need processing and would just be updated ever week or so, so merge
conflict hell should not be an issue.

## The problem

This idea is pretty decent. It allow Nixpkgs to have control over `crates.io` dependencies. When a CVE is published, a predicate
filtering that crate version out from the package set could be added, then all downstream and leaf packages would fail to build.

The issue is that the JSON file is quite large. It would at least double the size of Nixpkgs, but in reality closer to triple I would guess.

This tool can be a bit smarter than adding every version of every crate. We only want the latest semver-breaking versions. So the latest of
each major version would be added. The tool currently just adds the latest of every version, which outputs a 50MB JSON file. This is essetnially the
minimum bound.

The tool is pretty fast though! Takes around 30 seconds to run, which processses all 200k-ish crates

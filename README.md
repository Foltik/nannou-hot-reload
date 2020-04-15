# nannou-hot-reload

My experiments in hot reloading Nannou sketch code.

Contains two separate implementations, one that uses polling via
`dynamic_reload` and works on all platforms, and one that picks up
changes instantly using `inotify` and `libloading` directly, but only
works on Linux.

To test it, select one of the implementations, build each sketch
(`circle`, `square`), then run the main application. Make sure to
build the sketches in release mode, as they are hard coded to search
for the dynamic libraries in `target/release`. When running, the
spacebar switches between the two sketches. To trigger a reload, just
make a change to one of the sketch's source code, then rebuild it. The
main application should pick up the change and hot reload the sketch.

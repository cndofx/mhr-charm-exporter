# mhr-charm-exporter
Charm Exporter forked from [Cypher121/mhrise_charm_exporter](https://github.com/Cypher121/mhrise_charm_exporter), updated to work on the current version (v10.0.3.0).
Very work in progress, and will probably break again with the next update. See my other project [here](https://github.com/cndofx/mh-rise-ct) for a more reliable exporter.

## Problem
update: [pseuxide/toy-arms](https://github.com/pseuxide/toy-arms) looks promising for getting the base address and potentially also pattern scanning

This project currently uses a static address as a base for finding all of the other addresses it needs to read from, but that static address will change every time the game gets an update. It probably also won't work if the game happens to load into memory at an unexpected address. I have a general idea of how to handle this, as I have in my other project linked above, but I haven't been able to implement it in Rust. My method uses an AOB/signature scanner, but I can't find any crate that has this functionality. I would implement the signature scanner myself, but I need a way to get the base address of a module and I can't find any crate that can do this either. I can do that in C++ with the windows api, but I don't know Rust or the windows api well enough to make it happen here.

Any suggestions and/or contributions are welcome. Until I find a solution, I can't guarantee that this tool will remain working as long as the game continues to get updates.

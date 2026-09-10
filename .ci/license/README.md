# License dependency snapshot

`Cargo.lock` is intentionally scoped to license checks. Normal builds of this
library resolve the loose dependency constraints in `Cargo.toml`.

The license scripts use Cargo's alternate lockfile path support, which requires
Cargo 1.97 or newer. To intentionally refresh both the snapshot and the license
inventory, run:

```shell
./scripts/update-license.sh
```

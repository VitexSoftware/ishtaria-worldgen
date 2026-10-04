# ishtaria-worldgen

Deterministic procedural generator of an Earth-sized planet. Terrain is a pure function of `(seed, position)` on a cube-sphere; worlds store only deltas.

```sh
ishtaria-worldgen 42 256 > planet.pgm     # six cube faces as a height-map strip
```

## Installation

Debian / Ubuntu (x86-64) only:

```sh
echo "deb http://repo.vitexsoftware.com $(lsb_release -sc) main" | sudo tee /etc/apt/sources.list.d/vitexsoftware.list
sudo wget -O /etc/apt/trusted.gpg.d/vitexsoftware.gpg http://repo.vitexsoftware.com/keyring.gpg
sudo apt update
sudo apt install ishtaria-worldgen
```

License: MIT

## Part of Ishtaria

Ishtaria is an open-source, persistent, federated virtual planet of Earth size.
Documentation: https://vitexsoftware.github.io/ishtaria-docs/ · All repositories: https://github.com/VitexSoftware?q=ishtaria

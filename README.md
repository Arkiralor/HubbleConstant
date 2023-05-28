# Hubble Constant

A binary application written in Rust to calculate the Hubble Constant and use it to calculate the age of the observed universe.

## Theory

The Big Bang Theory, extrapolated from the works of [Edwin Powell Hubble](https://en.wikipedia.org/wiki/Edwin_Hubble) and [Georges Lemaître](https://en.wikipedia.org/wiki/Georges_Lema%C3%AEtre), basically describes that at ___t__<sub>0-n</sub>_, the universe was a single, infinitely dense singularity that somehow, _spontaneously_ started explosively expanding at ___t__<sub>0</sub>_.

[Vesto Slipher](https://en.wikipedia.org/wiki/Vesto_M._Slipher) first made the observations in the early years of the 20<sup>th</sup> century, when he observed that all the light (_electromagnatic radiation_) from distant galaxies was [redshifted](https://en.wikipedia.org/wiki/Redshift) i.e, all the light was stretched/elongated, hence, the sources of those waves were moving away from us[^1], although it is not clear if he comprehended the full ramifications of his observations.

Hence, if all objects in the known universe were speeding away from each other then it was logical to conclude that at one point in time, all matter and energy were one, infinitely dense singularity, which recently, was calculated to be approximately _1.38x10<sup>11</sup>_ years ago.[^2]

Based on further research, it was discovered that the further away two points were in the universe (_in scales that can be calculated in MegaParsecs_), the faster they were speeding away from each other[^3]. From this observation, spawned the _Hubble–Lemaître law_.

The _Hubble Constant_ can be defined as the rate at which the speed of separation changes as the distance between two objects increases. For any object significantly far away, it is described as

```math
    h_0 = {v \over d}
```

Where _v_ is the [Helio Radial velocity](https://en.wikipedia.org/wiki/Radial_velocity) of a distant galaxy denoted in _Km/s_ and _d_ is the distance of the distant galaxy from the Earth, denoted in _Mpc_.

Its unit is ___Km s<sup>-1</sup> Mpc<sup>-1</sup>___ i.e, _Kilometres per Second per Megaparsec_.
  
- It can be explained/used as "_The speed of separation between two sufficiently far away objects A and B increases by X Km sec<sup>-1</sup> for every 1 Mpc increase in the distance between A and B_".

Further reading on the subject is recommended and [_this link_](https://en.wikipedia.org/wiki/Hubble%27s_law) can be used as a starting point for laypersons; the more academically inclined may refer to the [_Pensylvania State University's dossier_](https://www.e-education.psu.edu/astro801/content/l10_p3.html) on the subject.

## Usage

There are two steps to use this application. First, we have to build the binary for the system that we need to run the application on; then we can run the binary itself.

### Pre-Requistes

There are a few pre-requisites to run this application as the source code makes some assumptions about the platform it is expected to run on.

1. __Console:__ _BASH_ for UNIX-based operating Systems
   - _GitBash_ for Windows-based Operating Systems
2. __Language Compiler:__ This application was written using the _RuctC_ compiler `v1.69`.
   - __Package Manager:__ The package manager for Rust, _Cargo_ is required; if not installed during the installation of the compiler, kindly rectify.

### 1. Build

This step compiles the source code into a platform specific executable.

```sh
sh scripts/build.sh `buildType`
```

- The `buildType` can be: `release` | `debug` | `both`
  - `both` will obviously build both targets sequentially.

### 2. Run

This step runs the compiled executable.

A data (_JSON_) file will be needed to calculate the Hubble Constant based on the distances and velocities of individual distant galaxies (_distance ~>= 1 Megaparsec_).

An example is provided in `data/galaxies.json`.

```sh
sh scripts/run.sh `executableType` `pathToDataFile`
```

- The `executableType` can be: `release` | `debug`.
- The `pathToDataFile` can either be the absolute path to the file or the relative path from `Cargo.toml`

  - If not set, the example file `$(PWD)/data/galaxies.json` is used and a notice is displayed on the console.

[^1]: Slipher, Vesto (1917). "Nebulae". Proceedings of the American Philosophical Society. 56: 403–409. Bibcode:1917PAPhS..56..403S.
[^2]: Planck Collaboration (2020). "Planck 2018 results. VI. Cosmological parameters". Astronomy & Astrophysics. 641. page A6 (see PDF page 15, Table 2: "Age/Gyr", last column). arXiv:1807.06209. Bibcode:2020A&A...641A...6P. doi:10.1051/0004-6361/201833910. S2CID 119335614.
[^3]: "Hubble Flow". The Swinburne Astronomy Online Encyclopedia of Astronomy. Swinburne University of Technology. Retrieved 2013-05-14.

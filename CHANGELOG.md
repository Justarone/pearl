# Changelog
Pearl changelog


## [Unreleased]
#### Added
- Add hierarchical filters support (#126)


#### Changed


#### Fixed


#### Updated


## [0.9.0] - 2021-12-09
#### Added
- Add hierarchical filters support (#126)


## [0.8.1] - 2021-12-02
#### Added
- Add key trait (#123)


## [0.8.0] - 2021-11-09
#### Added
- Add method to offload bloom filters (#121)

#### Changed
- Dump blob indices in separate thread on active blob close (#136)
- Remove second file descriptor from File (#124)
- Acquire advisory write lock on files (#124)


## [0.7.1] - 2021-10-18
#### Added
- Add more functions to interface to support work with optional active blob (#118)


## [0.7.0] - 2021-10-02
#### Added
- B+ Tree indexes (#84)
- Range indexes (#11)
- Move corrupted blobs into speacial directory (#98)
- Blob version validation (#120)

#### Changed
- Rebuild corrupted index automatically during startup (#94)
- Move build_time.rs file to OUT_DIR.

#### Fixed
- Create corrupted directory only when needed (#94)


## [0.6.2] - 2021-09-14
#### Added
- Setup build and test with GitHub Actions (#113)

#### Changed
- Helpers for build time, commit hash and version moved to `build_info` mod.
- Build time format.
- `build_time.rs` added to `.gitignore`.


## [0.6.1] - 2021-08-13
#### Updated
- Blob version v0 -> v1


## [0.6.0] - 2021-08-13
#### Changed
- Add key size check for blob from file (#99)
- `ahash` version fixed on `=v0.7.4` (#112)


#### Fixed
- Fix lint warnings (#100)


## [0.5.14] - 2021-04-14
#### Fixed
- Fix possible infinite readlock (#97)
  - Change all readlocks on writelocks
  - add more traces, increase check timer
  - disable old blobs dump
  - fix benchmark and lock issue


## [0.5.13] - 2021-04-13
#### Fixed
- Fix observer worker leak and simplify logic
- Fix 'core dumped' bug


## [0.5.12] - 2021-03-30
#### Added
- Add check before fsync index file in dump (#90)
- Add an option to ignore corrupted blobs (#86)

#### Changed
- Switch to RwLock for storage inner safe (instead of Mutex) (#91)

#### Updated
- update libs


## [0.5.11] - 2021-03-24
#### Added
- Add synchronization for closing blobs to keep them in stable state in case of future disk faults (#85)
- Add an option to ignore corrupted blobs (#86)

#### Updated
- tokio update v1.3


## [0.5.0] - 2020-08-21
#### Changed
- reduced disk IO calls when index used
- remove Entries
- Replace Error::RecordNotFound with Option
- use anyhow crate for result/error handling

#### Fixed
- wrong offset of index file read


## [0.4.0] - 2020-07-30
#### Added
- use of the io_uring, linux asynchronous I/O API.


## [0.3.1] - 2020-05-15
#### Added
- storage records count methods all/active/detailed


## [0.3.0] - 2020-03-02
#### Added
- CHANGELOG.md

# Changelog

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## [v3.4.0](https://github.com/Clay-6/WhatYouMean/releases/tag/v3.4.0)

### Added

* `--sources` to show definitions' source dictionary
* `--from` to only show results from a given source dictionary

### Fixed

* Some flags not having help text

### Changed

* Updated dependencies

## [v3.3.0](https://github.com/Clay-6/WhatYouMean/releases/tag/v3.3.0)

### Added

* `--wotd` flag for Wordnik's Word of the Day
* `--syllables` flag

### Fixed

* XML tags appearing in examples

### Changed

* More detailed error display

## [v3.2.1](https://github.com/Clay-6/WhatYouMean/releases/tag/v3.2.1)

### Changed

* Searched word appears in JSON output
* Improved error display

### Fixed

* `--use-key` behaviour

## [v3.2.0](https://github.com/Clay-6/WhatYouMean/releases/tag/v3.2.0)

### Added

* `--json` for JSON output

### Removed

* Remove default API key

## [v3.1.1](https://github.com/Clay-6/WhatYouMean/releases/tag/v3.1.1) [Yanked]

### Changed

* Default API key built-in differently

## [v3.1.0](https://github.com/Clay-6/WhatYouMean/releases/tag/v3.1.0) [Yanked]

### Added

* `--verbose` flag to show all available data

### Fixed

* `--random` flag now actually randomises

## [v3.0.0](https://github.com/Clay-6/WhatYouMean/releases/tag/v3.0.0)

### Added

* Default API key

### Changed

* API source now Wordnik
* Update [clap](lib.rs/crates/clap) to 4.0.29

## [v2.0.0](https://github.com/Clay-6/WhatYouMean/releases/tag/v2.0.0)

### Added

* `--max` to limit number of definitions shown

### Changed

* API source now WordsAPI
* Better displayed info

## [v1.3.0](https://github.com/Clay-6/WhatYouMean/releases/tag/v1.3.0)

### Added

* `--sysnonyms` flag
* `--antonyms` flag

## [v1.2.0](https://github.com/Clay-6/WhatYouMean/releases/tag/v1.2.0)

### Changed

* `--show-examples` renamed to `--examples`
* `--phonetics` shows all available phonetics

### Removed

* `--lang-code`

## [v1.1.2](https://github.com/Clay-6/WhatYouMean/releases/tag/v1.1.2)

### Added

* `-p`/`--phonetic` flag to show word's phonetic representation

## [v1.1.1](https://github.com/Clay-6/WhatYouMean/releases/tag/v1.1.1)

### Added

* Coloured output
  * Can disable with `--no-colour` option

## [v1.1.0](https://crates.io/crates/whatyoumean/1.1.0)

### Added

* `--show-examples` flag
* `--lang` flag
* `--no-types` flag

## [v1.0.0](https://crates.io/crates/whatyoumean/1.0.0)

* Initial release

# Pi approximation via Monte Carlo
[![Build Status](https://travis-ci.com/petrSchreiber/pi-monte-carlo-rs.svg?branch=master)](https://travis-ci.com/petrSchreiber/pi-monte-carlo-rs)

_Educational project_

The main target of the project is to try to use Rust for Monte Carlo approximation of Pi and measure how long does it take for inexperienced Rustacean.

## Measured time
- learning about Monte Carlo approximation of Pi: 7 minutes
  - involved mostly googling what exactly it is

- inital naive implementation: 15 minutes
  - had to think how certain things are done in Rust
  - involves some googling of random number generation, casting

- adding command line parameters: 12 minutes
  - did it first time, involved googling
  - a bit of struggle with conversion of parameter to u64

 - adding unit tests: 8 minutes
   - involved finding out I did a mistake in the original code
   - some googling, rearranging of tests

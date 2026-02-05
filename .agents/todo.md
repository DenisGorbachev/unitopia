# TODO

* Rename Measure::new_const to Measure::new
  * Rationale:
    * Callers that need "into" can either call it manually or use Measure::from
  * Tasks:
    * Rename in this crate
    * Rename in dependencies

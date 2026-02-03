# Knowledge

## General

* Some unit name multipliers are a power of 10
  * Examples
    * Milliliter (10^-3)
* Some unit names are not a power of 10
  * Examples
    * Minute (60 seconds)
    * Day (24 hours)
* Some SI units have a unit name with a power of 10
  * Examples
    * Kilogram
* Some physical quantities have units that are not an exact multiple of each other
  * Examples
    * Time has "year" and "second" (this is why "leap seconds" exist)
  * Notes
    * This implies that we must accept a Unit instead of Quantity as a generic argument
* Some units are not even fixed-duration
  * Examples
    * Year can be 365 or 366 days

## Technical

* The `add` and `sub` ops on two measures must type-check only if their units are equal
* The `mul` and `div` ops on two measures must always type-check, but the resulting unit must be a multiplication or division of the units of the two measures
* I'm not sure about `rem`

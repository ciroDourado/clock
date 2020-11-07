# clock

Runs in any Unix-like OS, or Windows.

Its usage is simple, the new() "constructor" creates a reference of the clock structure.
But, since the default is to initialize every attribute as None, nothing can be used. The heart to execute any method from this clock is to verify each of the hours, minutes, and seconds values, with everything_is_ok() function. This clock cannot run with invalid values.

So, when any reference is created, it's required that every attribute must be initialized too.
set_a_valid_hour(), set_a_valid_minute() and set_a_valid_second() should read until a valid value is read.

After all this, run() should do the rest.
Hugs

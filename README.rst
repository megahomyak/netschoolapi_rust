**The development is frozen unless I will need the thing**

**One issue was found: salt should not be passed to the `login` endpoint** (upd: why did I write this? Is it really an issue?)

Known flaws
===========

* The year is only received at the log-in procedure, which may (and is highly likely to) cause logic errors on the back end when the student is going to a new grade. No protection from this was made even in the original library.

Modular Mixer System
====================

This is an open-source design for a completely modular audio mixer /
amplifier system.

The design is based around a single backplane which provides communication
between a master microcontroller with USB and a number of daughter boards,
each with its own subordinate microcontroller. Four stereo audio buses are
provided by the backplane for linking the different daughter boards together
along with both digital and analog power supplies.

Communication is via an RS-485 differential pair with automatic board
detection and configuration based loosely around the CFGIN/CFGOUT system
of the Amiga's Zorro bus.

The basic audio concept is that multiple "input" boards receive signals
from external devices, condition them, amplify them, whatever is needed, then
feeds those audio signals into one or more of the internal buses. At the same
time other boards may receive the signals from those buses and do other things
with them - maybe process them more and place them back on to another bus, or
amplify them and send them out to another external destination, etc.

For example a simple system may have a number of simple stereo audio
input boards which all feed into one bus. That bus is then read by a stereo
power amplifier board to connect to a pair of speakers.

A more complex setup may have some input boards feeding two separate buses.
One bus may go direct to a headphone amplifier board as a "monitor" output.
The other bus may then go into a graphic equalizer board which then feeds the
sound on to a third bus. That third bus could then go into a power amplifier
to drive speakers.

Which, and how many, buses any board can connect to is determined by the
design of each board. It is the responsibility of the board designer to
maintain isolation and audio separation between the different buses.

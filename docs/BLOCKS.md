## How should we store block states

### Definition
One "Block" can generate any number of "Block States".

A Block State is a variant of a block which encoded to a single number (id) in range `0-n`

Blocks need to map this id back into the "Block Variant" which is aware of what the id means to determine how to render the block

#### Example:

"Dirt" block that stores three potential states, Bare, Grassy or Snowy. It can also have a true/false flag that it is a half block.
It will have 3*2=6 separate block states, where 0,1,2 map to Bare, Grassy and Snowy, and 3,4,5 map to Half Bare, Half Grassy, Half Snowy.

### Problems

#### Need a way to migrate between block state versions

We can hash the contents of the blockstates list & identifiers and store that with the chunk data. If they're different, lookup a migration script from & to the hashes.

#### Need to a sequential list of all block states from blocks

Loop over all blocks and precompute all of their block states, storing them in a large fast lookup list, at the cost of memory consumption.

#### How to translate from/to Block State to Block Variant


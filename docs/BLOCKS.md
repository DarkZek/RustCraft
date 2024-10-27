### Definitions

#### WorldBlock
A block with enough context to create a specific block variant from.

Fields:
- BlockDefinition
- RelativeBlockId

#### BlockStates
A store for all blocks, used for lookups.

Fields:
- BlockId to BlockDefinition+RelativeBlockId map
- BlockDefinitionIndex to BlockId map

#### BlockId
An identifer for a specific variant of a block. Globally unique across every block and variant

Fields:
- index

#### RelativeBlockId
An identifer for a specific variant of a single block. Unique across every variant of a single block

Fields:
- index

#### BlockDefinition
Stores references to BlockImpl functions and constructs each BlockImpl struct from its RelativeBlockId to create the variant

#### BlockImpl
Defines how a block functions and the variants it covers

Fields:

Any fields for the variant, eg `is_snowy` for grass

#### VisualBlock
Stores all of the sources required to visualise a block, returned upon draw
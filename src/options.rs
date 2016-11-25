/// maximum image buffer size to attempt to decode
/// (the format supports larger images/animations, but this threshold helps to avoid malicious input to grab too much memory)
/// this is one frame of 1000 megapixels 8-bit RGB (it's 5 bytes per pixel because YCoCg uses 2 bytes each for Co/Cg)
/// (or 1000 frames of 1 megapixel)
const MAX_IMAGE_BUFFER_SIZE: u64 = 1000 * 1000000 * 5;

/// refuse to decode something which claims to have more frames than this
const MAX_FRAMES: i32 = 50000;

/// more repeats makes encoding more expensive, but results in better trees (smaller files)
const TREE_LEARN_REPEATS: i32 = 2;

const DEFAULT_MAX_PALETTE_SIZE: i32 = 512;

/// 8 byte improvement needed before splitting a MANIAC leaf node
const CONTEXT_TREE_SPLIT_THRESHOLD: i32 = 5461 * 8 * 8;

const CONTEXT_TREE_COUNT_DIV: i32 = 30;
const CONTEXT_TREE_MIN_SUBTREE_SIZE: i32 = 50;

/**************************************************/
/* DANGER ZONE: OPTIONS THAT CHANGE THE BITSTREAM */
/* If you modify these, the bitstream format      */
/* changes, so it is no longer compatible!        */
/**************************************************/

/// output the first K zoomlevels without building trees (too little data anyway to learn much from it)
const NB_NOLEARN_ZOOMS: i32 = 12;
/// this is enough to get a reasonable thumbnail/icon before the tree gets built/transmitted (at most 64x64 pixels)

/// faster decoding, less compression (disable multi-scale bitchances, use 24-bit rac)
const FAST_BUT_WORSE_COMPRESSION: i32 = 1;

/// bounds for node counters
const CONTEXT_TREE_MIN_COUNT: i32 = 1;
const CONTEXT_TREE_MAX_COUNT: i32 = 512;

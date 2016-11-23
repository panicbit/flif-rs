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

#[derive(Debug,Clone)]
pub struct Options {
    pub learn_repeats: i32,
    pub acb: i32,
    pub frame_delay: Vec<i32>,
    pub palette_size: i32,
    pub lookback: i32,
    pub divisor: i32,
    pub min_size: i32,
    pub split_threshold: i32,
    pub ycocg: i32,
    pub subtract_green: i32,
    pub plc: i32,
    pub frs: i32,
    pub alpha_zero_special: i32,
    pub loss: i32,
    pub adaptive: i32,
    pub predictor: [i32; 5],

    pub method: Option<::format::Encoding>,
    pub invisible_predictor: i32,
    pub alpha: i32,
    pub cutoff: i32,
    pub crc_check: i32,
    pub metadata: i32,
    pub color_profile: i32,
    pub scale: i32,
    pub resize_width: i32,
    pub resize_height: i32,
    pub fit: bool,
    pub overwrite: i32,
    pub just_add_loss: i32,
    // pub show_breakpoints: bool,
    pub no_full_decode: i32,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            learn_repeats: -1,
            acb: -1,
            frame_delay: vec![100],
            palette_size: -1,
            lookback: 1,
            divisor: CONTEXT_TREE_COUNT_DIV,
            min_size: CONTEXT_TREE_MIN_SUBTREE_SIZE,
            split_threshold: CONTEXT_TREE_SPLIT_THRESHOLD,
            ycocg: 1,
            subtract_green: 1,
            plc: 1,
            frs: 1,
            alpha_zero_special: 1,
            loss: 0,
            adaptive: 0,
            predictor: [-2, -2, -2, -2, -2],

            method: None,
            invisible_predictor: 2,
            alpha: 19,
            cutoff: 2,
            crc_check: -1,
            metadata: 1,
            color_profile: 1,
            // quality: 100,
            scale: 1,
            resize_width: 0,
            resize_height: 0,
            fit: false,
            overwrite: 0,
            just_add_loss: 0,
            // show_breakpoints: false,
            no_full_decode: 0,
        }
    }
}

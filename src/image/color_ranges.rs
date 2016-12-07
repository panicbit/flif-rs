use std::iter::FromIterator;
use image::ColorVal;

// TODO: Rustify API
pub trait ColorRanges {
    fn num_planes(&self) -> usize;
    fn min(&self, plane: usize) -> ColorVal;
    fn max(&self, plane: usize) -> ColorVal;
    fn minmax(&self, plane: usize, prev_planes: &[ColorVal], min_val: &mut ColorVal, max_val: &mut ColorVal) {
        *min_val = self.min(plane);
        *max_val = self.max(plane);
    }
    fn snap(&self, plane: usize, prev_planes: &[ColorVal], min_val: &mut ColorVal, max_val: &mut ColorVal, val: &mut ColorVal) {
        self.minmax(plane, prev_planes, min_val, max_val);

        if min_val > max_val {
            *max_val = *min_val;
        }

        debug_assert!(min_val <= max_val);

        if val > max_val {
            *val = *max_val;
        }

        if val < min_val {
            *val = *min_val;
        }

        debug_assert!(val <= max_val);
        debug_assert!(val >= min_val);
    }
    fn is_static(&self) -> bool {
        true
    }
    fn previous(&self) -> Option<&[Box<ColorRanges>]> {
        None
    }
}

pub struct StaticColorRanges {
    ranges: Vec<(ColorVal, ColorVal)>,
}

impl From<Vec<(ColorVal, ColorVal)>> for StaticColorRanges {
    fn from(ranges: Vec<(ColorVal, ColorVal)>) -> Self {
        StaticColorRanges {
            ranges: ranges
        }
    }
}

impl FromIterator<(ColorVal, ColorVal)> for StaticColorRanges {
    fn from_iter<T>(iter: T) -> Self where
        T: IntoIterator<Item=(ColorVal, ColorVal)>
    {
        StaticColorRanges {
            ranges: iter.into_iter().collect(),
        }
    }
}

impl ColorRanges for StaticColorRanges {
    fn num_planes(&self) -> usize {
        self.ranges.len()
    }
    fn min(&self, plane: usize) -> ColorVal {
        self.ranges.get(plane).map(|r| r.0).unwrap_or(0)
    }
    fn max(&self, plane: usize) -> ColorVal {
        self.ranges.get(plane).map(|r| r.1).unwrap_or(0)
    }
}

pub struct DupColorRanges {
    ranges: Box<ColorRanges>,
}

impl DupColorRanges {
    pub fn new(ranges: Box<ColorRanges>) -> Self {
        DupColorRanges {
            ranges: ranges,
        }
    }
}

// impl ColorRanges for DupColorRanges {
//     fn num_planes(&self) -> usize {
//         self.ranges.num_planes()
//     }
//     fn min(&self, plane: usize) -> ColorVal {
//         self.ranges.min(plane)
//     }
//     fn max(&self, plane: usize) -> ColorVal {
//         self.ranges.min(plane)
//     }
//     fn minmax(&self, plane: usize, prev_planes: &[ColorVal], min_val: &mut ColorVal, max_val: &mut ColorVal) {
//         self.ranges.minmax(plane, prev_planes, min_val, max_val)
//     }
//     // fn snap(&self, plane: usize, prev_planes: &[ColorVal], min_val: &mut ColorVal, max_val: &mut ColorVal, val: &mut ColorVal) {
//     //     self.ranges.snap(plane, prev_planes, min_val, max_val, val)
//     // }
//     fn is_static(&self) -> bool {
//         true
//     }
//     fn previous(&self) -> Option<&[Box<ColorRanges>]> {
//         None
//     }
// }

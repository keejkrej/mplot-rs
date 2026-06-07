use num_traits::{Num, NumCast};

use crate::graph::{GraphEntity, GraphMaker};

#[derive(Clone, Debug, Default)]
pub struct BoxplotData {
    pub groups: Vec<Vec<f64>>,
    pub horizontal: bool,
    pub whisker: Option<f64>,
    pub positions: Vec<f64>,
    pub width: Option<f64>,
    pub no_fliers: bool,
    pub patch_artist: bool,
}

pub struct Boxplot {
    symbol: String,
    horizontal: bool,
    whisker: Option<f64>,
    positions: Vec<f64>,
    width: Option<f64>,
    no_fliers: bool,
    patch_artist: bool,
    median_props: String,
    box_props: String,
    whisker_props: String,
    extra: String,
    drawn: Option<BoxplotData>,
    buffer: String,
}

impl Default for Boxplot {
    fn default() -> Self {
        Self::new()
    }
}

impl Boxplot {
    pub fn new() -> Self {
        Boxplot {
            symbol: String::new(),
            horizontal: false,
            whisker: None,
            positions: Vec::new(),
            width: None,
            no_fliers: false,
            patch_artist: false,
            median_props: String::new(),
            box_props: String::new(),
            whisker_props: String::new(),
            extra: String::new(),
            drawn: None,
            buffer: String::new(),
        }
    }

    pub fn draw<T>(&mut self, data: &Vec<Vec<T>>)
    where
        T: Num + NumCast + Copy,
    {
        let groups = data
            .iter()
            .map(|group| {
                group
                    .iter()
                    .map(|value| num_traits::cast(*value).unwrap_or(0.0))
                    .collect()
            })
            .collect();
        self.drawn = Some(BoxplotData {
            groups,
            horizontal: self.horizontal,
            whisker: self.whisker,
            positions: self.positions.clone(),
            width: self.width,
            no_fliers: self.no_fliers,
            patch_artist: self.patch_artist,
        });
    }

    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    pub fn set_horizontal(&mut self, flag: bool) -> &mut Self {
        self.horizontal = flag;
        if let Some(drawn) = &mut self.drawn {
            drawn.horizontal = flag;
        }
        self
    }

    pub fn set_whisker(&mut self, whisker: f64) -> &mut Self {
        self.whisker = Some(whisker);
        if let Some(drawn) = &mut self.drawn {
            drawn.whisker = Some(whisker);
        }
        self
    }

    pub fn set_positions(&mut self, positions: &[f64]) -> &mut Self {
        self.positions = positions.to_vec();
        if let Some(drawn) = &mut self.drawn {
            drawn.positions = self.positions.clone();
        }
        self
    }

    pub fn set_width(&mut self, width: f64) -> &mut Self {
        self.width = Some(width);
        if let Some(drawn) = &mut self.drawn {
            drawn.width = Some(width);
        }
        self
    }

    pub fn set_no_fliers(&mut self, flag: bool) -> &mut Self {
        self.no_fliers = flag;
        if let Some(drawn) = &mut self.drawn {
            drawn.no_fliers = flag;
        }
        self
    }

    pub fn set_patch_artist(&mut self, flag: bool) -> &mut Self {
        self.patch_artist = flag;
        if let Some(drawn) = &mut self.drawn {
            drawn.patch_artist = flag;
        }
        self
    }

    pub fn set_medianprops(&mut self, props: &str) -> &mut Self {
        self.median_props = props.to_string();
        self
    }

    pub fn set_boxprops(&mut self, props: &str) -> &mut Self {
        self.box_props = props.to_string();
        self
    }

    pub fn set_whiskerprops(&mut self, props: &str) -> &mut Self {
        self.whisker_props = props.to_string();
        self
    }

    pub fn set_extra(&mut self, extra: &str) -> &mut Self {
        self.extra = extra.to_string();
        self
    }
}

impl GraphMaker for Boxplot {
    fn get_buffer(&self) -> &String {
        &self.buffer
    }

    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    fn graph_entity(&self) -> Option<GraphEntity> {
        self.drawn.clone().map(GraphEntity::Boxplot)
    }
}

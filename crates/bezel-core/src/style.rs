//! Sparse `Style` (what the app sends) and absolute `ResolvedStyle` (what backends get).
//! Mirrors wit/style.wit. Layout half is the Taffy-compatible flexbox/grid subset (ADR-0006).

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Display {
    #[default]
    Flex,
    Grid,
    None,
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Direction {
    Row,
    #[default]
    Column,
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Align {
    #[default]
    Stretch,
    Start,
    Center,
    End,
    Baseline,
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Justify {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
    Auto,
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Dimension {
    #[default]
    Auto,
    Px(f32),
    Percent(f32),
    Fr(f32),
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorToken {
    Fg,
    FgMuted,
    Bg,
    Surface,
    Accent,
    AccentFg,
    Border,
    Ok,
    Warn,
    Bad,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Paint {
    Rgba(Rgba),
    Token(ColorToken),
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FontWeight {
    #[default]
    Regular,
    Medium,
    Semibold,
    Bold,
}
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Font {
    pub size: Option<f32>,
    pub weight: Option<FontWeight>,
    pub family: Option<String>,
    pub line_height: Option<f32>,
}

/// Sparse. Every field optional. Sent by apps; merged into a node's stored style.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Style {
    pub display: Option<Display>,
    pub direction: Option<Direction>,
    pub gap: Option<f32>,
    pub align_items: Option<Align>,
    pub justify_content: Option<Justify>,
    pub align_self: Option<Align>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<Dimension>,
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub min_width: Option<Dimension>,
    pub min_height: Option<Dimension>,
    pub max_width: Option<Dimension>,
    pub max_height: Option<Dimension>,
    pub padding: Option<Edges>,
    pub margin: Option<Edges>,
    pub overflow: Option<Overflow>,
    pub color: Option<Paint>,
    pub background: Option<Paint>,
    pub border_width: Option<f32>,
    pub border_color: Option<Paint>,
    pub radius: Option<f32>,
    pub opacity: Option<f32>,
    pub font: Option<Font>,
    pub visible: Option<bool>,
}

macro_rules! merge_fields { ($self:ident, $other:ident, $($f:ident),*) => { $( if $other.$f.is_some() { $self.$f = $other.$f.clone(); } )* } }

impl Style {
    pub fn merge(&mut self, other: &Style) {
        merge_fields!(
            self,
            other,
            display,
            direction,
            gap,
            align_items,
            justify_content,
            align_self,
            flex_grow,
            flex_shrink,
            flex_basis,
            width,
            height,
            min_width,
            min_height,
            max_width,
            max_height,
            padding,
            margin,
            overflow,
            color,
            background,
            border_width,
            border_color,
            radius,
            opacity,
            font,
            visible
        );
    }
}

/// Theme: token → colour, plus base font. Light/dark variants are two themes.
#[derive(Clone, Debug)]
pub struct Theme {
    pub tokens: fn(ColorToken) -> Rgba,
    pub base_font_size: f32,
    pub font_family: &'static str,
}

impl Default for Theme {
    fn default() -> Self {
        // Bezel default light theme from brand/bezel.design-tokens.json
        fn light(t: ColorToken) -> Rgba {
            match t {
                ColorToken::Fg => Rgba(0x10, 0x18, 0x20, 255),
                ColorToken::FgMuted => Rgba(0x5A, 0x66, 0x72, 255),
                ColorToken::Bg => Rgba(0xEE, 0xF1, 0xF4, 255),
                ColorToken::Surface => Rgba(0xFF, 0xFF, 0xFF, 255),
                ColorToken::Accent => Rgba(0x1F, 0x4F, 0xD8, 255),
                ColorToken::AccentFg => Rgba(255, 255, 255, 255),
                ColorToken::Border => Rgba(0xC5, 0xCD, 0xD6, 255),
                ColorToken::Ok => Rgba(0x1F, 0x75, 0x6A, 255),
                ColorToken::Warn => Rgba(0x9A, 0x5B, 0x00, 255),
                ColorToken::Bad => Rgba(0xB5, 0x36, 0x1F, 255),
            }
        }
        Theme {
            tokens: light,
            base_font_size: 14.0,
            font_family: "sans",
        }
    }
}

/// Absolute. Inherited properties (color, font) are filled from ancestors; everything else from defaults.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedStyle {
    pub display: Display,
    pub direction: Direction,
    pub gap: f32,
    pub align_items: Align,
    pub justify_content: Justify,
    pub align_self: Option<Align>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
    pub width: Dimension,
    pub height: Dimension,
    pub min_width: Dimension,
    pub min_height: Dimension,
    pub max_width: Dimension,
    pub max_height: Dimension,
    pub padding: Edges,
    pub margin: Edges,
    pub overflow: Overflow,
    pub color: Rgba,
    pub background: Option<Rgba>,
    pub border_width: f32,
    pub border_color: Rgba,
    pub radius: f32,
    pub opacity: f32,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub font_family: String,
    pub line_height: f32,
    pub visible: bool,
}

impl ResolvedStyle {
    pub fn from_theme(t: &Theme) -> Self {
        ResolvedStyle {
            display: Display::Flex,
            direction: Direction::Column,
            gap: 0.0,
            align_items: Align::Stretch,
            justify_content: Justify::Start,
            align_self: None,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Dimension::Auto,
            width: Dimension::Auto,
            height: Dimension::Auto,
            min_width: Dimension::Auto,
            min_height: Dimension::Auto,
            max_width: Dimension::Auto,
            max_height: Dimension::Auto,
            padding: Edges::default(),
            margin: Edges::default(),
            overflow: Overflow::Visible,
            color: (t.tokens)(ColorToken::Fg),
            background: None,
            border_width: 0.0,
            border_color: (t.tokens)(ColorToken::Border),
            radius: 0.0,
            opacity: 1.0,
            font_size: t.base_font_size,
            font_weight: FontWeight::Regular,
            font_family: t.font_family.into(),
            line_height: 1.5,
            visible: true,
        }
    }

    fn paint(p: Paint, t: &Theme) -> Rgba {
        match p {
            Paint::Rgba(c) => c,
            Paint::Token(k) => (t.tokens)(k),
        }
    }

    /// Apply one node's sparse style on top of the current resolved (inherited) state.
    /// Only `color` and `font` inherit; layout/box properties reset per node.
    pub fn apply(&mut self, s: &Style, t: &Theme) {
        let inherited_color = self.color;
        let (fs, fw, ff, lh) = (
            self.font_size,
            self.font_weight,
            self.font_family.clone(),
            self.line_height,
        );
        *self = ResolvedStyle {
            color: inherited_color,
            font_size: fs,
            font_weight: fw,
            font_family: ff,
            line_height: lh,
            ..ResolvedStyle::from_theme(t)
        };
        if let Some(v) = s.display {
            self.display = v
        }
        if let Some(v) = s.direction {
            self.direction = v
        }
        if let Some(v) = s.gap {
            self.gap = v
        }
        if let Some(v) = s.align_items {
            self.align_items = v
        }
        if let Some(v) = s.justify_content {
            self.justify_content = v
        }
        if s.align_self.is_some() {
            self.align_self = s.align_self
        }
        if let Some(v) = s.flex_grow {
            self.flex_grow = v
        }
        if let Some(v) = s.flex_shrink {
            self.flex_shrink = v
        }
        if let Some(v) = s.flex_basis {
            self.flex_basis = v
        }
        if let Some(v) = s.width {
            self.width = v
        }
        if let Some(v) = s.height {
            self.height = v
        }
        if let Some(v) = s.min_width {
            self.min_width = v
        }
        if let Some(v) = s.min_height {
            self.min_height = v
        }
        if let Some(v) = s.max_width {
            self.max_width = v
        }
        if let Some(v) = s.max_height {
            self.max_height = v
        }
        if let Some(v) = s.padding {
            self.padding = v
        }
        if let Some(v) = s.margin {
            self.margin = v
        }
        if let Some(v) = s.overflow {
            self.overflow = v
        }
        if let Some(v) = s.color {
            self.color = Self::paint(v, t)
        }
        if let Some(v) = s.background {
            self.background = Some(Self::paint(v, t))
        }
        if let Some(v) = s.border_width {
            self.border_width = v
        }
        if let Some(v) = s.border_color {
            self.border_color = Self::paint(v, t)
        }
        if let Some(v) = s.radius {
            self.radius = v
        }
        if let Some(v) = s.opacity {
            self.opacity = v
        }
        if let Some(f) = &s.font {
            if let Some(v) = f.size {
                self.font_size = v
            }
            if let Some(v) = f.weight {
                self.font_weight = v
            }
            if let Some(v) = &f.family {
                self.font_family = v.clone()
            }
            if let Some(v) = f.line_height {
                self.line_height = v
            }
        }
        if let Some(v) = s.visible {
            self.visible = v
        }
    }
}

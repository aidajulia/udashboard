// uDashBoard: featherweight dashboard application.
//
// Copyright (C) 2019  Brandon Lewis
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program.  If not, see
// <https://www.gnu.org/licenses/>.

// Internal Representation used by all rendering backends

use std::{
    collections::HashMap,
    f64::consts::PI,
};

use serde::{Deserialize};

pub type Float = f64;

#[derive(Deserialize, Debug, Clone)]
pub enum Label {
    None,
    Plain(String),
    Sized(String, Float),
    Styled(String, Float, Color),
}

impl Label {
    pub fn append(&self, text: &String) -> Label {
        match self {
            Label::None => Label::Plain(text.clone()),
            Label::Plain(t) => Label::Plain(t.clone() + text),
            Label::Sized(t, s) => Label::Sized(t.clone() + text, *s),
            Label::Styled(t, s, c) => Label::Styled(t.clone() + text, *s, *c)
        }
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Screen {
    pub width: Float,
    pub height: Float
}

#[derive(Deserialize, Debug, Clone)]
pub enum Divisions {
    None,
    Uniform(Vec<Float>),
    MajorMinor(Vec<(Label, Float)>, Vec<Float>),
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Format {
    Integer(u32),
    Decimal(u32, u32),
    Time(Float),
}

impl Format {
    pub fn format_value(&self, value: Float) -> String {
        match self {
            Format::Integer(width) => format!(
                "{value:width$}",
                value = value as i32,
                width = *width as usize
            ),
            Format::Decimal(int, dec) => format!(
                "{value:int$.dec$}",
                value = value,
                int = *int as usize,
                dec = *dec as usize
            ),
            Format::Time(scale) => {
                let seconds = value * scale;
                let subseconds = (seconds * 100.0) as i32;
                let minutes = (seconds as i32) / 60;
                let hours = minutes / 60;

                format!(
                    "{h:2}:{m:02}:{s:02}.{ss:02}",
                    h = hours,
                    m = minutes % 60,
                    s = (seconds as i32) % 60,
                    ss = subseconds % 100
                )
            }
        }
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum GaugeStyle {
    IndicatorOnly,
    Outline,
    Filled,
    Dashed
}

#[derive(Deserialize, Debug, Clone)]
pub struct Scale(pub Float, pub Float, pub Divisions, pub GaugeStyle);

impl Scale {
    pub fn range(&self) -> Float {
        self.1 - self.0
    }

    pub fn to_percent(&self, val: Float) -> Float {
        ((val - self.0) / self.range()).max(self.0).min(self.1)
    }

    pub fn to_angle(&self, val: Float) -> Float {
        1.25 * PI * (self.to_percent(val) - 0.5)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Lamp {
    Round,
    Rect,
    RoundedRect,
    Image(String)
}

#[derive(Deserialize, Debug, Clone)]
pub enum GaugeType {
    Dial(Scale),
    VerticalBar(Scale),
    HorizontalBar(Scale),
    VerticalWedge(Scale),
    HorizontalWedge(Scale),
    IdiotLight(Lamp),
    Text(Format, GaugeStyle),
}

#[derive(Deserialize, Debug, Clone)]
pub enum Unit {
    None,
    Named(String),
}

#[derive(Deserialize, Debug, Hash, Clone, PartialEq, Eq)]
pub enum State {
    Default,
    Alarm(String)
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Test {
    Always,
    Never,
    LessThan(Float),
    GreaterThan(Float),
    Equal(Float),
    Between(Float, Float)
}

#[derive(Deserialize, Debug, Clone)]
pub struct When(String, Test, State);

pub type Logic = Vec<When>;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Point {
    pub x: Float,
    pub y: Float
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Bounds {
    pub x: Float,
    pub y: Float,
    pub width: Float,
    pub height: Float
}

impl Bounds {
    pub fn center(&self) -> (Float, Float) {
        ((self.x + self.width * 0.5), (self.y + self.height * 0.5))
    }

    pub fn radius(&self) -> Float {
        self.width.min(self.height) * 0.5
    }

    pub fn inset(&self, pixels: Float) -> Bounds {
        Bounds {
            x: self.x + pixels,
            y: self.y + pixels,
            width: self.width - pixels * 2.0,
            height: self.height - pixels * 2.0
        }
    }

    pub fn top_left(&self) -> (Float, Float) {
        (self.x, self.y)
    }

    pub fn top_right(&self) -> (Float, Float) {
        (self.x + self.width, self.y)
    }

    pub fn bottom_left(&self) -> (Float, Float) {
        (self.x, self.y + self.height)
    }

    pub fn bottom_right(&self) -> (Float, Float) {
        (self.x + self.width, self.y + self.height)
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Color(pub Float, pub Float, pub Float, pub Float);

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Pattern {
    Hidden,
    Solid(Color),
    SlowBlink(Color),
    FastBlink(Color),
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Style {
    pub background: Pattern,
    pub foreground: Pattern,
    pub indicator: Pattern
}

impl Style {
    // define a crazy style for debugging.
    pub fn default() -> Style {
        Style {
            background: Pattern::SlowBlink(Color(1.0, 0.0, 0.0, 1.0)),
            foreground: Pattern::Solid(Color(1.0, 0.0, 0.0, 1.0)),
            indicator: Pattern::FastBlink(Color(1.0, 0.0, 1.0, 1.0))
        }
    }
}

pub type StyleSet = HashMap<State, Style>;

#[derive(Deserialize, Debug, Clone)]
pub struct Gauge {
    pub name: String,
    pub label: Label,
    pub kind: GaugeType,
    pub channel: String,
    pub bounds: Bounds,
    pub styles: StyleSet
}

#[derive(Deserialize, Debug, Clone)]
pub enum Function {
    Identity,
    Scale(Float),
    Linear(Float, Float),
    Polynomial(Vec<Float>)
}

#[derive(Deserialize, Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub units: Unit
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub screen: Screen,
    pub channels: Vec<Channel>,
    pub pages: Vec<Vec<Gauge>>,
    pub logic: Logic,
}

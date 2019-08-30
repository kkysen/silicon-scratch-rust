use serde_json::Deserialize;
use std::collections::HashMap;
use zip::read::ZipFile;

pub struct AssetId {
    // TODO
}

pub enum ImageDataFormat {
    PNG,
    SVG,
    JPEG,
    JPG,
    BMP,
    GIF,
}

pub enum AudioDataFormat {
    WAV,
    WAVE,
    MP3,
}

pub struct Asset<DataFormat> {
    pub asset_id: AssetId,
    pub data_format: DataFormat,
    pub name: String,
    pub file: ZipFile,
}

pub struct Costume {
    pub asset: Asset<ImageDataFormat>,
    pub bitmap_resolution: u32,
    pub rotation_center: Vec2<double>,
}

pub struct Sound {
    pub asset: Asset<AudioDataFormat>,
    pub rate: u32,
    pub sample_count: u32,
}

pub enum Number {
    UInt(u64),
    Int(i64),
    Float(double),
}

pub enum NumberOrString {
    Number(Number),
    String(String),
}

pub enum ScalarValue {
    Bool(bool),
    Number(Number),
    String(String),
}

pub struct Broadcast {
    pub name: String,
}

pub enum Value {
    Scalar(ScalarValue),
    List(Vec<Value>),
    Broadcast(Broadcast),
}

pub struct Variable {
    pub name: String,
    pub value: Value,
    pub on_cloud: bool,
}

pub struct Color {
    // TODO
}

pub type NumPrimitive = NumberOrString;

pub type ColorPrimitive = Color;

pub type TextPrimitive = String;

pub struct VariablePrimitive {
    name: String,
    id: String,
    position: Vec2<double>,
}

#[serde(tag = "kind")]
pub enum Primitive {
    Num(NumPrimitive),
    Color(ColorPrimitive),
    Text(TextPrimitive),
    Variable(VariablePrimitive),
}

pub enum BlockCategory {
    Motion,
    Look,
    Sound,
    Event,
    Control,
    Sensing,
    Operator,
    Variable,
    Block,
}

pub enum CategoryOpCode {}

pub struct OpCode {
    pub category: BlockCategory,
    pub op_code: CategoryOpCode,
}

pub enum Shadow {
    UnObscured,
    None,
    Obscured,
}

pub struct Input {
    pub shadow: Shadow,
    pub args: Vec<Primitive>,
}

pub struct Field {
    // TODO
}

pub struct Block {
    pub op_code: OpCode,
    pub next: Option<&Block>,
    pub parent: Option<&Block>,
    pub comment: Option<String>,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Field>,
    pub top_level: bool,
    pub shadow: bool,
    pub position: Vec2<double>,
}

pub struct Comment {
    pub block: Option<&Block>,
    pub text: String,
    pub minimized: bool,
    pub position: Vec2<double>,
}

pub struct Target {
    pub current_costume: Costume,
    pub blocks: Vec<Block>,
    pub variables: Vec<Variable>,
    pub lists: Vec<List>,
    pub broadcasts: Vec<Broadcast>,
    pub comments: Vec<Comment>,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub volume: double,
}

pub enum VideoState {
    On,
    Off,
    OnFlipped,
}

pub struct Stage {
    pub target: Target,
    pub tempo: double,
    pub video_transparency: double,
    pub video_state: VideoState,
}

pub enum RotationStyle {
    AllAround,
    DontRotate,
    LeftRight,
}

pub struct Sprite {
    pub target: Target,
    pub name: String,
    pub visible: bool,
    pub position: Vec2<double>,
    pub size: double,
    pub direction: double,
    pub draggable: boolean,
    pub rotation_style: RotationStyle,
    pub layer_order: u32,
}

pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: String,
}

pub struct Meta {
    pub version: SemVer,
    pub vm: SemVer,
    pub user_agent: String,
}

pub struct Targets {
    pub stage: Stage,
    pub sprites: Vec<Sprite>,
}

pub struct Project {
    pub targets: Targets,
    pub meta: Meta,
}

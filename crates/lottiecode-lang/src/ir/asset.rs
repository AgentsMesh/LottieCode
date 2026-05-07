//! IR Asset —— 图像 / 预合成 / 音频 / 数据资源。

use super::layer::IrLayer;

#[derive(Debug, Clone)]
pub enum IrAsset {
    Image {
        id: String,
        width: u32,
        height: u32,
        path: String,
        embedded: bool,
    },
    Precomp {
        id: String,
        width: u32,
        height: u32,
        fps: f64,
        layers: Vec<IrLayer>,
    },
    Sound {
        id: String,
        path: String,
        embedded: bool,
    },
    Data {
        id: String,
        path: String,
        embedded: bool,
    },
}

impl IrAsset {
    pub fn id(&self) -> &str {
        match self {
            IrAsset::Image { id, .. }
            | IrAsset::Precomp { id, .. }
            | IrAsset::Sound { id, .. }
            | IrAsset::Data { id, .. } => id,
        }
    }
}

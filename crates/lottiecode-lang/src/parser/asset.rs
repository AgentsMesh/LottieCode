//! Asset 顶层声明 + image 层解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// `asset name { image = "path", width = N, height = N }`
    pub(crate) fn parse_asset(&mut self) -> Result<AssetDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Asset)?;
        let name = self.expect_ident()?;
        let attributes = self.parse_brace_attributes()?;

        // 检查 image / precomp 字段决定 kind
        let has_image = attributes.iter().any(|a| a.key == "image");
        let has_precomp = attributes.iter().any(|a| a.key == "precomp");
        let has_sound = attributes.iter().any(|a| a.key == "sound" || a.key == "audio");
        let has_data = attributes.iter().any(|a| a.key == "data");
        let kind = if has_image {
            AssetKind::Image
        } else if has_precomp {
            AssetKind::Precomp
        } else if has_sound {
            AssetKind::Sound
        } else if has_data {
            AssetKind::Data
        } else {
            return Err(LottieError::new(
                ErrorKind::InvalidValue,
                "asset 需要包含 image / precomp / sound / data 字段之一",
                Some(span),
            ));
        };

        Ok(AssetDecl {
            name,
            kind,
            attributes,
            span,
        })
    }

    /// `image my-name { asset = ref-name, position = [x, y] }`
    pub(crate) fn parse_image_layer(&mut self) -> Result<ImageLayerDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Image)?;
        let name = self.expect_ident()?;
        let attributes = self.parse_brace_attributes()?;
        Ok(ImageLayerDecl {
            name,
            attributes,
            span,
        })
    }
}

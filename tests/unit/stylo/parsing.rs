/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub use cssparser::ToCss;
pub use style::properties::{DeclaredValue, PropertyDeclaration, PropertyDeclarationBlock, Importance};

pub fn shorthand_properties_to_string(properties: Vec<PropertyDeclaration>) -> String {
    let block = PropertyDeclarationBlock {
        declarations: properties.into_iter().map(|d| (d, Importance::Normal)).collect(),
        important_count: 0,
    };

    block.to_css_string()
}

macro_rules! single_vec_keyword_value {
    ($name:ident, $kw:ident) => {
        DeclaredValue::Value($name::SpecifiedValue(
            vec![$name::single_value::SpecifiedValue::$kw]
        ))
    };
}

mod mask_shorthands {
    use style::properties::longhands::mask_mode;
    use style::properties::longhands::mask_repeat;
    use style::properties::longhands::mask_clip;
    use style::properties::longhands::mask_origin;
    use style::properties::longhands::mask_composite;
    use style::properties::longhands::mask_position;
    use style::properties::longhands::mask_size;
    use style::properties::longhands::mask_image;
    use super::*;

    #[test]
    fn test1() {
        let mut properties = Vec::new();

        let mode = single_vec_keyword_value!(mask_mode, alpha);
        properties.push(PropertyDeclaration::MaskMode(mode));
    }
}


use crate::types::BrickType;

#[derive(Clone, Copy)]
pub struct BrickSpec {
    pub brick_type: BrickType,
    pub width: f32,
    pub height: f32,
    pub default_y_offset: f32,
}

pub const BRICK_WIDTH: f32 = 348.181;

pub const H0_SPEC: BrickSpec = BrickSpec {
    brick_type: BrickType::H0Collapsed,
    width: BRICK_WIDTH,
    height: 16.0,
    default_y_offset: 0.0,
};

pub const H1_BASE_SPEC: BrickSpec = BrickSpec {
    brick_type: BrickType::H1Base,
    width: BRICK_WIDTH,
    height: 72.95,
    default_y_offset: 0.13,
};

pub const H1_CONTROL_SPEC: BrickSpec = BrickSpec {
    brick_type: BrickType::H1Control,
    width: BRICK_WIDTH,
    height: 72.95,
    default_y_offset: 0.22,
};

pub const H2_BASE_SPEC: BrickSpec = BrickSpec {
    brick_type: BrickType::H2Base,
    width: BRICK_WIDTH,
    height: 94.748,
    default_y_offset: 0.09,
};

pub const H2_CONTROL_SPEC: BrickSpec = BrickSpec {
    brick_type: BrickType::H2Control,
    width: BRICK_WIDTH,
    height: 94.748,
    default_y_offset: 0.17,
};

pub const H3_BASE_SPEC: BrickSpec = BrickSpec {
    brick_type: BrickType::H3Base,
    width: BRICK_WIDTH,
    height: 94.748,
    default_y_offset: 0.03,
};

pub fn spec_for(brick_type: BrickType) -> BrickSpec {
    match brick_type {
        BrickType::H0Collapsed => H0_SPEC,
        BrickType::H1Base => H1_BASE_SPEC,
        BrickType::H2Base => H2_BASE_SPEC,
        BrickType::H3Base => H3_BASE_SPEC,
        BrickType::H1Control => H1_CONTROL_SPEC,
        BrickType::H2Control => H2_CONTROL_SPEC,
    }
}

macro_rules! impl_brick_wrapper {
    ($name:ident, $spec:expr) => {
        #[derive(Clone, PartialEq)]
        pub struct $name {
            pub base: crate::brick::base::BaseBrick,
        }

        impl std::ops::Deref for $name {
            type Target = crate::brick::base::BaseBrick;

            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    base: crate::brick::base::BaseBrick {
                        offset: (crate::brick::base::DEFAULT_X_OFFSET, $spec.default_y_offset),
                        ..crate::brick::base::BaseBrick::default()
                    },
                }
            }
        }

        impl crate::common::Brick for $name {
            fn get_type(&self) -> crate::types::BrickType {
                $spec.brick_type
            }

            fn get_dimensions(&self) -> (u32, u32) {
                ($spec.width as u32, $spec.height as u32)
            }
        }
    };
}

pub mod base;
pub mod deserialize;
pub mod factory;
pub mod h0;
pub mod h1_base;
pub mod h1_control;
pub mod h2_base;
pub mod h2_control;
pub mod h3_base;
pub mod serialize;

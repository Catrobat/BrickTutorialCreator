use crate::brick::base::BaseBrick;
use crate::brick::h0::BrickH0;
use crate::brick::h1_base::BrickH1Base;
use crate::brick::h1_control::BrickH1Control;
use crate::brick::h2_base::BrickH2Base;
use crate::brick::h2_control::BrickH2Control;
use crate::brick::h3_base::BrickH3Base;
use crate::brick::spec_for;
use crate::common::BrickRenderable;
use crate::types::BrickType;

pub fn default_y_offset(brick_type: BrickType) -> f32 {
    spec_for(brick_type).default_y_offset
}

pub fn brick_from_base(brick_type: BrickType, base: BaseBrick) -> Box<dyn BrickRenderable> {
    match brick_type {
        BrickType::H0Collapsed => Box::new(BrickH0 { base }),
        BrickType::H1Base => Box::new(BrickH1Base { base }),
        BrickType::H2Base => Box::new(BrickH2Base { base }),
        BrickType::H3Base => Box::new(BrickH3Base { base }),
        BrickType::H1Control => Box::new(BrickH1Control { base }),
        BrickType::H2Control => Box::new(BrickH2Control { base }),
    }
}

pub fn default_brick(brick_type: BrickType) -> Box<dyn BrickRenderable> {
    match brick_type {
        BrickType::H0Collapsed => Box::new(BrickH0::default()),
        BrickType::H1Base => Box::new(BrickH1Base::default()),
        BrickType::H2Base => Box::new(BrickH2Base::default()),
        BrickType::H3Base => Box::new(BrickH3Base::default()),
        BrickType::H1Control => Box::new(BrickH1Control::default()),
        BrickType::H2Control => Box::new(BrickH2Control::default()),
    }
}

pub fn clone_brick(brick: &dyn BrickRenderable) -> Box<dyn BrickRenderable> {
    brick_from_base(brick.get_type(), brick.deref().clone())
}

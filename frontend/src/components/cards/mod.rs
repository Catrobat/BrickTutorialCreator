pub mod brick_type_card;
pub mod color_card;

#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_CARD_CLASS: &str = "max-[900px]:border-2";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_CARD_CONTENT_CLASS: &str = "mt-auto";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_CARD_SWATCH: &str = "hidden h-5 w-5 rounded-full max-[900px]:block";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_CARD_SWATCH_ROW: &str = "flex w-full items-stretch gap-1 max-[900px]:hidden";
#[cfg(target_arch = "wasm32")]
pub(crate) const COLOR_CARD_SWATCH_VALUE: &str = "flex min-w-0 flex-1 items-center justify-center overflow-hidden rounded-[3px] px-1 py-px text-center text-[10px] leading-none max-[900px]:px-[3px] max-[900px]:py-px max-[900px]:text-[10px]";

#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_TYPE_CARD_CLASS: &str = "brick-type-mobile-card";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_TYPE_CARD_TITLE_CLASS: &str = "brick-type-mobile-card-title";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_TYPE_CARD_PREVIEW_WRAP: &str =
    "h-[22px] overflow-hidden max-[900px]:h-[12px]";
#[cfg(target_arch = "wasm32")]
pub(crate) const BRICK_TYPE_CARD_PREVIEW_IMAGE: &str =
    "block h-full w-full object-contain object-left";

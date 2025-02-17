use crate::gpio::{self, Alternate};

/// Output channels markers
pub trait CPin<TIM, const C: u8> {}

/// Complementary output channels markers
pub trait NCPin<TIM, const C: u8> {}

pub trait EtrPin<TIM> {}
pub trait BkinPin<TIM> {}

/// Channel wrapper
pub struct Ch<const C: u8>;
pub const C1: u8 = 0;
pub const C2: u8 = 1;
pub const C3: u8 = 2;
pub const C4: u8 = 3;

macro_rules! channel_impl {
    ( $( $trait:ident<$TIM:ident $(, $C:ident)?>, $PINX:ident, $AF:literal; )+ ) => {
        $(
            impl<Otype> $trait<crate::pac::$TIM $(, $C)?> for gpio::$PINX<Alternate<$AF, Otype>> { }
        )+
    };
}

// Autogenerated from Cube-MX
#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
channel_impl! {
    CPin<TIM2, C1>, PA0, 1;
    EtrPin<TIM2>, PA0, 1;
    CPin<TIM2, C2>, PA1, 1;
    CPin<TIM2, C3>, PA2, 1;
    CPin<TIM2, C4>, PA3, 1;
    CPin<TIM2, C1>, PA5, 1;
    EtrPin<TIM2>, PA5, 1;
    CPin<TIM3, C1>, PA6, 2;
    CPin<TIM3, C2>, PA7, 2;
    CPin<TIM2, C1>, PA15, 1;
    EtrPin<TIM2>, PA15, 1;
    CPin<TIM3, C3>, PB0, 2;
    CPin<TIM3, C4>, PB1, 2;
    CPin<TIM2, C2>, PB3, 1;
    CPin<TIM3, C1>, PB4, 2;
    CPin<TIM3, C2>, PB5, 2;
    CPin<TIM4, C1>, PB6, 2;
    CPin<TIM4, C2>, PB7, 2;
    CPin<TIM4, C3>, PB8, 2;
    CPin<TIM10, C1>, PB8, 3;
    CPin<TIM4, C4>, PB9, 2;
    CPin<TIM2, C3>, PB10, 1;
    CPin<TIM2, C4>, PB11, 1;
    CPin<TIM3, C1>, PC6, 2;
    CPin<TIM3, C2>, PC7, 2;
    CPin<TIM3, C3>, PC8, 2;
    CPin<TIM3, C4>, PC9, 2;
    EtrPin<TIM3>, PD2, 2;
    CPin<TIM4, C1>, PD12, 2;
    CPin<TIM4, C2>, PD13, 2;
    CPin<TIM4, C3>, PD14, 2;
    CPin<TIM4, C4>, PD15, 2;
    EtrPin<TIM4>, PE0, 2;
    CPin<TIM9, C1>, PE5, 3;
    CPin<TIM9, C2>, PE6, 3;
    EtrPin<TIM1>, PE7, 1;
    NCPin<TIM1, C1>, PE8, 1;
    CPin<TIM1, C1>, PE9, 1;
    NCPin<TIM1, C2>, PE10, 1;
    CPin<TIM1, C2>, PE11, 1;
    NCPin<TIM1, C3>, PE12, 1;
    CPin<TIM1, C3>, PE13, 1;
    CPin<TIM1, C4>, PE14, 1;
    BkinPin<TIM1>, PE15, 1;
}

#[cfg(any(
    feature = "gpio-f401",
    feature = "gpio-f410",
    feature = "gpio-f411",
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
channel_impl! {
    CPin<TIM5, C1>, PA0, 2;
    CPin<TIM5, C2>, PA1, 2;
    CPin<TIM5, C3>, PA2, 2;
    CPin<TIM9, C1>, PA2, 3;
    CPin<TIM5, C4>, PA3, 2;
    CPin<TIM9, C2>, PA3, 3;
    BkinPin<TIM1>, PA6, 1;
    NCPin<TIM1, C1>, PA7, 1;
    CPin<TIM1, C1>, PA8, 1;
    CPin<TIM1, C2>, PA9, 1;
    CPin<TIM1, C3>, PA10, 1;
    CPin<TIM1, C4>, PA11, 1;
    EtrPin<TIM1>, PA12, 1;
    NCPin<TIM1, C2>, PB0, 1;
    NCPin<TIM1, C3>, PB1, 1;
    CPin<TIM11, C1>, PB9, 3;
    BkinPin<TIM1>, PB12, 1;
    NCPin<TIM1, C1>, PB13, 1;
    NCPin<TIM1, C2>, PB14, 1;
    NCPin<TIM1, C3>, PB15, 1;
}

#[cfg(feature = "gpio-f410")]
channel_impl! {
    CPin<TIM5, C4>, PB11, 2;
    CPin<TIM5, C1>, PB12, 2;
    CPin<TIM9, C1>, PC4, 3;
    CPin<TIM9, C2>, PC5, 3;
    CPin<TIM5, C2>, PC10, 2;
    CPin<TIM5, C3>, PC11, 2;
    CPin<TIM11, C1>, PC12, 3;
}

#[cfg(any(
    feature = "gpio-f412",
    feature = "gpio-f413",
    feature = "gpio-f417",
    feature = "gpio-f427",
    feature = "gpio-f446",
    feature = "gpio-f469"
))]
channel_impl! {
    EtrPin<TIM8>, PA0, 3;
    NCPin<TIM8, C1>, PA5, 3;
    BkinPin<TIM8>, PA6, 3;
    CPin<TIM13, C1>, PA6, 9;
    NCPin<TIM8, C1>, PA7, 3;
    CPin<TIM14, C1>, PA7, 9;
    NCPin<TIM8, C2>, PB0, 3;
    NCPin<TIM8, C3>, PB1, 3;
    NCPin<TIM8, C2>, PB14, 3;
    CPin<TIM12, C1>, PB14, 9;
    NCPin<TIM8, C3>, PB15, 3;
    CPin<TIM12, C2>, PB15, 9;
    CPin<TIM8, C1>, PC6, 3;
    CPin<TIM8, C2>, PC7, 3;
    CPin<TIM8, C3>, PC8, 3;
    CPin<TIM8, C4>, PC9, 3;
    CPin<TIM10, C1>, PF6, 3;
    CPin<TIM11, C1>, PF7, 3;
    CPin<TIM13, C1>, PF8, 9;
    CPin<TIM14, C1>, PF9, 9;
}

#[cfg(any(feature = "gpio-f412", feature = "gpio-f413"))]
channel_impl! {
    CPin<TIM5, C1>, PF3, 2;
    CPin<TIM5, C2>, PF4, 2;
    CPin<TIM5, C3>, PF5, 2;
    EtrPin<TIM1>, PF10, 1;
    CPin<TIM5, C4>, PF10, 2;
    EtrPin<TIM8>, PF11, 3;
    BkinPin<TIM8>, PF12, 3;
}

#[cfg(any(feature = "gpio-f417", feature = "gpio-f427", feature = "gpio-f469"))]
channel_impl! {
    CPin<TIM12, C1>, PH6, 9;
    CPin<TIM12, C2>, PH9, 9;
    CPin<TIM5, C1>, PH10, 2;
    CPin<TIM5, C2>, PH11, 2;
    CPin<TIM5, C3>, PH12, 2;
    NCPin<TIM8, C1>, PH13, 3;
    NCPin<TIM8, C2>, PH14, 3;
    NCPin<TIM8, C3>, PH15, 3;
    CPin<TIM5, C4>, PI0, 2;
    CPin<TIM8, C4>, PI2, 3;
    EtrPin<TIM8>, PI3, 3;
    BkinPin<TIM8>, PI4, 3;
    CPin<TIM8, C1>, PI5, 3;
    CPin<TIM8, C2>, PI6, 3;
    CPin<TIM8, C3>, PI7, 3;
}

#[cfg(feature = "gpio-f446")]
channel_impl! {
    CPin<TIM2, C4>, PB2, 1;
    CPin<TIM2, C1>, PB8, 1;
    EtrPin<TIM2>, PB8, 1;
    CPin<TIM2, C2>, PB9, 1;
}

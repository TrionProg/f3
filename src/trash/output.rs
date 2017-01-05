use ::stm32::gpio::functions::output::OutputPin;
pub use ::stm32::gpio::functions::output::OutputFun;
use core::marker::PhantomData;

create_function_output!(
    Output
);

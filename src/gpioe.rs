use memory_map;

create_gpio!(GPIOE, gpio_rb:Gpio,
    enable_gpio:{
        let rcc = deref_mut::<memory_map::rcc::Rcc>(memory_map::addresses::RCC);
        rcc.ahbenr.modify(|_, w| w.iopeen(true));
    },
    disable_gpio:{
        let rcc = deref_mut::<memory_map::rcc::Rcc>(memory_map::addresses::RCC);
        rcc.ahbenr.modify(|_, w| w.iopeen(false));
    }
);

use ::stm32::gpio::Pin;
use ::stm32::gpio::functions::output::OutputPin;

create_pin!(E8,8);
implement_output!(
    E8,
    enable_output:{
        let gpio_rb=Self::get_gpio_rb_mut();
        gpio_rb.moder.modify(|_, w| { w.moder(Self::get_index(),0b01) });
    },
    disable_output:{
        let gpio_rb=Self::get_gpio_rb_mut();
        gpio_rb.moder.modify(|_, w| { w.moder(Self::get_index(),0b00) });
    },
    turn_on:{
        let gpio_rb=Self::get_gpio_rb_mut();
        gpio_rb.bsrr.write(|w| w.bs8(true));
    },
    turn_off:{
        let gpio_rb=Self::get_gpio_rb_mut();
        gpio_rb.bsrr.write(|w| w.br8(true));
    }
);

create_pin!(E9,9);
implement_output!(
    E9,
    enable_output:{
        let gpio_rb=Self::get_gpio_rb_mut();
        gpio_rb.moder.modify(|_, w| { w.moder(Self::get_index(),0b01) });
    },
    disable_output:{
        let gpio_rb=Self::get_gpio_rb_mut();
        gpio_rb.moder.modify(|_, w| { w.moder(Self::get_index(),0b00) });
    },
    turn_on:{
        let gpio_rb=Self::get_gpio_rb_mut();
        gpio_rb.bsrr.write(|w| w.bs9(true));
    },
    turn_off:{
        let gpio_rb=Self::get_gpio_rb_mut();
        gpio_rb.bsrr.write(|w| w.br9(true));
    }
);

/*

impl Pin<peripheral::gpio::Gpio> for E0{
    fn get_index(&self) -> u8{
        0
    }

    fn get_register_block(&self) -> &'static peripheral::gpio::Gpio{
        unsafe { peripheral::deref(peripheral::GPIOE) }
    }

    fn get_register_block_mut(&self) -> &'static mut peripheral::gpio::Gpio{
        unsafe { peripheral::deref_mut(peripheral::GPIOE) }
    }
}

use ::stm32::gpio::functions::input::InputPin;

impl InputPin<peripheral::gpio::Gpio> for E0 {
    fn config(self) {
        self.get_register_block_mut().moder.modify(|_, w| { w.moder(self.get_index(), 0b00) });
    }
}

trait InputFun<Pin:InputPin<peripheral::gpio::Gpio> + Sized>{
    fn config(pin:Pin){
        InputPin::config(pin); // and how to enable next?
    }
}
*/

/*

struct Input<Pin:InputPin<peripheral::gpio::Gpio>>{
    pin:Pin,
}

impl InputFun<Pin<peripheral::gpio::Gpio>> for Input<Pin<peripheral::gpio::Gpio>>{
}
*/

use stm32::gpio::Pin;
use memory_map::gpioe::GpioRB;
use stm32::rcc::RccRBTrait;
use memory_map::rcc::RccRB;


static mut GPIO_COUNTER:u8=0;

fn enable_gpio(){
    unsafe{
        if GPIO_COUNTER==0 {
            RccRB::get_rcc_mut().ahbenr.modify(|_, w| w.iopeen(true));
        }

        GPIO_COUNTER+=1;
    }
}

fn disable_gpio(){
    unsafe{
        GPIO_COUNTER-=1;

        if GPIO_COUNTER==0 {
            RccRB::get_rcc_mut().ahbenr.modify(|_, w| w.iopeen(false));
        }
    }
}

/*
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
*/

use ::stm32::gpio::functions::output::OutputPin;

create_pin!(E8,8);
impl OutputPin for E8{}

create_pin!(E9,9);

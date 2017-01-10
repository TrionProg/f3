use stm32::timer::{TimerTrait, Callback, DMARequest};
use stm32::timer::{BasicTimerTrait, BasicTimer, BasicTimerConfig, UIFRemapTrait};
use memory_map::timer6::TimerRB;

use stm32::rcc::RccRBTrait;
use memory_map::rcc::RccRB;

pub struct Timer6;

impl TimerTrait for Timer6{
    type TimerRB=TimerRB;

    /// Timer enable
    fn enable_timer(){
        RccRB::get_rcc_mut().apb1enr.modify(|_, w| w.tim6en(true));
    }

    fn disable_timer(){
        RccRB::get_rcc_mut().apb1enr.modify(|_, w| w.tim6en(false));
    }
}

impl BasicTimerTrait for Timer6{
    type BasicTimerRB=TimerRB;

    fn get_callback() -> &'static Callback{
        unsafe{ &EVENT_CALLBACK }
    }

    fn get_callback_mut() -> &'static mut Callback{
        unsafe{ &mut EVENT_CALLBACK }
    }
}

static mut EVENT_CALLBACK:Callback=None;

#[export_name = "_tim6"]  // <-- Important! Note the underscore.
pub extern "C" fn my_tim6_interrupt_handler() {
    unsafe{
        bkpt!();

        match EVENT_CALLBACK{
            Some( ref mut cb ) => (cb)(),
            None => {},
        }
    }

    Timer6::reset_uif();
}
/*

use Box;

pub struct Timer6{
    config:Timer6Config,
}

impl TimerTrait for Timer6{
    type TimerRB=TimerRB;
}

impl UIFRemapTrait for Timer6{
    type UIFRemapRB=TimerRB;
}

impl Timer6{
    pub const fn new(config:Timer6Config) -> Timer6{
        Timer6{config:config}
    }

    pub fn config(&self, config:Timer6Config){
        RccRB::get_rcc_mut().apb1enr.modify(|_, w| w.tim6en(true));

        self.set_prescaler(self.config.prescaler);
        self.set_uifremap(self.config.uif_remap);
    }

    fn enable(&self, period:u16, mode_once:bool, enable_interrupt:bool, enable_request:bool){
        RccRB::get_rcc_mut().apb1enr.modify(|_, w| w.tim6en(true));

        self.set_enabled(true);
    }

    fn disable(&self, period:u16, mode_once:bool, enable_interrupt:bool, enable_request:bool){
        RccRB::get_rcc_mut().apb1enr.modify(|_, w| w.tim6en(true));

        self.set_enabled(true);
    }

    pub fn delay(&self, interval:u16, callback:Option<Box<FnMut() -> ()>>, request:bool){
        let isCallback=callback.is_some();

        unsafe{
            EVENT_CALLBACK=callback;
        }

        //self.enable(interval, true, isCallback, request);

        TimerRB::set_opm( 1 );
        TimerRB::set_arr(interval as u32);
        //TimerRB::set_ug(1);
        //TimerRB::reset_uif();

        TimerRB::set_uie( 1 );
        TimerRB::set_ude( 0 );

        use stm32::timer::TimerRBTrait;
        TimerRB::set_cen(1);
    }

    pub fn wait(&self, interval:u16){
        //self.enable(interval, true, false, false);
        TimerRB::set_opm( 1 );
        TimerRB::set_arr(interval as u32);
        //TimerRB::set_ug(1);
        //TimerRB::reset_uif();

        TimerRB::set_uie( 0 );
        TimerRB::set_ude( 0 );

        use stm32::timer::TimerRBTrait;
        TimerRB::set_cen(1);

        while(TimerRB::get_uif()==0){}

        TimerRB::reset_uif();
    }
}

pub struct Timer6Config{
    pub prescaler:u16,
    pub uif_remap:bool,
}

impl Default for Timer6Config{
    fn default() -> Timer6Config{
        Timer6Config{
            prescaler:7999,
            uif_remap:false,
        }
    }
}

static mut EVENT_CALLBACK:Option<Box< FnMut() -> ()> >=None;

#[export_name = "_tim6"]  // <-- Important! Note the underscore.
pub extern "C" fn my_tim6_interrupt_handler() {
    unsafe{
        match EVENT_CALLBACK{
            Some( ref mut cb ) => (cb)(),
            None => {},
        }
    }
}
*/

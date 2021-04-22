#![no_main]
#![no_std]

#[allow(unused)]
use panic_rtt_target;
use rtic;
use rtt_target::{rprint, rprintln, rtt_init_print};

#[rtic::app(device = stm32f3xx_hal::pac, peripherals = true)]
const APP: () = {
    #[init(spawn=[foo])]
    fn init(mut cx: init::Context) {
        rtt_init_print!();
        rprintln!("init");
        cx.spawn.foo().unwrap();
        rprintln!("init end");
    }

    #[task]
    fn foo(_: foo::Context) {
        rprintln!("foo()");
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rprintln!("Spin");
        }
    }
    extern "C" {
        fn EXTI0();
    }
};
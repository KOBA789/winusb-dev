#![no_std]
#![no_main]
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use hal::usb::UsbBus;
use panic_probe as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::XOSC_CRYSTAL_FREQ;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::StringDescriptors;
use usb_device::device::UsbDeviceBuilder;
use usb_device::device::UsbDeviceState;
use usb_device::device::UsbVidPid;
use usb_device::LangID;

mod bulk_class;
mod winusb;

#[entry]
fn does_not_have_to_be_main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let bus = UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );
    let bus_allocator = UsbBusAllocator::new(bus);
    let mut bulk_out = bulk_class::BulkDuplexClass::new(&bus_allocator);
    let mut winusb = winusb::WinUsbClass;
    let mut dev = UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x6666, 0x4345))
        .strings(&[StringDescriptors::new(LangID::EN_US)
            .manufacturer("ArkEdge Space Inc.")
            .product("WinUSB DEV")])
        .unwrap()
        .build();

    let mut led_pin = pins.led.into_push_pull_output();
    led_pin.set_low().unwrap();
    loop {
        dev.poll(&mut [&mut winusb, &mut bulk_out]);
        if dev.state() == UsbDeviceState::Configured {
            led_pin.set_high().unwrap();
        } else {
            led_pin.set_low().unwrap();
        }
    }
}

use usb_device::{
    bus::{InterfaceNumber, UsbBus, UsbBusAllocator},
    class::UsbClass,
    endpoint::{EndpointIn, EndpointOut},
};

pub struct BulkDuplexClass<'a, B>
where
    B: UsbBus,
{
    ifn: InterfaceNumber,
    read_ep: EndpointOut<'a, B>,
    write_ep: EndpointIn<'a, B>,
}

impl<'a, B> BulkDuplexClass<'a, B>
where
    B: UsbBus,
{
    pub fn new(alloc: &'a UsbBusAllocator<B>) -> BulkDuplexClass<'a, B> {
        let ifn = alloc.interface();
        let read_ep = alloc.bulk(64);
        let write_ep = alloc.bulk(64);
        BulkDuplexClass { ifn, read_ep, write_ep }
    }

    pub fn read(&mut self, data: &mut [u8]) -> usb_device::Result<usize> {
        self.read_ep.read(data)
    }

    pub fn write(&mut self, data: &[u8]) -> usb_device::Result<usize> {
        self.write_ep.write(data)
    }
}

impl<'a, B> UsbClass<B> for BulkDuplexClass<'a, B>
where
    B: UsbBus,
{
    fn get_configuration_descriptors(
        &self,
        writer: &mut usb_device::descriptor::DescriptorWriter,
    ) -> usb_device::Result<()> {
        writer.interface(self.ifn, 0xFF, 0x00, 0x00)?;
        writer.endpoint(&self.read_ep)?;
        writer.endpoint(&self.write_ep)?;
        Ok(())
    }
}

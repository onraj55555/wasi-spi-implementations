use super::bindings;
use super::util;
use super::host_component;

pub struct SpiController {
    config: bindings::wasi::spi::general::SpiConfig,
    inner: rppal::spi::Spi,
}

impl bindings::wasi::spi::controller::Host for host_component::HostComponent {}

impl bindings::wasi::spi::controller::HostSpiController for host_component::HostComponent {
    fn select_chip(
        &mut self,
        self_: wasmtime::component::Resource<SpiController>,
    ) -> Result<(), bindings::wasi::spi::general::SpiError> {
        Err(bindings::wasi::spi::general::SpiError::OperationNotSupported)
    }

    fn deselect_chip(
        &mut self,
        self_: wasmtime::component::Resource<SpiController>,
    ) -> Result<(), bindings::wasi::spi::general::SpiError> {
        Err(bindings::wasi::spi::general::SpiError::OperationNotSupported)
    }

    fn transaction(
        &mut self,
        self_: wasmtime::component::Resource<SpiController>,
        operations: wasmtime::component::__internal::Vec<bindings::wasi::spi::general::Operation>,
    ) -> wasmtime::component::__internal::Vec<
        Result<
            Option<wasmtime::component::__internal::Vec<u8>>,
            bindings::wasi::spi::general::SpiError,
        >,
    > {
        let mut result = Vec::new();
        result.push(Err(
            bindings::wasi::spi::general::SpiError::OperationNotSupported,
        ));
        result
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<SpiController>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
    

    fn get(&mut self,config:bindings::wasi::spi::general::SpiConfig,) -> Result<wasmtime::component::Resource<SpiController>,bindings::wasi::spi::general::SpiError> {
        let spi = SpiController::new(config)?;

        self.table.push(spi).map_err(|e| bindings::wasi::spi::general::SpiError::Other(e.to_string()))
    }
    
    fn get_config(&mut self,self_:wasmtime::component::Resource<SpiController>,) -> Result<bindings::wasi::spi::general::SpiConfig,bindings::wasi::spi::general::SpiError> {
        let spi = self.table.get(&self_).map_err(|_| bindings::wasi::spi::general::SpiError::ResourceInvalidated)?;
        Ok(spi.get_config().clone())
    }
    
    fn write_word(&mut self,self_:wasmtime::component::Resource<SpiController>,write:wasmtime::component::__internal::Vec<u8>,) -> Result<(),bindings::wasi::spi::general::SpiError> {
        let spi = self.table.get_mut(&self_).map_err(|_| bindings::wasi::spi::general::SpiError::ResourceInvalidated)?;
        if ((write.len() * 8) as u64) < spi.get_config().word_size {
            return Err(bindings::wasi::spi::general::SpiError::WrongAmountOfData);
        }

        let len = (spi.get_config().word_size + 4) / 8; // 44 + 4 = 48, 48 / 8 = 6 -> 44 bits fit in 6 bytes

        let mut word = Vec::with_capacity(len as usize);
        
        for i in 0..len {
            word.push(write[i as usize]);
        }

        self.write_n_words(self_, word)
    }
    
    fn write_n_words(&mut self,self_:wasmtime::component::Resource<SpiController>,write:wasmtime::component::__internal::Vec<u8>,) -> Result<(),bindings::wasi::spi::general::SpiError> {
        let spi = self.table.get_mut(&self_).map_err(|_| bindings::wasi::spi::general::SpiError::ResourceInvalidated)?;
        if ((write.len() * 8) as u64) < spi.get_config().word_size {
            return Err(bindings::wasi::spi::general::SpiError::WrongAmountOfData);
        }

        spi.write(&write)
    }

    fn read_word(&mut self,self_:wasmtime::component::Resource<SpiController>,) -> Result<wasmtime::component::__internal::Vec<u8>,bindings::wasi::spi::general::SpiError> {       
        self.read_n_words(self_, 1)
    }
    
    fn read_n_words(&mut self,self_:wasmtime::component::Resource<SpiController>,n:u64,) -> Result<wasmtime::component::__internal::Vec<u8>,bindings::wasi::spi::general::SpiError> {
        let spi = self.table.get_mut(&self_).map_err(|_| bindings::wasi::spi::general::SpiError::ResourceInvalidated)?;

        let bit_count = n * spi.get_config().word_size;

        let len = (bit_count + 4) / 8;

        let mut word = vec![0; len as usize];
        
        spi.read(&mut word)?;
        Ok(word)
    }
    
    fn transfer_word(&mut self,self_:wasmtime::component::Resource<SpiController>,write:wasmtime::component::__internal::Vec<u8>,) -> Result<wasmtime::component::__internal::Vec<u8>,bindings::wasi::spi::general::SpiError> {
        let spi = self.table.get_mut(&self_).map_err(|_| bindings::wasi::spi::general::SpiError::ResourceInvalidated)?;

        let len = (spi.get_config().word_size + 4) / 8;

        let mut write_word = vec![0u8; len as usize];

        for i in 0..len {
            write_word[i as usize] = write[i as usize];
        }
        
        self.transfer_n_words(self_, write_word)
    }
    
    #[doc = " Write len(writer) and read n bytes to and from the peripheral, manually turning on and off the CS pin unless it is manually turned on"]
    fn transfer_n_words(&mut self,self_:wasmtime::component::Resource<SpiController>,write:wasmtime::component::__internal::Vec<u8>) -> Result<wasmtime::component::__internal::Vec<u8>,bindings::wasi::spi::general::SpiError> {
        let spi = self.table.get_mut(&self_).map_err(|_| bindings::wasi::spi::general::SpiError::ResourceInvalidated)?;
        
        if ((write.len() * 8) as u64) < spi.get_config().word_size {
            return Err(bindings::wasi::spi::general::SpiError::WrongAmountOfData);
        }

        let mut read = vec![0u8; write.len()];

        spi.transfer(&write, &mut read)?;
        Ok(read)
    }
}

impl SpiController {
    pub fn new(
        config: bindings::wasi::spi::general::SpiConfig,
    ) -> Result<Self, bindings::wasi::spi::general::SpiError> {
        

        let bus = util::get_bus(&config.bus)?;
        let cs_pin = match &config.cs_pin {
            Some(cs_pin) => util::get_cs_pin(cs_pin)?,
            None => todo!(),
        };

        let inner =
            match rppal::spi::Spi::new(bus, cs_pin, config.frequency as u32, config.mode.into()) {
                Ok(inner) => inner,
                Err(e) => return Err(bindings::wasi::spi::general::SpiError::Other(e.to_string())),
            };

        Ok(Self {
            config,
            inner,
        })
    }
    
    pub fn get_config(&self) -> &bindings::wasi::spi::general::SpiConfig {
        &self.config
    }
    
    pub fn write(&mut self, buffer: &[u8]) -> Result<(),bindings::wasi::spi::general::SpiError> {
        self.inner.write(&buffer).map(|_| ()).map_err(|e| bindings::wasi::spi::general::SpiError::Other(e.to_string()))
    }
    
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(),bindings::wasi::spi::general::SpiError> {
        self.inner.read(buffer).map_err(|e| bindings::wasi::spi::general::SpiError::Other(e.to_string()))?;
        Ok(())
    }
    
    pub fn transfer(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), bindings::wasi::spi::general::SpiError> {
        self.inner.transfer(read, write).map_err(|e| bindings::wasi::spi::general::SpiError::Other(e.to_string()))?;
        Ok(())
    }

    pub fn transaction(&mut self, operations: Vec<bindings::wasi::spi::general::Operation>) -> Vec<Result<Option<Vec<u8>>, bindings::wasi::spi::general::SpiError>>{
        /*let mut segments = Vec::new();
        let mut result = Vec::new();

        for operation in operations {
            match operation {
                bindings::wasi::spi::general::Operation::Read(n) => {
                    let bits = n * self.config.word_size;
                    let len = (bits + 4) % 8;
                    let mut buffer = vec![0u8; len as usize];
                    segments.push(rppal::spi::Segment::with_read(&mut buffer));
                    result.push(Ok(Some(buffer)));
                },
                bindings::wasi::spi::general::Operation::Write(items) => {
                    segments.push(rppal::spi::Segment::with_write(&items));
                    result.push(Ok(None));
                },
                bindings::wasi::spi::general::Operation::Transfer(items) => {
                    let mut read = vec![0u8; items.len()];
                    segments.push(rppal::spi::Segment::new(&mut read, &items));
                    result.push(Ok(Some(read)));
                },
                bindings::wasi::spi::general::Operation::Delay(d) => {
                    let mut s = rppal::spi::Segment::new(&mut [], & []);
                    s.set_delay(d as u16);
                    segments.push(s);
                    result.push(Ok(None));
                },
            };
        }

        match self.inner.transfer_segments(&segments) {
            Ok(_) => result,
            Err(e) => vec![Err(bindings::wasi::spi::general::SpiError::Other(e.to_string()))],
        }*/

        todo!()
    }
}
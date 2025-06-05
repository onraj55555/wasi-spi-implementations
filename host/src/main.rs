use wasmtime::{component::{self, Linker}, Config, Engine, RootScope};
use clap::Parser;
use wasmtime_wasi::ResourceTable;

mod controller;
mod util;
mod host_component;

pub use controller::SpiController;

#[derive(Parser)]
struct CliConfig {
    #[arg(short, long)]
    component: String
}

mod bindings {
    wasmtime::component::bindgen!({
        path: "../../wit",
        with: {
            "wasi:spi/controller/spi-controller": crate::SpiController,
        }
    });
}


pub struct State {
    host: host_component::HostComponent,
    ctx: wasmtime_wasi::WasiCtx
}

impl bindings::wasi::spi::general::Host for host_component::HostComponent {}

impl wasmtime_wasi::WasiView for State {
    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.ctx
    }
}

impl wasmtime_wasi::IoView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.host.table
    }
}

fn main() {
    let config = CliConfig::parse();

    let engine = Engine::new(Config::new().wasm_component_model(true)).unwrap();
    let mut linker = Linker::new(&engine);

    bindings::wasi::spi::general::add_to_linker(&mut linker, |state: &mut State| &mut state.host).unwrap();
    bindings::wasi::spi::controller::add_to_linker(&mut linker, |state: &mut State| {&mut state.host}).unwrap();

    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();

    let component = wasmtime::component::Component::from_file(&engine, config.component).unwrap();

    let mut host = host_component::HostComponent { table: wasmtime::component::ResourceTable::new() };

    let state = State {
        host,
        ctx: wasmtime_wasi::WasiCtxBuilder::new().inherit_stdio().build()
    };

    let mut store = wasmtime::Store::new(&engine, state);

    let instance = bindings::RpiController::instantiate(&mut store, &component, &linker).unwrap();

    instance.call_start(&mut store).unwrap();
}

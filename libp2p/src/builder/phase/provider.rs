use super::*;
use crate::SwarmBuilder;
use std::marker::PhantomData;

pub struct ProviderPhase {}

impl SwarmBuilder<NoProviderSpecified, ProviderPhase> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "async-std"))]
    pub fn with_async_std(self) -> SwarmBuilder<AsyncStd, TcpPhase> {
        SwarmBuilder {
            keypair: self.keypair,
            phantom: PhantomData,
            phase: TcpPhase {},
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "tokio"))]
    pub fn with_tokio(self) -> SwarmBuilder<Tokio, TcpPhase> {
        SwarmBuilder {
            keypair: self.keypair,
            phantom: PhantomData,
            phase: TcpPhase {},
        }
    }

    #[cfg(feature = "wasm-bindgen")]
    pub fn with_wasm_bindgen(self) -> SwarmBuilder<WasmBindgen, TcpPhase> {
        SwarmBuilder {
            keypair: self.keypair,
            phantom: PhantomData,
            phase: TcpPhase {},
        }
    }
}

pub enum NoProviderSpecified {}

#[cfg(feature = "async-std")]
pub enum AsyncStd {}

#[cfg(feature = "tokio")]
pub enum Tokio {}

#[cfg(feature = "wasm-bindgen")]
pub enum WasmBindgen {}

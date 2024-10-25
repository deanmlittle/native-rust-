#![cfg_attr(target_os = "solana", feature(asm_experimental_arch))]

#[cfg(test)]
mod tests;

#[cfg(all(target_os = "solana", feature = "based"))]
mod based;
#[cfg(all(target_os = "solana", feature = "based"))]
use based::*;

#[cfg(feature = "optimized")]
mod optimized;
#[cfg(feature = "optimized")]
use optimized::*;

#[cfg(all(target_os = "solana", feature = "native"))]
mod native;
#[cfg(all(target_os = "solana", feature = "native"))]
use native::*;
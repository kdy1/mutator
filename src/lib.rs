
//! You can use it to allow extending method chain.
//!
//!
//!
//!
//! # Examples (Extendable builder with future-proof)
//!
//! ```rust
//! // mock
//! mod othercrate {
//! 	use mutator::Mutator;
//! 	pub struct CrateBuilder {
//! 		pool_size: usize,
//! 	}
//!
//! 	impl CrateBuilder {
//! 		// not important in this example
//! 		pub fn build(self) -> () {
//! 		}
//! 	}
//!
//! 	pub trait CrateBuilderMut: Mutator<CrateBuilder> {
//! 		// returns self, so it can be builder type of wrapper library
//! 		//
//! 		fn with_pool_size(self, pool_size: usize) -> Self {
//! 			self.mutate(|b| b.pool_size = pool_size)
//! 		}
//! 	}
//!
//!
//! 	/// Mutator<CrateBuilder> is implemented for CrateBuilder, as logically it is.
//! 	///
//!  	/// Any type which can mutate CrateBuilder can be used as CrateBuilder.
//! 	///
//! 	impl<B> CrateBuilderMut for B where B: Mutator<CrateBuilder> {}
//!
//! 	pub fn builder() -> CrateBuilder {
//! 		CrateBuilder{ pool_size: 0 }
//! 	}
//! }
//!
//!
//! // mock wrapper crate
//! mod runner {
//! 	use mutator::Mutator;
//! 	use othercrate::{self, CrateBuilder};
//! 	pub use othercrate::CrateBuilderMut;
//!
//! 	pub struct RunnerBuilder {
//! 		name: Option<&'static str>,
//! 		crate_builder: CrateBuilder,
//! 	}
//!
//! 	impl RunnerBuilder {
//! 		/// not important in this example
//! 		pub fn build(self) -> () {
//! 		}
//! 	}
//!
//!
//! 	pub trait RunnerBuilderMut: Mutator<RunnerBuilder> {
//! 		fn with_name(self, name: &'static str) -> Self {
//! 			self.mutate(|b| b.name = Some(name))
//! 		}
//! 	}
//!
//! 	impl<B> RunnerBuilderMut for B where B: Mutator<RunnerBuilder> {}
//!
//! 	/// By this, methods on CrateBuilder can be called on RunnerBuilder
//! 	impl Mutator<CrateBuilder> for RunnerBuilder {
//! 		fn mutate<F>(mut self, op: F) -> Self where F: FnOnce(&mut CrateBuilder) {
//! 			op(&mut self.crate_builder);
//! 			self
//! 		}
//! 	}
//!
//!
//! 	pub fn new() -> RunnerBuilder {
//! 		RunnerBuilder {
//! 			name: None,
//! 			crate_builder: othercrate::builder(),
//! 		}
//! 	}
//! }
//!
//!
//!
//! // end-user
//!
//! use runner::{RunnerBuilderMut, CrateBuilderMut};
//!
//! fn main() {
//! 	let _ = runner::new().with_pool_size(10).with_name("whoami").build();
//!
//! }
//! ```


/// **Mutator<T>** is a type which can mutate T.
///
/// Any type implements Mutator<Self> as logically it is.
pub trait Mutator<T>: Sized {
    fn mutate<F>(self, op: F) -> Self where F: FnOnce(&mut T);
}

impl<T> Mutator<T> for T {
    fn mutate<F>(mut self, op: F) -> Self
        where F: FnOnce(&mut T)
    {
        op(&mut self);
        self
    }
}
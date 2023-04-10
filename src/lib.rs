// Import necessary modules
pub mod encryption;
pub mod utils;

// Declare the crate as a workspace
// This is required to enable the use of features across multiple crates in the workspace
// For example, if the caesar feature is enabled in SadieFish, it will also be enabled in any other
// crate in the workspace that depends on SadieFish.
// This is useful for testing and for avoiding version mismatches between crates.
#[cfg(not(test))] // Only include this section in non-test builds
#[cfg(feature = "workspace")]
pub mod workspace {
    pub use SadieFish_encryption as encryption;
    pub use SadieFish_utils as utils;
}

// Declare the encryption module as the default feature
#[cfg(not(test))] // Only include this section in non-test builds
#[cfg(feature = "default")]
pub use SadieFish_encryption::*;

// Declare features for the various encryption algorithms and utilities
#[cfg(not(test))] // Only include this section in non-test builds
#[cfg(any(feature = "caesar", feature = "vigenere", feature = "aes", feature = "des", feature = "rsa"))]
#[cfg(not(feature = "default"))]
pub use SadieFish_encryption::*;

#[cfg(not(test))] // Only include this section in non-test builds
#[cfg(any(feature = "base64", feature = "hex", feature = "bit_ops"))]
pub use SadieFish_utils::*;

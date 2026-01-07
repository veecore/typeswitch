//! # TypeSwitch
//!
//! `typeswitch` provides a powerful, Go-inspired macro to perform runtime type switching 
//! on `dyn Any` trait objects in Rust.
//!
//! While Rust's `match` statement is powerful for enums, it cannot natively branch based 
//! on the concrete type of a trait object. This crate bridges that gap with a clean, 
//! block-based syntax that supports:
//!
//! - **Immutable and Mutable access** to the underlying data.
//! - **Owned Consumption**: Move values out of a `Box<dyn Any>`.
//! - **Go-style Binding**: Automatically bind the downcasted value to a variable for all branches.
//! - **Or-Patterns**: Match against multiple types in a single branch.


/// A powerful macro to emulate a type switch statement for `dyn Any` trait objects.
///
/// This macro allows you to match on the concrete type of a `Box<dyn Any>` or `&dyn Any`,
/// similar to a `match` statement but for runtime types. It supports both immutable and
/// mutable downcasting, attributes, and default fallbacks.
///
/// # Syntax
///
/// ```text
/// typeswitch! {
///     [modifiers] subject {
///         [attributes] binding: Type => { block }
///         TypeA | TypeB => { block }
///         ...
///         _ => { fallback block }
///     }
/// }
/// ```
///
/// - **subject**: The variable to switch on. It must be an identifier.
///   - Use `x` for immutable access (bindings will be `&T`).
///   - Use `mut x` for mutable access (bindings will be `&mut T`).
/// - **binding**: The name to bind the downcasted value to.
/// - **Type**: The concrete type to check for.
///
/// # Examples
///
/// ## 1. Basic Immutable Switch
///
/// ```rust
/// # use typeswitch::typeswitch;
/// # use std::any::Any;
/// let b: Box<dyn Any> = Box::new(42i32);
///
/// let res = typeswitch! { b {
///         num: i32 => { format!("Integer: {}", num) }
///         s: String => { format!("String: {}", s) }
///         _ => { "Unknown".to_string() }
///     }
/// };
/// ```
///
/// ## 2. Bound Mutable Switch
/// Prefix the binding with `mut` or use the `mut v as x` syntax to gain mutable access.
///
/// ```rust
/// # use typeswitch::typeswitch;
/// # use std::any::Any;
/// let mut x: Box<dyn Any> = Box::new(10i32);
///
/// typeswitch!(mut v as x {
///     i32 => { *v += 5; }
///     _ => {},
/// });
/// ```
///
/// ## 3. Owned Consumption (Moving out of Box)
/// Use the `box` keyword to take ownership of the value. This branch will only 
/// execute if the type matches, and it will consume the `Box`.
///
/// ```rust
/// # use typeswitch::typeswitch;
/// # use std::any::Any;
/// let x: Box<dyn Any> = Box::new(String::from("Move me"));
///
/// typeswitch! { x {
///     box s: String => { println!("Consumed: {}", s) } // s is String (owned)
///     _ => {}
/// }}
/// ```
///
/// ## 4. Piped Switch
///
/// ```rust
/// # use typeswitch::typeswitch;
/// # use std::any::Any;
/// let b: Box<dyn Any> = Box::new(10i32);
///
/// typeswitch! { b {
///         String | &str => {println!("string")}
///         i32 => {println!("Number!")}
///     }
/// }
/// ```
///
/// ## 5. Automatic Binding (Go-style)
/// By providing a variable name before the block, that name is automatically 
/// bound to the downcasted type in every branch.
/// ```rust
/// # use typeswitch::typeswitch;
/// # use std::any::Any;
/// let x: &dyn Any = &100i32;
///
/// typeswitch!(val as x {
///     i32 => {println!("Double is: {}", val * 2)}
///     f64 => {println!("Half is: {}", val / 2.0)}
///     _ => {}
/// });
/// ```
#[macro_export]
macro_rules! typeswitch {
    // ============================================================
    // ENTRY POINTS
    // ============================================================

    // 1. Pre-binding syntax: typeswitch!(v as x; ...)
    // This shadows 'v' inside the branches automatically.
    ($bind:ident as $var:ident { $($rest:tt)* } ) => {{
        $crate::typeswitch!(@step $var; $bind $($rest)*)
    }};

    // 2. Modified pre-binding syntax: typeswitch!(mut v as x; ...)
    ($modifier:ident $bind:ident as $var:ident { $($rest:tt)* } ) => {{
        $crate::typeswitch!(@step $var; $bind $modifier $($rest)*)
    }};

    // 3. Standard syntax: typeswitch!(x; ...)
    // No automatic binding is applied unless explicitly stated in cases.
    ($var:ident { $($rest:tt)* } ) => {{
        $crate::typeswitch!(@step $var; $($rest)*)
    }};

    // ============================================================
    // NORMALIZATION (Redistributors)
    // ============================================================

    // 1. Handle `_` explicitly before capturing it as $ty.
    // This prevents `_` from turning into an opaque Type AST node.
    (@step $var:expr; $auto:ident _ => $block:block $($rest:tt)*) => {
        $crate::typeswitch!{@step $var; $auto : _ => $block $auto $($rest)*}
    };

    // 2. Handle `modifier _` explicitly as well
    (@step $var:expr; $auto:ident $modifier:ident _ => $block:block $($rest:tt)*) => {
        $crate::typeswitch!{@step $var; $modifier $auto : _ => $block $auto $($rest)*}
    };

    // FIXME: We think $modifier is what we added but it could be from this specific
    // 3. Generic redistributor for modifiers (mut, box)
    (@step $var:expr; $auto:ident $modifier:ident $ty:ty => $block:block $($rest:tt)*) => {
        $crate::typeswitch!{@step $var; $modifier $auto : $ty => $block $auto $modifier $($rest)*}
    };

    // 4. Generic redistributor for standard types
    (@step $var:expr; $auto:ident $ty:ty => $block:block $($rest:tt)*) => {
        $crate::typeswitch!{@step $var; $auto : $ty => $block $auto $($rest)*}
    };

    // ============================================================
    // ARM COLLECTION (The "Muncher")
    // ============================================================

    // ----------------------------------------------------------------
    // PATTERN: _ => { ... } (Default case)
    // ----------------------------------------------------------------

    // 1. Default case with modifier (e.g. mut v: _)
    (@step $var:expr; $modifier:ident $auto:ident : _ => $block:block $($rest:tt)*) => {
        $block
    };

    // 2. Default case standard (e.g. v: _)
    (@step $var:expr; $auto:ident : _ => $block:block $($rest:tt)*) => {
        $block
    };

    // 3. Default case no-binding (e.g. _ =>)
    (@step $var:expr; _ => $block:block $($rest:tt)*) => {
        $block
    };

    // 4.
    // ----------------------------------------------------------------
    // PATTERN: box binding: Type => { ... }
    // Requirement: $var must be Box<dyn Any>
    // ----------------------------------------------------------------
    (@step $var:expr; box $bind:ident : $ty:ty => $block:block $($rest:tt)*) => {
        // We check 'is' first to avoid consuming the box if the type doesn't match.
        // If it does match, we unwrap.
        if $var.is::<$ty>() {
            // We must cast to the concrete type.
            // Note: downcast returns Result<Box<T>, Box<dyn Any>>
            let $bind = *$var.downcast::<$ty>().expect("typeswitch: type check passed but downcast failed");
            $block
        } else {
            $crate::typeswitch!{@step $var; $($rest)*}
        }
    };

    // 5.
    // ----------------------------------------------------------------
    // PATTERN: mut binding @ Type => { ... }
    // Requirement: $var must be &mut dyn Any (or Box)
    // ----------------------------------------------------------------
    (@step $var:expr; mut $bind:ident : $ty:ty => $block:block $($rest:tt)*) => {
        if let Some($bind) = <dyn std::any::Any>::downcast_mut::<$ty>(&mut *$var) {
            $block
        } else {
            $crate::typeswitch!{@step $var; $($rest)*}
        }
    };

    // 6.
    // ----------------------------------------------------------------
    // PATTERN: binding: Type => { ... }
    // Requirement: $var must be &dyn Any (or &mut/Box)
    // ----------------------------------------------------------------
    (@step $var:expr; $bind:ident : $ty:ty => $block:block $($rest:tt)*) => {
        if let Some($bind) = <dyn std::any::Any>::downcast_ref::<$ty>(&*$var) {
            $block
        } else {
            $crate::typeswitch!{@step $var; $($rest)*}
        }
    };

    // 7.
    // ----------------------------------------------------------------
    // PATTERN: Type => { ... } (No binding, just check)
    // ----------------------------------------------------------------
    (@step $var:expr; $ty:ty => $block:block $($rest:tt)*) => {
        if <dyn std::any::Any>::is::<$ty>(&*$var as _) {
            $block
        } else {
            $crate::typeswitch!{@step $var; $($rest)*}
        }
    };

    // 8.
    // ----------------------------------------------------------------
    // PATTERN: Type | Type => { ... } (Or pattern)
    // ----------------------------------------------------------------
    (@step $var:expr; $head:ty | $($tail:ty)|+ => $block:block $($rest:tt)*) => {
        if <dyn std::any::Any>::is::<$head>(&$var as _) $(|| <dyn std::any::Any>::is::<$tail>(&* $var))+ {
            $block
        } else {
            $crate::typeswitch!{@step $var; $($rest)*}
        }
    };

    // TODO: Support attributes on arms..

    // ----------------------------------------------------------------
    // Base Case: No more patterns
    // ----------------------------------------------------------------
    (@step $var:expr;) => {};

    // ----------------------------------------------------------------
    // Base Case: No more patterns auto
    // ----------------------------------------------------------------
    (@step $var:expr; $auto:ident) => {};
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;

    #[test]
    fn test_standard_immutable() {
        let x: &dyn Any = &42i32;
        let result = typeswitch! { x {
                val: i32 => { format!("int {}", val) }
                val: String => { format!("string {}", val) }
                _ => { "unknown".to_string() }
            }
        };
        assert_eq!(result, "int 42");
    }

    #[test]
    fn test_standard_mutable() {
        let mut val = 10i32;
        let x: &mut dyn Any = &mut val;

        typeswitch! { x {
                // #[cfg(feature = "d")]
                mut v: i32 => { *v += 10; }
                _ => {}
            }
        }

        assert_eq!(val, 20);
    }

    #[test]
    fn test_owned_box() {
        let x: Box<dyn Any> = Box::new(String::from("Hello"));

        let res = typeswitch! { x {
                // Checks types but consumes box only if match
                box s: String => { s }
                _ => { String::new() }
            }
        };

        assert_eq!(res, "Hello");
    }

    #[test]
    fn test_pre_binding_syntax() {
        let x: &dyn Any = &100i32;

        let res = typeswitch! {
            // Automatically binds 'v' to the downcasted type
            v as x {
                String => { v.len() }
                i32 => { *v as usize }
                _ => { 0 }
            }
        };

        assert_eq!(res, 100);
    }

    #[test]
    fn test_pre_binding_mutable_() {
        let mut x: Box<dyn Any> = Box::new(String::from("Hello"));

        let result = typeswitch! { mut v as x {
                i32 => { format!("int {}", v) }
                String => { format!("string {}", v) }
                _ => { "unknown".to_string() }
            }
        };
        assert_eq!(result, "string Hello");
    }    

    #[test]
    fn test_expression_return() {
        let x: &dyn Any = &1.5f64;
        let val = typeswitch! { x {
                f64 => { 1 }
                _ => { 0 }
            }
        };
        assert_eq!(val, 1);
    }

    #[test]
    fn test_or_pattern() {
        let x: Box<dyn Any> = Box::new(10i64);
        
        let res = typeswitch! { x {
                f32 | f64 => { "float" }
                i32 | i64 => { "int" }
                _ => { "other" }
            }
        };
        
        assert_eq!(res, "int");
    }

    // TODO:
    #[test]
    fn test_type_param() {
        fn _func<T: 'static>(t: &T) {
            typeswitch! { t {
                t: String => {println!("Amen: {t}");}
            }}
        }
    }
}

//! String trait implementations.

use crate::{
    FromValue, OwnedMut, OwnedRef, RawOwnedMut, RawOwnedRef, ReflectValueType, Shared, ToValue,
    UnsafeFromValue, Value, ValueError, ValueType, ValueTypeInfo,
};

impl ReflectValueType for String {
    type Owned = String;

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::String
    }
}

impl<'a> ReflectValueType for &'a str {
    type Owned = String;

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::String
    }
}

impl<'a> ReflectValueType for &'a mut str {
    type Owned = String;

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::String
    }
}

impl ToValue for String {
    fn to_value(self) -> Result<Value, ValueError> {
        Ok(Value::String(Shared::new(self)))
    }
}

impl FromValue for String {
    fn from_value(value: Value) -> Result<Self, ValueError> {
        match value {
            Value::String(string) => Ok(string.borrow_ref()?.clone()),
            Value::StaticString(string) => Ok(string.as_ref().clone()),
            actual => Err(ValueError::ExpectedString {
                actual: actual.type_info()?,
            }),
        }
    }
}

/// Convert a string into a value type.
impl ReflectValueType for Box<str> {
    type Owned = String;

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::String
    }
}

impl ToValue for Box<str> {
    fn to_value(self) -> Result<Value, ValueError> {
        Ok(Value::String(Shared::new(self.to_string())))
    }
}

impl FromValue for Box<str> {
    fn from_value(value: Value) -> Result<Self, ValueError> {
        let string = value.into_string()?;
        let string = string.borrow_ref()?.clone();
        Ok(string.into_boxed_str())
    }
}

impl UnsafeFromValue for &'_ str {
    type Output = *const str;
    type Guard = Option<RawOwnedRef>;

    unsafe fn unsafe_from_value(value: Value) -> Result<(Self::Output, Self::Guard), ValueError> {
        Ok(match value {
            Value::String(string) => {
                let string = string.owned_ref()?;
                let (s, guard) = OwnedRef::into_raw(string);
                ((*s).as_str(), Some(guard))
            }
            Value::StaticString(string) => (string.as_ref().as_str(), None),
            actual => {
                return Err(ValueError::ExpectedString {
                    actual: actual.type_info()?,
                })
            }
        })
    }

    unsafe fn to_arg(output: Self::Output) -> Self {
        &*output
    }
}

impl UnsafeFromValue for &'_ String {
    type Output = *const String;
    type Guard = Option<RawOwnedRef>;

    unsafe fn unsafe_from_value(value: Value) -> Result<(Self::Output, Self::Guard), ValueError> {
        Ok(match value {
            Value::String(string) => {
                let string = string.owned_ref()?;
                let (s, guard) = OwnedRef::into_raw(string);
                (s, Some(guard))
            }
            Value::StaticString(string) => (string.as_ref(), None),
            actual => {
                return Err(ValueError::ExpectedString {
                    actual: actual.type_info()?,
                })
            }
        })
    }

    unsafe fn to_arg(output: Self::Output) -> Self {
        &*output
    }
}

impl ReflectValueType for &'_ String {
    type Owned = String;

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::String
    }
}

impl UnsafeFromValue for &'_ mut String {
    type Output = *mut String;
    type Guard = RawOwnedMut;

    unsafe fn unsafe_from_value(value: Value) -> Result<(Self::Output, Self::Guard), ValueError> {
        Ok(match value {
            Value::String(string) => {
                let string = string.owned_mut()?;
                let (s, guard) = OwnedMut::into_raw(string);
                (s, guard)
            }
            actual => {
                return Err(ValueError::ExpectedString {
                    actual: actual.type_info()?,
                })
            }
        })
    }

    unsafe fn to_arg(output: Self::Output) -> Self {
        &mut *output
    }
}

impl ReflectValueType for &'_ mut String {
    type Owned = String;

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::String
    }
}

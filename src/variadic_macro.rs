//! This module contains a macro that is used to pre-process safe arguments to an unsafe FFI function accepting variatic arguments
//!
//! Do not use directly

/// This macro is not meant to be used directly
///
/// It is used to pre-process safe arguments to an unsafe FFI function accepting variatic arguments
///
/// This makes sure that arguments must be safe in order to call the function
#[doc(hidden)]
#[macro_export]
macro_rules! variadic_unsafe_nightmare_spaghetti {
    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        identified_args = [$($ident:ident = $arg:expr),*]
    ) => { {
        $(
            let $ident = $arg;
        )*

        unsafe {
            $fn_name(
                $($normal_arg,)*
                $($ident,)*
            )
        }
    } };

    //
    // Begin the variadic argument handling
    // This will only go up to 32
    //

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = []
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti!(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = []
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [$arg1:expr]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti!(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [$arg1:expr, $arg2:expr]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti!(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti!(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti!(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti!(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti!(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr,
            $arg8:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7,
                arg8 = $arg8
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr,
            $arg8:expr,
            $arg9:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7,
                arg8 = $arg8,
                arg9 = $arg9
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr,
            $arg8:expr,
            $arg9:expr,
            $arg10:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7,
                arg8 = $arg8,
                arg9 = $arg9,
                arg10 = $arg10
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr,
            $arg8:expr,
            $arg9:expr,
            $arg10:expr,
            $arg11:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7,
                arg8 = $arg8,
                arg9 = $arg9,
                arg10 = $arg10,
                arg11 = $arg11
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr,
            $arg8:expr,
            $arg9:expr,
            $arg10:expr,
            $arg11:expr,
            $arg12:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7,
                arg8 = $arg8,
                arg9 = $arg9,
                arg10 = $arg10,
                arg11 = $arg11,
                arg12 = $arg12
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr,
            $arg8:expr,
            $arg9:expr,
            $arg10:expr,
            $arg11:expr,
            $arg12:expr,
            $arg13:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7,
                arg8 = $arg8,
                arg9 = $arg9,
                arg10 = $arg10,
                arg11 = $arg11,
                arg12 = $arg12,
                arg13 = $arg13
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr,
            $arg8:expr,
            $arg9:expr,
            $arg10:expr,
            $arg11:expr,
            $arg12:expr,
            $arg13:expr,
            $arg14:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7,
                arg8 = $arg8,
                arg9 = $arg9,
                arg10 = $arg10,
                arg11 = $arg11,
                arg12 = $arg12,
                arg13 = $arg13,
                arg14 = $arg14
            ]
        )
    };

    (
        $fn_name:path,
        args = [$($normal_arg:expr),*],
        va_args = [
            $arg1:expr,
            $arg2:expr,
            $arg3:expr,
            $arg4:expr,
            $arg5:expr,
            $arg6:expr,
            $arg7:expr,
            $arg8:expr,
            $arg9:expr,
            $arg10:expr,
            $arg11:expr,
            $arg12:expr,
            $arg13:expr,
            $arg14:expr,
            $arg15:expr
        ]
    ) => {
        $crate::variadic_unsafe_nightmare_spaghetti(
            $fn_name,
            args = [$($normal_arg),*],
            identified_args = [
                arg1 = $arg1,
                arg2 = $arg2,
                arg3 = $arg3,
                arg4 = $arg4,
                arg5 = $arg5,
                arg6 = $arg6,
                arg7 = $arg7,
                arg8 = $arg8,
                arg9 = $arg9,
                arg10 = $arg10,
                arg11 = $arg11,
                arg12 = $arg12,
                arg13 = $arg13,
                arg14 = $arg14,
                arg15 = $arg15
            ]
        )
    };
}

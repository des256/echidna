#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![warn(single_use_lifetimes, unreachable_pub)]
#![warn(clippy::default_trait_access)]

#[macro_export]
macro_rules! pin_project {
    ($($tt:tt)*) => {
        $crate::__pin_project_internal! {
            [][][][]
            $($tt)*
        }
    };
}

#[macro_export]
macro_rules! __pin_project_internal {
    (@struct=>internal;
        [$($proj_mut_ident:ident)?]
        [$($proj_ref_ident:ident)?]
        [$($proj_replace_ident:ident)?]
        [$proj_vis:vis]
        [$(#[$attrs:meta])* $vis:vis struct $ident:ident]
        [$($def_generics:tt)*]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)*)?]
        {
            $(
                $(#[$pin:ident])?
                $field_vis:vis $field:ident: $field_ty:ty
            ),+
        }
    ) => {
        $(#[$attrs])*
        $vis struct $ident $($def_generics)*
        $(where
            $($where_clause)*)?
        {
            $(
                $field_vis $field: $field_ty
            ),+
        }

        $crate::__pin_project_internal! { @struct=>make_proj_ty=>named;
            [$proj_vis]
            [$($proj_mut_ident)?]
            [make_proj_field_mut]
            [$ident]
            [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            {
                $(
                    $(#[$pin])?
                    $field_vis $field: $field_ty
                ),+
            }
        }
        $crate::__pin_project_internal! { @struct=>make_proj_ty=>named;
            [$proj_vis]
            [$($proj_ref_ident)?]
            [make_proj_field_ref]
            [$ident]
            [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            {
                $(
                    $(#[$pin])?
                    $field_vis $field: $field_ty
                ),+
            }
        }
        $crate::__pin_project_internal! { @struct=>make_proj_replace_ty=>named;
            [$proj_vis]
            [$($proj_replace_ident)?]
            [make_proj_field_replace]
            [$ident]
            [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            {
                $(
                    $(#[$pin])?
                    $field_vis $field: $field_ty
                ),+
            }
        }

        #[allow(explicit_outlives_requirements)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            $crate::__pin_project_internal! { @struct=>make_proj_ty=>unnamed;
                [$proj_vis]
                [$($proj_mut_ident)?][Projection]
                [make_proj_field_mut]
                [$ident]
                [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
                {
                    $(
                        $(#[$pin])?
                        $field_vis $field: $field_ty
                    ),+
                }
            }
            $crate::__pin_project_internal! { @struct=>make_proj_ty=>unnamed;
                [$proj_vis]
                [$($proj_ref_ident)?][ProjectionRef]
                [make_proj_field_ref]
                [$ident]
                [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
                {
                    $(
                        $(#[$pin])?
                        $field_vis $field: $field_ty
                    ),+
                }
            }
            $crate::__pin_project_internal! { @struct=>make_proj_replace_ty=>unnamed;
                [$proj_vis]
                [$($proj_replace_ident)?][ProjectionReplace]
                [make_proj_field_replace]
                [$ident]
                [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
                {
                    $(
                        $(#[$pin])?
                        $field_vis $field: $field_ty
                    ),+
                }
            }

            impl <$($impl_generics)*> $ident <$($ty_generics)*>
            $(where
                $($where_clause)*)?
            {
                $crate::__pin_project_internal! { @struct=>make_proj_method;
                    [$proj_vis]
                    [$($proj_mut_ident)?][Projection]
                    [project get_unchecked_mut mut]
                    [$($ty_generics)*]
                    {
                        $(
                            $(#[$pin])?
                            $field_vis $field
                        ),+
                    }
                }
                $crate::__pin_project_internal! { @struct=>make_proj_method;
                    [$proj_vis]
                    [$($proj_ref_ident)?][ProjectionRef]
                    [project_ref get_ref]
                    [$($ty_generics)*]
                    {
                        $(
                            $(#[$pin])?
                            $field_vis $field
                        ),+
                    }
                }
                $crate::__pin_project_internal! { @struct=>make_proj_replace_method;
                    [$proj_vis]
                    [$($proj_replace_ident)?][ProjectionReplace]
                    [$($ty_generics)*]
                    {
                        $(
                            $(#[$pin])?
                            $field_vis $field
                        ),+
                    }
                }
            }

            $crate::__pin_project_internal! { @make_unpin_impl;
                [$vis $ident]
                [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
                $(
                    $field: $crate::__pin_project_internal!(@make_unpin_bound;
                        $(#[$pin])? $field_ty
                    )
                ),+
            }

            $crate::__pin_project_internal! { @make_drop_impl;
                [$ident]
                [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            }

            #[forbid(unaligned_references, safe_packed_borrows)]
            fn __assert_not_repr_packed <$($impl_generics)*> (this: &$ident <$($ty_generics)*>)
            $(where
                $($where_clause)*)?
            {
                $(
                    let _ = &this.$field;
                )+
            }
        };
    };

    (@enum=>internal;
        [$($proj_mut_ident:ident)?]
        [$($proj_ref_ident:ident)?]
        [$($proj_replace_ident:ident)?]
        [$proj_vis:vis]
        [$(#[$attrs:meta])* $vis:vis enum $ident:ident]
        [$($def_generics:tt)*]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)*)?]
        {
            $(
                $(#[$variant_attrs:meta])*
                $variant:ident $({
                    $(
                        $(#[$pin:ident])?
                        $field:ident: $field_ty:ty
                    ),+
                })?
            ),+
        }
    ) => {
        $(#[$attrs])*
        $vis enum $ident $($def_generics)*
        $(where
            $($where_clause)*)?
        {
            $(
                $(#[$variant_attrs])*
                $variant $({
                    $(
                        $field: $field_ty
                    ),+
                })?
            ),+
        }

        $crate::__pin_project_internal! { @enum=>make_proj_ty;
            [$proj_vis]
            [$($proj_mut_ident)?]
            [make_proj_field_mut]
            [$ident]
            [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            {
                $(
                    $variant $({
                        $(
                            $(#[$pin])?
                            $field: $field_ty
                        ),+
                    })?
                ),+
            }
        }
        $crate::__pin_project_internal! { @enum=>make_proj_ty;
            [$proj_vis]
            [$($proj_ref_ident)?]
            [make_proj_field_ref]
            [$ident]
            [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            {
                $(
                    $variant $({
                        $(
                            $(#[$pin])?
                            $field: $field_ty
                        ),+
                    })?
                ),+
            }
        }
        $crate::__pin_project_internal! { @enum=>make_proj_replace_ty;
            [$proj_vis]
            [$($proj_replace_ident)?]
            [make_proj_field_replace]
            [$ident]
            [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            {
                $(
                    $variant $({
                        $(
                            $(#[$pin])?
                            $field: $field_ty
                        ),+
                    })?
                ),+
            }
        }

        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::used_underscore_binding)]
        const _: () = {
            impl <$($impl_generics)*> $ident <$($ty_generics)*>
            $(where
                $($where_clause)*)?
            {
                $crate::__pin_project_internal! { @enum=>make_proj_method;
                    [$proj_vis]
                    [$($proj_mut_ident)?]
                    [project get_unchecked_mut mut]
                    [$($ty_generics)*]
                    {
                        $(
                            $variant $({
                                $(
                                    $(#[$pin])?
                                    $field
                                ),+
                            })?
                        ),+
                    }
                }
                $crate::__pin_project_internal! { @enum=>make_proj_method;
                    [$proj_vis]
                    [$($proj_ref_ident)?]
                    [project_ref get_ref]
                    [$($ty_generics)*]
                    {
                        $(
                            $variant $({
                                $(
                                    $(#[$pin])?
                                    $field
                                ),+
                            })?
                        ),+
                    }
                }
                $crate::__pin_project_internal! { @enum=>make_proj_replace_method;
                    [$proj_vis]
                    [$($proj_replace_ident)?]
                    [$($ty_generics)*]
                    {
                        $(
                            $variant $({
                                $(
                                    $(#[$pin])?
                                    $field
                                ),+
                            })?
                        ),+
                    }
                }
            }

            $crate::__pin_project_internal! { @make_unpin_impl;
                [$vis $ident]
                [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
                $(
                    $variant: ($(
                        $(
                            $crate::__pin_project_internal!(@make_unpin_bound;
                                $(#[$pin])? $field_ty
                            )
                        ),+
                    )?)
                ),+
            }

            $crate::__pin_project_internal! { @make_drop_impl;
                [$ident]
                [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            }
        };
    };

    (@struct=>make_proj_ty=>unnamed;
        [$proj_vis:vis]
        [$_proj_ty_ident:ident][$proj_ty_ident:ident]
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($field:tt)*
    ) => {};
    (@struct=>make_proj_ty=>unnamed;
        [$proj_vis:vis]
        [][$proj_ty_ident:ident]
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($field:tt)*
    ) => {
        $crate::__pin_project_internal! { @struct=>make_proj_ty=>named;
            [$proj_vis]
            [$proj_ty_ident]
            [$make_proj_field]
            [$ident]
            [$($impl_generics)*] [$($ty_generics)*] [$(where $($where_clause)*)?]
            $($field)*
        }
    };
    (@struct=>make_proj_ty=>named;
        [$proj_vis:vis]
        [$proj_ty_ident:ident]
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        {
            $(
                $(#[$pin:ident])?
                $field_vis:vis $field:ident: $field_ty:ty
            ),+
        }
    ) => {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        $proj_vis struct $proj_ty_ident <'__pin, $($impl_generics)*>
        where
            $ident <$($ty_generics)*>: '__pin
            $(, $($where_clause)*)?
        {
            $(
                $field_vis $field: $crate::__pin_project_internal!(@$make_proj_field;
                    $(#[$pin])? $field_ty
                )
            ),+
        }
    };
    (@struct=>make_proj_ty=>named;
        [$proj_vis:vis]
        []
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($field:tt)*
    ) => {};

    (@struct=>make_proj_replace_ty=>unnamed;
        [$proj_vis:vis]
        [$_proj_ty_ident:ident][$proj_ty_ident:ident]
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($field:tt)*
    ) => {};
    (@struct=>make_proj_replace_ty=>unnamed;
        [$proj_vis:vis]
        [][$proj_ty_ident:ident]
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($field:tt)*
    ) => {
    };
    (@struct=>make_proj_replace_ty=>named;
        [$proj_vis:vis]
        [$proj_ty_ident:ident]
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        {
            $(
                $(#[$pin:ident])?
                $field_vis:vis $field:ident: $field_ty:ty
            ),+
        }
    ) => {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::type_repetition_in_bounds)]
        $proj_vis struct $proj_ty_ident <$($impl_generics)*>
        where
            $($($where_clause)*)?
        {
            $(
                $field_vis $field: $crate::__pin_project_internal!(@$make_proj_field;
                    $(#[$pin])? $field_ty
                )
            ),+
        }
    };
    (@struct=>make_proj_replace_ty=>named;
        [$proj_vis:vis]
        []
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($field:tt)*
    ) => {};

    (@enum=>make_proj_ty;
        [$proj_vis:vis]
        [$proj_ty_ident:ident]
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        {
            $(
                $variant:ident $({
                    $(
                        $(#[$pin:ident])?
                        $field:ident: $field_ty:ty
                    ),+
                })?
            ),+
        }
    ) => {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::ref_option_ref)]
        #[allow(clippy::type_repetition_in_bounds)]
        $proj_vis enum $proj_ty_ident <'__pin, $($impl_generics)*>
        where
            $ident <$($ty_generics)*>: '__pin
            $(, $($where_clause)*)?
        {
            $(
                $variant $({
                    $(
                        $field: $crate::__pin_project_internal!(@$make_proj_field;
                            $(#[$pin])? $field_ty
                        )
                    ),+
                })?
            ),+
        }
    };
    (@enum=>make_proj_ty;
        [$proj_vis:vis]
        []
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($variant:tt)*
    ) => {};

    (@enum=>make_proj_replace_ty;
        [$proj_vis:vis]
        [$proj_ty_ident:ident]
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        {
            $(
                $variant:ident $({
                    $(
                        $(#[$pin:ident])?
                        $field:ident: $field_ty:ty
                    ),+
                })?
            ),+
        }
    ) => {
        #[allow(dead_code)]
        #[allow(single_use_lifetimes)]
        #[allow(clippy::mut_mut)]
        #[allow(clippy::redundant_pub_crate)]
        #[allow(clippy::type_repetition_in_bounds)]
        $proj_vis enum $proj_ty_ident <$($impl_generics)*>
        where
            $($($where_clause)*)?
        {
            $(
                $variant $({
                    $(
                        $field: $crate::__pin_project_internal!(@$make_proj_field;
                            $(#[$pin])? $field_ty
                        )
                    ),+
                })?
            ),+
        }
    };
    (@enum=>make_proj_replace_ty;
        [$proj_vis:vis]
        []
        [$make_proj_field:ident]
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($variant:tt)*
    ) => {};

    (@make_proj_replace_block;
        [$($proj_path: tt)+]
        {
            $(
                $(#[$pin:ident])?
                $field_vis:vis $field:ident
            ),+
        }
    ) => {
        let result = $($proj_path)* {
            $(
                $field: $crate::__pin_project_internal!(@make_replace_field_proj;
                    $(#[$pin])? $field
                )
            ),+
        };

        {
            ( $(
                $crate::__pin_project_internal!(@make_unsafe_drop_in_place_guard;
                    $(#[$pin])? $field
                ),
            )* );
        }

        result
    };
    (@make_proj_replace_block;
        [$($proj_path: tt)+]
    ) => {
        $($proj_path)*
    };

    (@struct=>make_proj_method;
        [$proj_vis:vis]
        [$proj_ty_ident:ident][$_proj_ty_ident:ident]
        [$method_ident:ident $get_method:ident $($mut:ident)?]
        [$($ty_generics:tt)*]
        {
            $(
                $(#[$pin:ident])?
                $field_vis:vis $field:ident
            ),+
        }
    ) => {
        $proj_vis fn $method_ident<'__pin>(
            self: $crate::__private::Pin<&'__pin $($mut)? Self>,
        ) -> $proj_ty_ident <'__pin, $($ty_generics)*> {
            #[allow(unused_unsafe)]
            unsafe {
                let Self { $($field),* } = self.$get_method();
                $proj_ty_ident {
                    $(
                        $field: $crate::__pin_project_internal!(@make_unsafe_field_proj;
                            $(#[$pin])? $field
                        )
                    ),+
                }
            }
        }
    };
    (@struct=>make_proj_method;
        [$proj_vis:vis]
        [][$proj_ty_ident:ident]
        [$method_ident:ident $get_method:ident $($mut:ident)?]
        [$($ty_generics:tt)*]
        $($variant:tt)*
    ) => {
        $crate::__pin_project_internal! { @struct=>make_proj_method;
            [$proj_vis]
            [$proj_ty_ident][$proj_ty_ident]
            [$method_ident $get_method $($mut)?]
            [$($ty_generics)*]
            $($variant)*
        }
    };

    (@struct=>make_proj_replace_method;
        [$proj_vis:vis]
        [$proj_ty_ident:ident][$_proj_ty_ident:ident]
        [$($ty_generics:tt)*]
        {
            $(
                $(#[$pin:ident])?
                $field_vis:vis $field:ident
            ),+
        }
    ) => {
        $proj_vis fn project_replace(
            self: $crate::__private::Pin<&mut Self>,
            replacement: Self,
        ) -> $proj_ty_ident <$($ty_generics)*> {
            unsafe {
                let __self_ptr: *mut Self = self.get_unchecked_mut();

                let __guard = $crate::__private::UnsafeOverwriteGuard {
                    target: __self_ptr,
                    value: $crate::__private::ManuallyDrop::new(replacement),
                };

                let Self { $($field),* } = &mut *__self_ptr;

                $crate::__pin_project_internal!{@make_proj_replace_block;
                    [$proj_ty_ident]
                    {
                        $(
                            $(#[$pin])?
                            $field
                        ),+
                    }
                }
            }
        }
    };
    (@struct=>make_proj_replace_method;
        [$proj_vis:vis]
        [][$proj_ty_ident:ident]
        [$($ty_generics:tt)*]
        $($variant:tt)*
    ) => {
    };

    (@enum=>make_proj_method;
        [$proj_vis:vis]
        [$proj_ty_ident:ident]
        [$method_ident:ident $get_method:ident $($mut:ident)?]
        [$($ty_generics:tt)*]
        {
            $(
                $variant:ident $({
                    $(
                        $(#[$pin:ident])?
                        $field:ident
                    ),+
                })?
            ),+
        }
    ) => {
        $proj_vis fn $method_ident<'__pin>(
            self: $crate::__private::Pin<&'__pin $($mut)? Self>,
        ) -> $proj_ty_ident <'__pin, $($ty_generics)*> {
            unsafe {
                match self.$get_method() {
                    $(
                        Self::$variant $({
                            $($field),+
                        })? => {
                            $proj_ty_ident::$variant $({
                                $(
                                    $field: $crate::__pin_project_internal!(
                                        @make_unsafe_field_proj;
                                        $(#[$pin])? $field
                                    )
                                ),+
                            })?
                        }
                    ),+
                }
            }
        }
    };
    (@enum=>make_proj_method;
        [$proj_vis:vis]
        []
        [$method_ident:ident $get_method:ident $($mut:ident)?]
        [$($ty_generics:tt)*]
        $($variant:tt)*
    ) => {};

    (@enum=>make_proj_replace_method;
        [$proj_vis:vis]
        [$proj_ty_ident:ident]
        [$($ty_generics:tt)*]
        {
            $(
                $variant:ident $({
                    $(
                        $(#[$pin:ident])?
                        $field:ident
                    ),+
                })?
            ),+
        }
    ) => {
        $proj_vis fn project_replace(
            self: $crate::__private::Pin<&mut Self>,
            replacement: Self,
        ) -> $proj_ty_ident <$($ty_generics)*> {
            unsafe {
                let __self_ptr: *mut Self = self.get_unchecked_mut();

                let __guard = $crate::__private::UnsafeOverwriteGuard {
                    target: __self_ptr,
                    value: $crate::__private::ManuallyDrop::new(replacement),
                };

                match &mut *__self_ptr {
                    $(
                        Self::$variant $({
                            $($field),+
                        })? => {
                            $crate::__pin_project_internal!{@make_proj_replace_block;
                                [$proj_ty_ident :: $variant]
                                $({
                                    $(
                                        $(#[$pin])?
                                        $field
                                    ),+
                                })?
                            }
                        }
                    ),+
                }
            }
        }
    };
    (@enum=>make_proj_replace_method;
        [$proj_vis:vis]
        []
        [$($ty_generics:tt)*]
        $($variant:tt)*
    ) => {};

    (@make_unpin_impl;
        [$vis:vis $ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
        $($field:tt)*
    ) => {
        #[allow(non_snake_case)]
        $vis struct __Origin <'__pin, $($impl_generics)*>
        $(where
            $($where_clause)*)?
        {
            __dummy_lifetime: $crate::__private::PhantomData<&'__pin ()>,
            $($field)*
        }
        impl <'__pin, $($impl_generics)*> $crate::__private::Unpin for $ident <$($ty_generics)*>
        where
            __Origin <'__pin, $($ty_generics)*>: $crate::__private::Unpin
            $(, $($where_clause)*)?
        {
        }
    };

    (@make_drop_impl;
        [$ident:ident]
        [$($impl_generics:tt)*] [$($ty_generics:tt)*] [$(where $($where_clause:tt)* )?]
    ) => {
        trait MustNotImplDrop {}
        #[allow(clippy::drop_bounds, drop_bounds)]
        impl<T: $crate::__private::Drop> MustNotImplDrop for T {}
        impl <$($impl_generics)*> MustNotImplDrop for $ident <$($ty_generics)*>
        $(where
            $($where_clause)*)?
        {
        }
    };

    (@make_unpin_bound;
        #[pin]
        $field_ty:ty
    ) => {
        $field_ty
    };
    (@make_unpin_bound;
        $field_ty:ty
    ) => {
        $crate::__private::AlwaysUnpin<$field_ty>
    };

    (@make_unsafe_field_proj;
        #[pin]
        $field:ident
    ) => {
        $crate::__private::Pin::new_unchecked($field)
    };
    (@make_unsafe_field_proj;
        $field:ident
    ) => {
        $field
    };

    (@make_replace_field_proj;
        #[pin]
        $field:ident
    ) => {
        $crate::__private::PhantomData
    };
    (@make_replace_field_proj;
        $field:ident
    ) => {
        $crate::__private::ptr::read($field)
    };

    (@make_unsafe_drop_in_place_guard;
        #[pin]
        $field:ident
    ) => {
        $crate::__private::UnsafeDropInPlaceGuard($field)
    };
    (@make_unsafe_drop_in_place_guard;
        $field:ident
    ) => {
        ()
    };

    (@make_proj_field_mut;
        #[pin]
        $field_ty:ty
    ) => {
        $crate::__private::Pin<&'__pin mut $field_ty>
    };
    (@make_proj_field_mut;
        $field_ty:ty
    ) => {
        &'__pin mut $field_ty
    };
    (@make_proj_field_ref;
        #[pin]
        $field_ty:ty
    ) => {
        $crate::__private::Pin<&'__pin $field_ty>
    };
    (@make_proj_field_ref;
        $field_ty:ty
    ) => {
        &'__pin $field_ty
    };

    (@make_proj_field_replace;
        #[pin]
        $field_ty:ty
    ) => {
        $crate::__private::PhantomData<$field_ty>
    };
    (@make_proj_field_replace;
        $field_ty:ty
    ) => {
        $field_ty
    };

    (
        []
        [$($proj_ref_ident:ident)?]
        [$($proj_replace_ident:ident)?]
        [$($attrs:tt)*]

        #[project = $proj_mut_ident:ident]
        $($tt:tt)*
    ) => {
        $crate::__pin_project_internal! {
            [$proj_mut_ident]
            [$($proj_ref_ident)?]
            [$($proj_replace_ident)?]
            [$($attrs)*]
            $($tt)*
        }
    };

    {
        [$($proj_mut_ident:ident)?]
        []
        [$($proj_replace_ident:ident)?]
        [$($attrs:tt)*]

        #[project_ref = $proj_ref_ident:ident]
        $($tt:tt)*
    } => {
        $crate::__pin_project_internal! {
            [$($proj_mut_ident)?]
            [$proj_ref_ident]
            [$($proj_replace_ident)?]
            [$($attrs)*]
            $($tt)*
        }
    };

    {
        [$($proj_mut_ident:ident)?]
        [$($proj_ref_ident:ident)?]
        []
        [$($attrs:tt)*]

        #[project_replace = $proj_replace_ident:ident]
        $($tt:tt)*
    } => {
        $crate::__pin_project_internal! {
            [$($proj_mut_ident)?]
            [$($proj_ref_ident)?]
            [$proj_replace_ident]
            [$($attrs)*]
            $($tt)*
        }
    };

    {
        [$($proj_mut_ident:ident)?]
        [$($proj_ref_ident:ident)?]
        [$($proj_replace_ident:ident)?]
        [$($attrs:tt)*]

        #[$($attr:tt)*]
        $($tt:tt)*
    } => {
        $crate::__pin_project_internal! {
            [$($proj_mut_ident)?]
            [$($proj_ref_ident)?]
            [$($proj_replace_ident)?]
            [$($attrs)* #[$($attr)*]]
            $($tt)*
        }
    };

    (
        [$($proj_mut_ident:ident)?]
        [$($proj_ref_ident:ident)?]
        [$($proj_replace_ident:ident)?]
        [$($attrs:tt)*]

        pub struct $ident:ident $(<
            $( $lifetime:lifetime $(: $lifetime_bound:lifetime)? ),* $(,)?
            $( $generics:ident
                $(: $generics_bound:path)?
                $(: ?$generics_unsized_bound:path)?
                $(: $generics_lifetime_bound:lifetime)?
                $(= $generics_default:ty)?
            ),* $(,)?
        >)?
        $(where
            $( $where_clause_ty:ty
                $(: $where_clause_bound:path)?
                $(: ?$where_clause_unsized_bound:path)?
                $(: $where_clause_lifetime_bound:lifetime)?
            ),* $(,)?
        )?
        {
            $(
                $(#[$pin:ident])?
                $field_vis:vis $field:ident: $field_ty:ty
            ),+ $(,)?
        }
    ) => {
        $crate::__pin_project_internal! { @struct=>internal;
            [$($proj_mut_ident)?]
            [$($proj_ref_ident)?]
            [$($proj_replace_ident)?]
            [pub(crate)]
            [$($attrs)* pub struct $ident]
            [$(<
                $( $lifetime $(: $lifetime_bound)? ,)*
                $( $generics
                    $(: $generics_bound)?
                    $(: ?$generics_unsized_bound)?
                    $(: $generics_lifetime_bound)?
                    $(= $generics_default)?
                ),*
            >)?]
            [$(
                $( $lifetime $(: $lifetime_bound)? ,)*
                $( $generics
                    $(: $generics_bound)?
                    $(: ?$generics_unsized_bound)?
                    $(: $generics_lifetime_bound)?
                ),*
            )?]
            [$( $( $lifetime ,)* $( $generics ),* )?]
            [$(where $( $where_clause_ty
                $(: $where_clause_bound)?
                $(: ?$where_clause_unsized_bound)?
                $(: $where_clause_lifetime_bound)?
            ),* )?]
            {
                $(
                    $(#[$pin])?
                    $field_vis $field: $field_ty
                ),+
            }
        }
    };
    (
        [$($proj_mut_ident:ident)?]
        [$($proj_ref_ident:ident)?]
        [$($proj_replace_ident:ident)?]
        [$($attrs:tt)*]

        $vis:vis struct $ident:ident $(<
            $( $lifetime:lifetime $(: $lifetime_bound:lifetime)? ),* $(,)?
            $( $generics:ident
                $(: $generics_bound:path)?
                $(: ?$generics_unsized_bound:path)?
                $(: $generics_lifetime_bound:lifetime)?
                $(= $generics_default:ty)?
            ),* $(,)?
        >)?
        $(where
            $( $where_clause_ty:ty
                $(: $where_clause_bound:path)?
                $(: ?$where_clause_unsized_bound:path)?
                $(: $where_clause_lifetime_bound:lifetime)?
            ),* $(,)?
        )?
        {
            $(
                $(#[$pin:ident])?
                $field_vis:vis $field:ident: $field_ty:ty
            ),+ $(,)?
        }
    ) => {
        $crate::__pin_project_internal! { @struct=>internal;
            [$($proj_mut_ident)?]
            [$($proj_ref_ident)?]
            [$($proj_replace_ident)?]
            [$vis]
            [$($attrs)* $vis struct $ident]
            [$(<
                $( $lifetime $(: $lifetime_bound)? ,)*
                $( $generics
                    $(: $generics_bound)?
                    $(: ?$generics_unsized_bound)?
                    $(: $generics_lifetime_bound)?
                    $(= $generics_default)?
                ),*
            >)?]
            [$(
                $( $lifetime $(: $lifetime_bound)? ,)*
                $( $generics
                    $(: $generics_bound)?
                    $(: ?$generics_unsized_bound)?
                    $(: $generics_lifetime_bound)?
                ),*
            )?]
            [$( $( $lifetime ,)* $( $generics ),* )?]
            [$(where $( $where_clause_ty
                $(: $where_clause_bound)?
                $(: ?$where_clause_unsized_bound)?
                $(: $where_clause_lifetime_bound)?
            ),* )?]
            {
                $(
                    $(#[$pin])?
                    $field_vis $field: $field_ty
                ),+
            }
        }
    };
    (
        [$($proj_mut_ident:ident)?]
        [$($proj_ref_ident:ident)?]
        [$($proj_replace_ident:ident)?]
        [$($attrs:tt)*]

        pub enum $ident:ident $(<
            $( $lifetime:lifetime $(: $lifetime_bound:lifetime)? ),* $(,)?
            $( $generics:ident
                $(: $generics_bound:path)?
                $(: ?$generics_unsized_bound:path)?
                $(: $generics_lifetime_bound:lifetime)?
                $(= $generics_default:ty)?
            ),* $(,)?
        >)?
        $(where
            $( $where_clause_ty:ty
                $(: $where_clause_bound:path)?
                $(: ?$where_clause_unsized_bound:path)?
                $(: $where_clause_lifetime_bound:lifetime)?
            ),* $(,)?
        )?
        {
            $(
                $(#[$variant_attrs:meta])*
                $variant:ident $({
                    $(
                        $(#[$pin:ident])?
                        $field:ident: $field_ty:ty
                    ),+ $(,)?
                })?
            ),+ $(,)?
        }
    ) => {
        $crate::__pin_project_internal! { @enum=>internal;
            [$($proj_mut_ident)?]
            [$($proj_ref_ident)?]
            [$($proj_replace_ident)?]
            [pub(crate)]
            [$($attrs)* pub enum $ident]
            [$(<
                $( $lifetime $(: $lifetime_bound)? ,)*
                $( $generics
                    $(: $generics_bound)?
                    $(: ?$generics_unsized_bound)?
                    $(: $generics_lifetime_bound)?
                    $(= $generics_default)?
                ),*
            >)?]
            [$(
                $( $lifetime $(: $lifetime_bound)? ,)*
                $( $generics
                    $(: $generics_bound)?
                    $(: ?$generics_unsized_bound)?
                    $(: $generics_lifetime_bound)?
                ),*
            )?]
            [$( $( $lifetime ,)* $( $generics ),* )?]
            [$(where $( $where_clause_ty
                $(: $where_clause_bound)?
                $(: ?$where_clause_unsized_bound)?
                $(: $where_clause_lifetime_bound)?
            ),* )?]
            {
                $(
                    $(#[$variant_attrs])*
                    $variant $({
                        $(
                            $(#[$pin])?
                            $field: $field_ty
                        ),+
                    })?
                ),+
            }
        }
    };
    (
        [$($proj_mut_ident:ident)?]
        [$($proj_ref_ident:ident)?]
        [$($proj_replace_ident:ident)?]
        [$($attrs:tt)*]

        $vis:vis enum $ident:ident $(<
            $( $lifetime:lifetime $(: $lifetime_bound:lifetime)? ),* $(,)?
            $( $generics:ident
                $(: $generics_bound:path)?
                $(: ?$generics_unsized_bound:path)?
                $(: $generics_lifetime_bound:lifetime)?
                $(= $generics_default:ty)?
            ),* $(,)?
        >)?
        $(where
            $( $where_clause_ty:ty
                $(: $where_clause_bound:path)?
                $(: ?$where_clause_unsized_bound:path)?
                $(: $where_clause_lifetime_bound:lifetime)?
            ),* $(,)?
        )?
        {
            $(
                $(#[$variant_attrs:meta])*
                $variant:ident $({
                    $(
                        $(#[$pin:ident])?
                        $field:ident: $field_ty:ty
                    ),+ $(,)?
                })?
            ),+ $(,)?
        }
    ) => {
        $crate::__pin_project_internal! { @enum=>internal;
            [$($proj_mut_ident)?]
            [$($proj_ref_ident)?]
            [$($proj_replace_ident)?]
            [$vis]
            [$($attrs)* $vis enum $ident]
            [$(<
                $( $lifetime $(: $lifetime_bound)? ,)*
                $( $generics
                    $(: $generics_bound)?
                    $(: ?$generics_unsized_bound)?
                    $(: $generics_lifetime_bound)?
                    $(= $generics_default)?
                ),*
            >)?]
            [$(
                $( $lifetime $(: $lifetime_bound)? ,)*
                $( $generics
                    $(: $generics_bound)?
                    $(: ?$generics_unsized_bound)?
                    $(: $generics_lifetime_bound)?
                ),*
            )?]
            [$( $( $lifetime ,)* $( $generics ),* )?]
            [$(where $( $where_clause_ty
                $(: $where_clause_bound)?
                $(: ?$where_clause_unsized_bound)?
                $(: $where_clause_lifetime_bound)?
            ),* )?]
            {
                $(
                    $(#[$variant_attrs])*
                    $variant $({
                        $(
                            $(#[$pin])?
                            $field: $field_ty
                        ),+
                    })?
                ),+
            }
        }
    };
}

pub mod __private {

    pub use core::{
        marker::{PhantomData, Unpin},
        mem::ManuallyDrop,
        ops::Drop,
        pin::Pin,
        ptr,
    };

    pub struct AlwaysUnpin<T: ?Sized>(PhantomData<T>);

    impl<T: ?Sized> Unpin for AlwaysUnpin<T> {}

    pub struct UnsafeDropInPlaceGuard<T: ?Sized>(pub *mut T);

    impl<T: ?Sized> Drop for UnsafeDropInPlaceGuard<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::drop_in_place(self.0);
            }
        }
    }

    pub struct UnsafeOverwriteGuard<T> {
        pub value: ManuallyDrop<T>,
        pub target: *mut T,
    }

    impl<T> Drop for UnsafeOverwriteGuard<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::write(self.target, ptr::read(&*self.value));
            }
        }
    }
}

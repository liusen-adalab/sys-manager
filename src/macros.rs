#[macro_export]
macro_rules! log_if_err {
    ($run:expr) => {
        crate::log_if_err!($run, stringify!($run))
    };

    ($run:expr, $msg:expr $(,)?) => {
        if let Err(err) = $run {
            ::tracing::error!(?err, concat!("FAILED: ", $msg))
        }
    };
}

#[macro_export]
macro_rules! log_err_ctx {
    ({$($runs:expr)*}) => {{
        let context = || {};
        crate::log_err_ctx!(@invoke $($runs)*, context)
    }};

    ({$($runs:expr)*} $(,$fields:ident)+ $(,)?) => {{
        let context = || {
            ::tracing::info!($(?$fields,)+);
        };
        crate::log_err_ctx!(@invoke $($runs)*, context)
    }};

    (@invoke $($runs:expr)*, $context:ident) => {{
        #[allow(redundant_semicolons)]
        {
            $(;crate::log_err_ctx!(@closure $runs, $context))*
        }
    }};

    (@closure $run:expr, $context:ident) => {{
        match $run {
            Ok(ok) => ok,
            Err(err) => {
                ::tracing::info!(concat!("FAILED: ", stringify!($run)));
                $context();
                return Err(err.into());
            }
        }
    }};

    ($run:expr $(, $fileds:ident)* $(,)?) => {{
        match $run {
            Ok(ok) => ok,
            Err(err) => {
                ::tracing::info!($(?$fileds,)* concat!("FAILED: ", stringify!($run)));
                return Err(err.into());
            }
        }
    }};
}

#[macro_export]
macro_rules! build_endpoint {
    (@build_endpoint
        {
            $mod_name:expr,
            [$($err_items:ident,)*]
        },
        {
            $endpoint:ident,
            $cur_err_index:expr,
            $(
                {
                    $field_name:ident,
                    $named_ty:ty,
                    $actual_ty:ty,
                }
            )*
        }
        $prv_ty:ty,
        $($tail:tt)*
    ) => {
        paste::paste!{
            struct [< $endpoint $prv_ty:camel >];

            const [< $endpoint $prv_ty>]: Err = Err {
                code: $cur_err_index,
                msg: concat!("/", $mod_name, "/", stringify!($endpoint), "/", stringify!($prv_ty))
            };

            impl [< $endpoint $prv_ty:camel >] {
                const fn generate() -> Err {
                    [< $endpoint $prv_ty>]
                }
            }
        }

        paste::paste!{
            build_endpoint!{
                @build_endpoint
                {
                    $mod_name,
                    [$($err_items,)* [< $endpoint $prv_ty>],]
                },
                {
                    $endpoint,
                    $cur_err_index + 10,
                    $(
                        {
                            $field_name,
                            $named_ty,
                            $actual_ty,
                        }
                    )*
                        {
                            [< $prv_ty:lower >],
                            [< $endpoint $prv_ty:camel >],
                            Err,
                        }
                }
                $($tail)*
            }
        }
    };

    (@build_endpoint
        {
            $mod_name:expr,
            [$($err_items:ident,)*]
        },
        {
            $endpoint:ident,
            $cur_err_index:expr,
            $(
                {
                    $field_name:ident,
                    $named_ty:ty,
                    $actual_ty:ty,
                }
            )*
        }
    use $pub_ty:ty,
    $($tail:tt)*
    ) => {
        paste::paste!{
            build_endpoint!{
                @build_endpoint
                {
                    $mod_name,
                    [$($err_items,)*]
                },
                {
                    $endpoint,
                    $cur_err_index,
                    $(
                        {
                            $field_name,
                            $named_ty,
                            $actual_ty,
                        }
                    )*
                        {
                            [< $pub_ty:lower >],
                            $pub_ty,
                            $pub_ty,
                        }
                }
                $($tail)*
            }
        }
    };


    (@build_endpoint
        {
            $mod_name:expr,
            [$($err_items:ident,)*]
        },
        {
            $endpoint:ident,
            $cur_err_index:expr,
            $(
                {
                    $field_name:ident,
                    $named_ty:ty,
                    $actual_ty:ty,
                }
            )*
        }
        $(,)?
    ) => {
        pub struct $endpoint {
            $(pub $field_name: $actual_ty,)*
        }

        paste::paste!{
            fn [< doc_ $endpoint >]() -> Vec<Document> {
                vec![
                    $(
                        Document {
                            err: $err_items,
                            endpoint: stringify!($endpoint)
                        },
                    )*
                ]
            }

            pub static [< $endpoint:upper >]: $endpoint = $endpoint {
                $($field_name: $named_ty::generate(),)*
            };
        }
    };
}

#[macro_export]
macro_rules! err_items {
    ({$mod_name:literal, $err_ty:ident}, $cur_code:expr, $(,)?) => {};

    ({$mod_name:literal, $err_ty:ident}, $cur_code:expr, $item_name:ident, $($tail:tt)*) => {
        paste::paste! {
            const [< $err_ty $item_name >]: Err = Err {
                code: $cur_code,
                msg: concat!("/", $mod_name, "/", stringify!($err_ty), "/", stringify!($item_name)),
            };
            crate::err_items!({$mod_name, $err_ty}, $cur_code + 1, $($tail)*);
        }
    };

}

#[macro_export]
macro_rules! doc_pub {
    (
        $(
        pub $pub_err:ident = $pub_code:literal {
            $($pub_err_item:tt),* $(,)?
        }
        )*
    ) => {
        pub fn doc_pub() -> Vec<Document> {
            paste::paste! {
                vec![
                    $(
                        $(
                        Document {
                            err: [< $pub_err $pub_err_item >],
                            endpoint: "public"
                        },
                        )*
                    )*
                ]
            }
        }
    };
}

#[macro_export]
macro_rules! code {
    (
        mod = $mod_name:literal
        index = $index:literal
        err = $err:path;

        $(pub $pub_err:ident = $pub_code:literal {
            $($pub_err_item:tt),* $(,)?
        })*

        ---
        $($tts:tt)*
    ) => {
        pub use code::*;
        mod code {
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[derive(Debug, Copy, Clone)]
            pub struct Err {
                pub code: u32,
                pub msg: &'static str,
            }

            impl From<Err> for $err {
                fn from(value: Err) -> $err {
                    <$err>::BusinessError(value.code, value.msg.to_string())
                }
            }

            #[derive(Debug, Copy, Clone)]
            pub struct Document {
                pub err: Err,
                pub endpoint: &'static str
            }


            $(
                #[allow(non_snake_case)]
                #[derive(Debug, Copy, Clone)]
                pub struct $pub_err {
                    $($pub_err_item: Err,)*
                }

                crate::err_items!({$mod_name, $pub_err}, $pub_code, $($pub_err_item,)*);

                impl $pub_err {
                    pub const fn generate() -> Self {

                        paste::paste! {
                            $pub_err {
                                $($pub_err_item: [< $pub_err $pub_err_item >],)*
                            }
                        }
                    }
                }
            )*

            crate::doc_pub!(
            $(pub $pub_err = $pub_code{
                $($pub_err_item),*
            })*
            );

            code!(@build_endpoint {$mod_name, $index * 100}, $($tts)*);

            pub fn err_list() -> Vec<Document> {
                let mut doc_pub = doc_pub();
                let endpoint_docs =  crate::doc_encpoints!($($tts)*);

                doc_pub.extend(endpoint_docs);
                doc_pub
            }

            pub fn document() -> Vec<(u32, &'static str, &'static str)> {
                let errs = err_list();
                errs.into_iter()
                    .map(|d| (d.err.code, d.endpoint, d.err.msg))
                    .collect()
            }

            pub fn doc_csv() -> String {
                let mut table = String::from("code, endpoint, msg\n");
                for d in err_list() {
                    table += &format!("{},{},{}\n", d.err.code, d.endpoint, d.err.msg);
                }

                table
            }
        }
    };

    (@build_endpoint {$mod_name:expr, $cur_endpoint_index:expr }, $endpoint:ident {$($fields:tt)*} $($tts:tt)* ) => {
        build_endpoint!(@build_endpoint {$mod_name, []}, {$endpoint, $cur_endpoint_index * 100,} $($fields)*,);
        code!(@build_endpoint {$mod_name, ($cur_endpoint_index + 1)}, $($tts)*);
    };

    (@build_endpoint {$mod_name:expr, $cur_endpoint_index:expr }, $(,)? ) => {
    };
}

#[macro_export]
macro_rules! doc_encpoints {
    ($($endpoint:ident {$($fields:tt)*})*) => {
        {
            paste::paste!{
                let docs = vec![
                    $([< doc_ $endpoint >]() ,)*
                ];
            }
            let docs: Vec<_> = docs.into_iter().flatten().collect();
            docs
        }
    };
}

#[cfg(test)]
mod aa {
    #![allow(dead_code)]
    code! {
        mod = "user"
        index = 10
        err = crate::http::ApiError;

        pub Password = 20 {
            too_long,
            too_short
        }

        ---

        Register {
            use Password,
            alredy_register,
            alredy_register2,
        }

        Login {
            use Password,
            alredy_register,
        }
    }

    #[test]
    fn t_code() {
        dbg!(REGISTER.password);
        dbg!(REGISTER.alredy_register);
        dbg!(LOGIN.alredy_register);
        dbg!(LOGIN.password);
        println!("{}", doc_csv());
    }
}

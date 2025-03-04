use akshare_macros::define_op;

define_op! {
    name: stock_sse_summary,
    params: {
        // no param
    }
}

define_op! {
    name: stock_szse_summary,
    params: {
        date: str,
    }
}

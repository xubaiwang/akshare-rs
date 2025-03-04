pub mod others {
    pub mod others {
        use akshare_macros::define_op;
        define_op! {
            name: car_market_total_cpca,
            params: {
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_js_weibo_report,
            params: {
                time_period: str,
            },
        }
        define_op! {
            name: car_sale_rank_gasgoo,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: cost_living,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: sunrise_daily,
            params: {
                date: str,
                city: str,
            },
        }
        define_op! {
            name: movie_boxoffice_monthly,
            params: {
                date: str,
            },
        }
        define_op! {
            name: movie_boxoffice_yearly,
            params: {
                date: str,
            },
        }
        define_op! {
            name: online_value_artist,
            params: {
            },
        }
        define_op! {
            name: forbes_rank,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: news_cctv,
            params: {
                date: str,
            },
        }
        define_op! {
            name: car_market_segment_cpca,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: air_quality_rank,
            params: {
                date: str,
            },
        }
        define_op! {
            name: xincaifu_rank,
            params: {
                year: str,
            },
        }
        define_op! {
            name: car_market_cate_cpca,
            params: {
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: video_variety_show,
            params: {
            },
        }
        define_op! {
            name: business_value_artist,
            params: {
            },
        }
        define_op! {
            name: car_market_country_cpca,
            params: {
            },
        }
        define_op! {
            name: movie_boxoffice_weekly,
            params: {
                date: str,
            },
        }
        define_op! {
            name: index_bloomberg_billionaires,
            params: {
            },
        }
        define_op! {
            name: sunrise_monthly,
            params: {
                date: str,
                city: str,
            },
        }
        define_op! {
            name: index_bloomberg_billionaires_hist,
            params: {
                year: str,
            },
        }
        define_op! {
            name: car_market_man_rank_cpca,
            params: {
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: air_quality_watch_point,
            params: {
                city: str,
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: air_quality_hebei,
            params: {
            },
        }
        define_op! {
            name: car_market_fuel_cpca,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fortune_rank,
            params: {
                year: str,
            },
        }
        define_op! {
            name: video_tv,
            params: {
            },
        }
        define_op! {
            name: movie_boxoffice_daily,
            params: {
                date: str,
            },
        }
        define_op! {
            name: movie_boxoffice_cinema_daily,
            params: {
                date: str,
            },
        }
        define_op! {
            name: air_city_table,
            params: {
            },
        }
        define_op! {
            name: movie_boxoffice_cinema_weekly,
            params: {
                date: str,
            },
        }
        define_op! {
            name: movie_boxoffice_yearly_first_week,
            params: {
                date: str,
            },
        }
        define_op! {
            name: sport_olympic_hist,
            params: {
            },
        }
        define_op! {
            name: air_quality_hist,
            params: {
                period: str,
                start_date: str,
                end_date: str,
                city: str,
            },
        }
        define_op! {
            name: movie_boxoffice_realtime,
            params: {
            },
        }
        define_op! {
            name: hurun_rank,
            params: {
                indicator: str,
                year: str,
            },
        }
    }
}
pub mod tool {
    pub mod tool {
        use akshare_macros::define_op;
        define_op! {
            name: tool_trade_date_hist_sina,
            params: {
            },
        }
    }
}
pub mod interest_rate {
    pub mod interest_rate {
        use akshare_macros::define_op;
        define_op! {
            name: repo_rate_query,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: macro_bank_newzealand_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_euro_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_australia_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_usa_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_switzerland_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_english_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_russia_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_japan_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_china_interest_rate,
            params: {
            },
        }
        define_op! {
            name: macro_bank_india_interest_rate,
            params: {
            },
        }
        define_op! {
            name: rate_interbank,
            params: {
                symbol: str,
                market: str,
                indicator: str,
            },
        }
        define_op! {
            name: macro_bank_brazil_interest_rate,
            params: {
            },
        }
        define_op! {
            name: repo_rate_hist,
            params: {
                end_date: str,
                start_date: str,
            },
        }
    }
}
pub mod futures {
    pub mod futures {
        use akshare_macros::define_op;
        define_op! {
            name: futures_delivery_czce,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_to_spot_shfe,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_index_min_ccidx,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_contract_info_gfex,
            params: {
            },
        }
        define_op! {
            name: futures_inventory_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_to_spot_dce,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_contract_info_ine,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_zh_spot,
            params: {
                subscribe_list: str,
                adjust: str,
                market: str,
            },
        }
        define_op! {
            name: futures_zh_realtime,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_hq_subscribe_exchange_symbol,
            params: {
            },
        }
        define_op! {
            name: futures_spot_stock,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_comm_info,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_czce_warehouse_receipt,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_foreign_commodity_realtime,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_hog_spot_price,
            params: {
            },
        }
        define_op! {
            name: futures_hold_pos_sina,
            params: {
                symbol: str,
                contract: str,
                date: str,
            },
        }
        define_op! {
            name: futures_delivery_dce,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_delivery_shfe,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_zh_minute_sina,
            params: {
                symbol: str,
                period: str,
            },
        }
        define_op! {
            name: futures_dce_warehouse_receipt,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_contract_info_dce,
            params: {
            },
        }
        define_op! {
            name: futures_global_em,
            params: {
            },
        }
        define_op! {
            name: futures_spot_sys,
            params: {
                symbol: str,
                contract: str,
            },
        }
        define_op! {
            name: futures_foreign_detail,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_rule,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_shfe_warehouse_receipt,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_to_spot_czce,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_delivery_match_czce,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_dce_position_rank,
            params: {
                vars_list: str,
                date: str,
            },
        }
        define_op! {
            name: futures_hist_em,
            params: {
                period: str,
                start_date: str,
                end_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: futures_settlement_price_sgx,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_contract_info_cffex,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_zh_daily_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_contract_detail,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_hog_cost,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_gfex_warehouse_receipt,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_index_ccidx,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_inventory_99,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_contract_info_shfe,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_contract_info_czce,
            params: {
                date: str,
            },
        }
        define_op! {
            name: futures_comex_inventory,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_fees_info,
            params: {
            },
        }
        define_op! {
            name: futures_stock_shfe_js,
            params: {
                date: str,
            },
        }
        define_op! {
            name: get_futures_daily,
            params: {
                market: str,
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: futures_hog_core,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_main_sina,
            params: {
                symbol: str,
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: futures_foreign_hist,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_delivery_match_dce,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_gfex_position_rank,
            params: {
                date: str,
                vars_list: str,
            },
        }
        define_op! {
            name: futures_hog_supply,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: futures_news_shmet,
            params: {
                symbol: str,
            },
        }
    }
}
pub mod dc {
    pub mod dc {
        use akshare_macros::define_op;
        define_op! {
            name: crypto_bitcoin_hold_report,
            params: {
            },
        }
        define_op! {
            name: crypto_bitcoin_cme,
            params: {
                date: str,
            },
        }
        define_op! {
            name: crypto_js_spot,
            params: {
            },
        }
    }
}
pub mod article {
    pub mod article {
        use akshare_macros::define_op;
        define_op! {
            name: article_oman_rv,
            params: {
                symbol: str,
                index: str,
            },
        }
        define_op! {
            name: article_rlab_rv,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: article_ff_crr,
            params: {
            },
        }
        define_op! {
            name: article_epu_index,
            params: {
                symbol: str,
            },
        }
    }
}
pub mod option {
    pub mod option {
        use akshare_macros::define_op;
        define_op! {
            name: option_cffex_hs300_list_sina,
            params: {
            },
        }
        define_op! {
            name: option_sse_greeks_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_sse_minute_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_risk_analysis_em,
            params: {
            },
        }
        define_op! {
            name: option_commodity_hist_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_sse_expire_day_sina,
            params: {
                symbol: str,
                exchange: str,
                trade_date: str,
            },
        }
        define_op! {
            name: option_cffex_zz1000_spot_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_risk_indicator_sse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: option_daily_stats_szse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: option_sse_list_sina,
            params: {
                symbol: str,
                exchange: str,
            },
        }
        define_op! {
            name: option_cffex_zz1000_list_sina,
            params: {
            },
        }
        define_op! {
            name: option_shfe_daily,
            params: {
                symbol: str,
                trade_date: str,
            },
        }
        define_op! {
            name: option_cffex_hs300_spot_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_premium_analysis_em,
            params: {
            },
        }
        define_op! {
            name: option_sse_daily_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_comm_info,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_cffex_sz50_daily_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_czce_daily,
            params: {
                trade_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: option_commodity_contract_table_sina,
            params: {
                symbol: str,
                contract: str,
            },
        }
        define_op! {
            name: option_cffex_sz50_spot_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_sse_spot_price_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_cffex_hs300_daily_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_current_em,
            params: {
            },
        }
        define_op! {
            name: option_lhb_em,
            params: {
                symbol: str,
                indicator: str,
                trade_date: str,
            },
        }
        define_op! {
            name: option_value_analysis_em,
            params: {
            },
        }
        define_op! {
            name: option_gfex_vol_daily,
            params: {
                symbol: str,
                trade_date: str,
            },
        }
        define_op! {
            name: option_cffex_sz50_list_sina,
            params: {
            },
        }
        define_op! {
            name: option_finance_board,
            params: {
                end_month: str,
                symbol: str,
            },
        }
        define_op! {
            name: option_finance_minute_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_dce_daily,
            params: {
                symbol: str,
                trade_date: str,
            },
        }
        define_op! {
            name: option_czce_hist,
            params: {
                symbol: str,
                year: str,
            },
        }
        define_op! {
            name: option_sse_codes_sina,
            params: {
                trade_date: str,
                underlying: str,
                symbol: str,
            },
        }
        define_op! {
            name: option_commodity_contract_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_sse_underlying_spot_price_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_daily_stats_sse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: option_minute_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: option_gfex_daily,
            params: {
                trade_date: str,
                symbol: str,
            },
        }
    }
}
pub mod qdii {
    pub mod qdii {
        use akshare_macros::define_op;
        define_op! {
            name: qdii_e_comm_jsl,
            params: {
            },
        }
        define_op! {
            name: qdii_a_index_jsl,
            params: {
            },
        }
        define_op! {
            name: qdii_e_index_jsl,
            params: {
            },
        }
    }
}
pub mod event {
    pub mod event {
        use akshare_macros::define_op;
        define_op! {
            name: migration_area_baidu,
            params: {
                date: str,
                area: str,
                indicator: str,
            },
        }
        define_op! {
            name: migration_scale_baidu,
            params: {
                indicator: str,
                area: str,
            },
        }
    }
}
pub mod stock {
    pub mod stock {
        use akshare_macros::define_op;
        define_op! {
            name: stock_comment_detail_scrd_desire_daily_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_yjyg_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_zh_kcb_daily,
            params: {
                adjust: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_info_cjzc_em,
            params: {
            },
        }
        define_op! {
            name: stock_board_industry_index_ths,
            params: {
                symbol: str,
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_gdfx_free_holding_statistics_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_qsjy_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_fhps_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_gddh_em,
            params: {
            },
        }
        define_op! {
            name: stock_ebs_lg,
            params: {
            },
        }
        define_op! {
            name: stock_board_concept_index_ths,
            params: {
                end_date: str,
                start_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_dxsyl_em,
            params: {
            },
        }
        define_op! {
            name: stock_individual_fund_flow_rank,
            params: {
                indicator: str,
            },
        }
        define_op! {
            name: stock_hk_gxl_lg,
            params: {
            },
        }
        define_op! {
            name: stock_value_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_sy_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_financial_cash_ths,
            params: {
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_qbzf_em,
            params: {
            },
        }
        define_op! {
            name: stock_board_concept_info_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zt_pool_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_zh_ah_name,
            params: {
            },
        }
        define_op! {
            name: stock_rank_lxsz_ths,
            params: {
            },
        }
        define_op! {
            name: stock_news_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_profit_forecast_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_rank_cxfl_ths,
            params: {
            },
        }
        define_op! {
            name: stock_zcfz_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_index_pe_lg,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_changes_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_a_disclosure_report_cninfo,
            params: {
                start_date: str,
                end_date: str,
                symbol: str,
                market: str,
                keyword: str,
                category: str,
            },
        }
        define_op! {
            name: stock_kc_a_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_institute_recommend_detail,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_info_sh_name_code,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hold_num_cninfo,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_rank_cxg_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_esg_zd_sina,
            params: {
            },
        }
        define_op! {
            name: stock_gdfx_free_top_10_em,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_hot_rank_detail_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_info_global_cls,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_sz_a_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_financial_report_sina,
            params: {
                symbol: str,
                stock: str,
            },
        }
        define_op! {
            name: stock_info_change_name,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_esg_msci_sina,
            params: {
            },
        }
        define_op! {
            name: stock_lhb_yybph_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hold_management_detail_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_gdfx_free_holding_change_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_lhb_yytj_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zt_pool_dtgc_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_ipo_benefit_ths,
            params: {
            },
        }
        define_op! {
            name: stock_comment_em,
            params: {
            },
        }
        define_op! {
            name: stock_financial_hk_report_em,
            params: {
                stock: str,
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_jgdy_detail_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_hsgt_fund_min_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_a_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_lh_yyb_control,
            params: {
            },
        }
        define_op! {
            name: stock_zh_kcb_spot,
            params: {
            },
        }
        define_op! {
            name: stock_gpzy_industry_data_em,
            params: {
            },
        }
        define_op! {
            name: stock_profit_sheet_by_quarterly_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_market_pe_lg,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_index_pb_lg,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_restricted_release_queue_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_sse_summary,
            params: {
            },
        }
        define_op! {
            name: stock_zh_ah_spot,
            params: {
            },
        }
        define_op! {
            name: stock_hk_indicator_eniu,
            params: {
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_zygc_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_cyq_em,
            params: {
                symbol: str,
                adjust: str,
            },
        }
        define_op! {
            name: stock_lhb_stock_detail_em,
            params: {
                flag: str,
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_us_pink_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_hsgt_fund_flow_summary_em,
            params: {
            },
        }
        define_op! {
            name: stock_lh_yyb_capital,
            params: {
            },
        }
        define_op! {
            name: stock_us_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_a_below_net_asset_statistics,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_intraday_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hk_ggt_components_em,
            params: {
            },
        }
        define_op! {
            name: stock_gpzy_distribute_statistics_bank_em,
            params: {
            },
        }
        define_op! {
            name: stock_a_indicator_lg,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_industry_change_cninfo,
            params: {
                start_date: str,
                end_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_report_disclosure,
            params: {
                market: str,
                period: str,
            },
        }
        define_op! {
            name: stock_fhps_detail_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_sector_detail,
            params: {
                sector: str,
            },
        }
        define_op! {
            name: stock_info_sz_change_name,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_repurchase_em,
            params: {
            },
        }
        define_op! {
            name: stock_sector_fund_flow_rank,
            params: {
                indicator: str,
                sector_type: str,
            },
        }
        define_op! {
            name: stock_board_concept_hist_em,
            params: {
                symbol: str,
                period: str,
                end_date: str,
                adjust: str,
                start_date: str,
            },
        }
        define_op! {
            name: stock_main_stock_holder,
            params: {
                stock: str,
            },
        }
        define_op! {
            name: stock_analyst_detail_em,
            params: {
                analyst_id: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_margin_underlying_info_szse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_esg_hz_sina,
            params: {
            },
        }
        define_op! {
            name: stock_hsgt_stock_statistics_em,
            params: {
                end_date: str,
                start_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_restricted_release_summary_em,
            params: {
                end_date: str,
                symbol: str,
                start_date: str,
            },
        }
        define_op! {
            name: stock_ipo_summary_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hot_rank_relate_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_a_congestion_lg,
            params: {
            },
        }
        define_op! {
            name: stock_register_db,
            params: {
            },
        }
        define_op! {
            name: stock_zh_a_stop_em,
            params: {
            },
        }
        define_op! {
            name: stock_share_change_cninfo,
            params: {
                end_date: str,
                symbol: str,
                start_date: str,
            },
        }
        define_op! {
            name: stock_sgt_reference_exchange_rate_szse,
            params: {
            },
        }
        define_op! {
            name: stock_profit_forecast_ths,
            params: {
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_b_minute,
            params: {
                symbol: str,
                adjust: str,
                period: str,
            },
        }
        define_op! {
            name: stock_rank_xstp_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_gpzy_pledge_ratio_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_gpzy_pledge_ratio_detail_em,
            params: {
            },
        }
        define_op! {
            name: stock_history_dividend,
            params: {
            },
        }
        define_op! {
            name: stock_info_a_code_name,
            params: {
            },
        }
        define_op! {
            name: stock_board_industry_cons_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_info_global_futu,
            params: {
            },
        }
        define_op! {
            name: stock_yjkb_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_individual_info_em,
            params: {
                timeout: float,
                symbol: str,
            },
        }
        define_op! {
            name: stock_fund_stock_holder,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hk_profit_forecast_et,
            params: {
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_restricted_release_queue_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_comment_detail_zlkp_jgcyd_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_ggcg_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_us_hist,
            params: {
                end_date: str,
                period: str,
                adjust: str,
                start_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_buffett_index_lg,
            params: {
            },
        }
        define_op! {
            name: stock_lh_yyb_most,
            params: {
            },
        }
        define_op! {
            name: stock_lhb_detail_daily_sina,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_hk_main_board_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_zh_a_hist_tx,
            params: {
                symbol: str,
                start_date: str,
                adjust: str,
                timeout: float,
                end_date: str,
            },
        }
        define_op! {
            name: stock_hot_rank_em,
            params: {
            },
        }
        define_op! {
            name: stock_hold_management_person_em,
            params: {
                name: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_kcb_report_em,
            params: {
                to_page: int,
                from_page: int,
            },
        }
        define_op! {
            name: stock_margin_ratio_pa,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_hsgt_individual_em,
            params: {
                stock: str,
            },
        }
        define_op! {
            name: stock_hold_control_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_gdfx_holding_teamwork_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hold_change_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_analyst_rank_em,
            params: {
                year: str,
            },
        }
        define_op! {
            name: stock_zh_ah_daily,
            params: {
                adjust: str,
                symbol: str,
                end_year: str,
                start_year: str,
            },
        }
        define_op! {
            name: stock_staq_net_stop,
            params: {
            },
        }
        define_op! {
            name: stock_register_sz,
            params: {
            },
        }
        define_op! {
            name: stock_hot_search_baidu,
            params: {
                date: str,
                symbol: str,
                time: str,
            },
        }
        define_op! {
            name: stock_industry_clf_hist_sw,
            params: {
            },
        }
        define_op! {
            name: stock_irm_ans_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_a_cdr_daily,
            params: {
                end_date: str,
                symbol: str,
                start_date: str,
            },
        }
        define_op! {
            name: stock_new_gh_cninfo,
            params: {
            },
        }
        define_op! {
            name: stock_margin_detail_szse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_hk_hot_rank_detail_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_yzxdr_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_xgsglb_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_board_concept_name_em,
            params: {
            },
        }
        define_op! {
            name: stock_industry_category_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hsgt_institution_statistics_em,
            params: {
                end_date: str,
                market: str,
                start_date: str,
            },
        }
        define_op! {
            name: stock_info_global_ths,
            params: {
            },
        }
        define_op! {
            name: stock_zh_a_new,
            params: {
            },
        }
        define_op! {
            name: stock_zh_a_new_em,
            params: {
            },
        }
        define_op! {
            name: stock_zygc_ym,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_share_hold_change_sse,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: news_report_time_baidu,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_zh_vote_baidu,
            params: {
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_hk_valuation_baidu,
            params: {
                indicator: str,
                period: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_a_high_low_statistics,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hk_fhpx_detail_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_cash_flow_sheet_by_quarterly_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_management_change_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hot_up_em,
            params: {
            },
        }
        define_op! {
            name: stock_institute_hold_detail,
            params: {
                stock: str,
                quarter: str,
            },
        }
        define_op! {
            name: stock_rank_lxxd_ths,
            params: {
            },
        }
        define_op! {
            name: stock_allotment_cninfo,
            params: {
                end_date: str,
                start_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_market_fund_flow,
            params: {
            },
        }
        define_op! {
            name: stock_financial_benefit_ths,
            params: {
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_market_pb_lg,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_margin_account_info,
            params: {
            },
        }
        define_op! {
            name: stock_info_global_em,
            params: {
            },
        }
        define_op! {
            name: stock_circulate_stock_holder,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_restricted_release_detail_em,
            params: {
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_zh_a_hist_min_em,
            params: {
                symbol: str,
                start_date: str,
                end_date: str,
                adjust: str,
                period: str,
            },
        }
        define_op! {
            name: stock_szse_area_summary,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_comment_detail_scrd_desire_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hk_hist_min_em,
            params: {
                end_date: str,
                symbol: str,
                period: str,
                start_date: str,
                adjust: str,
            },
        }
        define_op! {
            name: stock_sy_hy_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_sector_fund_flow_summary,
            params: {
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_sgt_settlement_exchange_rate_sse,
            params: {
            },
        }
        define_op! {
            name: stock_zcfz_bj_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_board_change_em,
            params: {
            },
        }
        define_op! {
            name: stock_sgt_reference_exchange_rate_sse,
            params: {
            },
        }
        define_op! {
            name: stock_comment_detail_scrd_cost_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_info_global_sina,
            params: {
            },
        }
        define_op! {
            name: stock_sector_fund_flow_hist,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_fund_flow_concept,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_us_hist_min_em,
            params: {
                start_date: str,
                symbol: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_fhps_detail_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_financial_abstract_ths,
            params: {
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_gdfx_holding_change_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_gdfx_free_holding_analyse_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_cg_equity_mortgage_cninfo,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_lhb_traderstatistic_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_dzjy_yybph,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_board_concept_hist_min_em,
            params: {
                symbol: str,
                period: str,
            },
        }
        define_op! {
            name: stock_tfp_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_cash_flow_sheet_by_report_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_ipo_info,
            params: {
                stock: str,
            },
        }
        define_op! {
            name: stock_info_bj_name_code,
            params: {
            },
        }
        define_op! {
            name: stock_board_industry_hist_em,
            params: {
                start_date: str,
                end_date: str,
                period: str,
                adjust: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_register_cyb,
            params: {
            },
        }
        define_op! {
            name: stock_zt_pool_strong_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_rank_cxsl_ths,
            params: {
            },
        }
        define_op! {
            name: stock_lhb_jgstatistic_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_shareholder_change_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_balance_sheet_by_yearly_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_balance_sheet_by_report_delisted_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_fund_flow_big_deal,
            params: {
            },
        }
        define_op! {
            name: stock_financial_analysis_indicator,
            params: {
                start_year: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_bj_a_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_zh_b_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_hold_management_detail_em,
            params: {
            },
        }
        define_op! {
            name: stock_dzjy_hyyybtj,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_xjll_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_market_activity_legu,
            params: {
            },
        }
        define_op! {
            name: stock_us_spot,
            params: {
            },
        }
        define_op! {
            name: stock_margin_sse,
            params: {
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_zh_a_disclosure_relation_cninfo,
            params: {
                start_date: str,
                symbol: str,
                market: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_board_industry_hist_min_em,
            params: {
                period: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_jgdy_tj_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_financial_hk_analysis_indicator_em,
            params: {
                symbol: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_hsgt_hold_stock_em,
            params: {
                market: str,
                indicator: str,
            },
        }
        define_op! {
            name: stock_restricted_release_stockholder_em,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_hot_follow_xq,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_dzjy_hygtj,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_a_all_pb,
            params: {
            },
        }
        define_op! {
            name: stock_zh_a_daily,
            params: {
                start_date: str,
                adjust: str,
                symbol: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_zdhtmx_em,
            params: {
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: stock_szse_summary,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_sse_deal_daily,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_notice_report,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_dividend_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_report_fund_hold_detail,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_price_js,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_lhb_jgmx_sina,
            params: {
            },
        }
        define_op! {
            name: stock_register_bj,
            params: {
            },
        }
        define_op! {
            name: stock_sy_yq_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_hk_hot_rank_em,
            params: {
            },
        }
        define_op! {
            name: stock_hk_hist,
            params: {
                symbol: str,
                end_date: str,
                start_date: str,
                period: str,
                adjust: str,
            },
        }
        define_op! {
            name: stock_zh_a_st_em,
            params: {
            },
        }
        define_op! {
            name: stock_gpzy_profile_em,
            params: {
            },
        }
        define_op! {
            name: stock_gdfx_free_holding_teamwork_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_institute_recommend,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hk_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_individual_spot_xq,
            params: {
                timeout: float,
                symbol: str,
                token: float,
            },
        }
        define_op! {
            name: stock_pg_em,
            params: {
            },
        }
        define_op! {
            name: stock_zt_pool_previous_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_new_a_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_zt_pool_zbgc_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_gpzy_distribute_statistics_company_em,
            params: {
            },
        }
        define_op! {
            name: stock_zt_pool_sub_new_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_concept_cons_futu,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_fund_flow_industry,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_lrb_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_margin_detail_sse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_sy_jz_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_irm_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_dzjy_sctj,
            params: {
            },
        }
        define_op! {
            name: stock_cg_lawsuit_cninfo,
            params: {
                start_date: str,
                symbol: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_hk_spot,
            params: {
            },
        }
        define_op! {
            name: stock_register_kcb,
            params: {
            },
        }
        define_op! {
            name: stock_lhb_stock_statistic_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_lhb_ggtj_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_profile_cninfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_margin_szse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_rank_xxtp_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_rank_ljqd_ths,
            params: {
            },
        }
        define_op! {
            name: stock_main_fund_flow,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_b_daily,
            params: {
                end_date: str,
                adjust: str,
                symbol: str,
                start_date: str,
            },
        }
        define_op! {
            name: stock_a_gxl_lg,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_ipo_declare,
            params: {
            },
        }
        define_op! {
            name: news_trade_notify_dividend_baidu,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_zh_a_spot,
            params: {
            },
        }
        define_op! {
            name: stock_sh_a_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_new_ipo_cninfo,
            params: {
            },
        }
        define_op! {
            name: stock_sgt_settlement_exchange_rate_szse,
            params: {
            },
        }
        define_op! {
            name: stock_financial_debt_ths,
            params: {
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_zyjs_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_lhb_jgzz_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_board_concept_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_board_industry_summary_ths,
            params: {
            },
        }
        define_op! {
            name: stock_cy_a_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_info_sz_name_code,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hsgt_individual_detail_em,
            params: {
                start_date: str,
                end_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_sector_spot,
            params: {
                indicator: str,
            },
        }
        define_op! {
            name: stock_cg_guarantee_cninfo,
            params: {
                end_date: str,
                start_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_yysj_em,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_balance_sheet_by_report_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_gdfx_holding_detail_em,
            params: {
                date: str,
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_hot_deal_xq,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hot_keyword_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_inner_trade_xq,
            params: {
            },
        }
        define_op! {
            name: stock_mda_ym,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_share_hold_change_szse,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_a_ttm_lyr,
            params: {
            },
        }
        define_op! {
            name: stock_bid_ask_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_sns_sseinfo,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_gdfx_holding_statistics_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_hk_daily,
            params: {
                symbol: str,
                adjust: str,
            },
        }
        define_op! {
            name: stock_info_sh_delist,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_us_daily,
            params: {
                symbol: str,
                adjust: str,
            },
        }
        define_op! {
            name: stock_financial_abstract,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_add_stock,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_info_broker_sina,
            params: {
                page: str,
            },
        }
        define_op! {
            name: stock_rank_xzjp_ths,
            params: {
            },
        }
        define_op! {
            name: stock_intraday_sina,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_hot_rank_latest_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_valuation_baidu,
            params: {
                indicator: str,
                period: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_a_hist,
            params: {
                adjust: str,
                period: str,
                start_date: str,
                symbol: str,
                timeout: float,
                end_date: str,
            },
        }
        define_op! {
            name: stock_zh_a_minute,
            params: {
                adjust: str,
                symbol: str,
                period: str,
            },
        }
        define_op! {
            name: news_trade_notify_suspend_baidu,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_board_concept_cons_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_esg_rate_sina,
            params: {
            },
        }
        define_op! {
            name: stock_individual_fund_flow,
            params: {
                market: str,
                stock: str,
            },
        }
        define_op! {
            name: stock_share_hold_change_bse,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_lhb_jgmmtj_em,
            params: {
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_dzjy_mrtj,
            params: {
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_concept_fund_flow_hist,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_lhb_hyyyb_em,
            params: {
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: stock_register_sh,
            params: {
            },
        }
        define_op! {
            name: stock_gsrl_gsdt_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_yjbb_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_zh_a_gdhs,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hot_rank_wc,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_profit_sheet_by_report_delisted_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_history_dividend_detail,
            params: {
                indicator: str,
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_gdfx_free_holding_detail_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_hsgt_hist_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hk_hot_rank_detail_realtime_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_rank_forecast_cninfo,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_szse_sector_summary,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_board_industry_spot_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_account_statistics_em,
            params: {
            },
        }
        define_op! {
            name: stock_hsgt_board_rank_em,
            params: {
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_comment_detail_scrd_focus_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_info_sz_delist,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_us_famous_spot_em,
            params: {
            },
        }
        define_op! {
            name: stock_zh_a_hist_pre_min_em,
            params: {
                end_time: str,
                symbol: str,
                start_time: str,
            },
        }
        define_op! {
            name: stock_news_main_cx,
            params: {
            },
        }
        define_op! {
            name: stock_comment_detail_zhpj_lspf_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_industry_pe_ratio_cninfo,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_lhb_detail_em,
            params: {
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: stock_research_report_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_report_fund_hold,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_gdfx_top_10_em,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: stock_zh_a_gdhs_detail_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_dzjy_mrmx,
            params: {
                start_date: str,
                end_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: stock_hot_tweet_xq,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_hot_rank_detail_realtime_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_profit_sheet_by_report_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_profit_sheet_by_yearly_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_rank_ljqs_ths,
            params: {
            },
        }
        define_op! {
            name: stock_esg_rft_sina,
            params: {
            },
        }
        define_op! {
            name: stock_cash_flow_sheet_by_yearly_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_gdfx_holding_analyse_em,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_hk_hot_rank_latest_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_sy_profile_em,
            params: {
            },
        }
        define_op! {
            name: stock_fund_flow_individual,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_board_industry_name_em,
            params: {
            },
        }
        define_op! {
            name: stock_xgsr_ths,
            params: {
            },
        }
        define_op! {
            name: stock_cash_flow_sheet_by_report_delisted_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_rank_cxd_ths,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_institute_hold,
            params: {
                symbol: str,
            },
        }
    }
}
pub mod nlp {
    pub mod nlp {
        use akshare_macros::define_op;
        define_op! {
            name: nlp_answer,
            params: {
                question: str,
            },
        }
        define_op! {
            name: nlp_ownthink,
            params: {
                indicator: str,
                word: str,
            },
        }
    }
}
pub mod bank {
    pub mod bank {
        use akshare_macros::define_op;
        define_op! {
            name: bank_fjcf_table_detail,
            params: {
                item: int,
                begin: int,
                page: int,
            },
        }
    }
}
pub mod spot {
    pub mod spot {
        use akshare_macros::define_op;
        define_op! {
            name: spot_price_qh,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: spot_quotations_sge,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: spot_golden_benchmark_sge,
            params: {
            },
        }
        define_op! {
            name: spot_hog_lean_price_soozhu,
            params: {
            },
        }
        define_op! {
            name: spot_hog_crossbred_soozhu,
            params: {
            },
        }
        define_op! {
            name: spot_hist_sge,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: spot_corn_price_soozhu,
            params: {
            },
        }
        define_op! {
            name: spot_soybean_price_soozhu,
            params: {
            },
        }
        define_op! {
            name: spot_mixed_feed_soozhu,
            params: {
            },
        }
        define_op! {
            name: spot_hog_soozhu,
            params: {
            },
        }
        define_op! {
            name: spot_hog_year_trend_soozhu,
            params: {
            },
        }
        define_op! {
            name: spot_silver_benchmark_sge,
            params: {
            },
        }
    }
}
pub mod currency {
    pub mod currency {
        use akshare_macros::define_op;
        define_op! {
            name: currency_currencies,
            params: {
                c_type: str,
                api_key: str,
            },
        }
        define_op! {
            name: currency_history,
            params: {
                symbols: str,
                api_key: str,
                base: str,
                date: str,
            },
        }
        define_op! {
            name: currency_convert,
            params: {
                amount: str,
                to: str,
                base: str,
                api_key: str,
            },
        }
        define_op! {
            name: currency_latest,
            params: {
                base: str,
                symbols: str,
                api_key: str,
            },
        }
        define_op! {
            name: currency_time_series,
            params: {
                start_date: str,
                symbols: str,
                base: str,
                end_date: str,
                api_key: str,
            },
        }
    }
}
pub mod index {
    pub mod index {
        use akshare_macros::define_op;
        define_op! {
            name: index_neei_cx,
            params: {
            },
        }
        define_op! {
            name: index_ai_cx,
            params: {
            },
        }
        define_op! {
            name: index_realtime_sw,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_neaw_cx,
            params: {
            },
        }
        define_op! {
            name: index_option_100etf_qvix,
            params: {
            },
        }
        define_op! {
            name: stock_hk_index_daily_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_option_kcb_min_qvix,
            params: {
            },
        }
        define_op! {
            name: index_li_cx,
            params: {
            },
        }
        define_op! {
            name: index_ci_cx,
            params: {
            },
        }
        define_op! {
            name: index_component_sw,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_option_500etf_qvix,
            params: {
            },
        }
        define_op! {
            name: index_kq_fz,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_index_daily_em,
            params: {
                symbol: str,
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: index_yw,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_ii_cx,
            params: {
            },
        }
        define_op! {
            name: index_cci_cx,
            params: {
            },
        }
        define_op! {
            name: index_qli_cx,
            params: {
            },
        }
        define_op! {
            name: index_option_kcb_qvix,
            params: {
            },
        }
        define_op! {
            name: index_option_1000index_qvix,
            params: {
            },
        }
        define_op! {
            name: index_stock_cons_weight_csindex,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: sw_index_third_info,
            params: {
            },
        }
        define_op! {
            name: index_dei_cx,
            params: {
            },
        }
        define_op! {
            name: index_all_cni,
            params: {
            },
        }
        define_op! {
            name: index_volume_cflp,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_option_cyb_min_qvix,
            params: {
            },
        }
        define_op! {
            name: index_hist_fund_sw,
            params: {
                symbol: str,
                period: str,
            },
        }
        define_op! {
            name: sw_index_second_info,
            params: {
            },
        }
        define_op! {
            name: stock_zh_index_spot_sina,
            params: {
            },
        }
        define_op! {
            name: index_option_50index_min_qvix,
            params: {
            },
        }
        define_op! {
            name: index_stock_cons_csindex,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_min_sw,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: spot_goods,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_nei_cx,
            params: {
            },
        }
        define_op! {
            name: stock_hk_index_spot_sina,
            params: {
            },
        }
        define_op! {
            name: index_realtime_fund_sw,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_us_stock_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_bi_cx,
            params: {
            },
        }
        define_op! {
            name: stock_hk_index_daily_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_option_100etf_min_qvix,
            params: {
            },
        }
        define_op! {
            name: sw_index_third_cons,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: sw_index_first_info,
            params: {
            },
        }
        define_op! {
            name: index_pmi_com_cx,
            params: {
            },
        }
        define_op! {
            name: index_option_50etf_qvix,
            params: {
            },
        }
        define_op! {
            name: index_ti_cx,
            params: {
            },
        }
        define_op! {
            name: index_awpr_cx,
            params: {
            },
        }
        define_op! {
            name: index_analysis_weekly_sw,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: index_option_300index_min_qvix,
            params: {
            },
        }
        define_op! {
            name: index_zh_a_hist,
            params: {
                start_date: str,
                symbol: str,
                end_date: str,
                period: str,
            },
        }
        define_op! {
            name: stock_zh_index_daily,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_option_cyb_qvix,
            params: {
            },
        }
        define_op! {
            name: index_option_50index_qvix,
            params: {
            },
        }
        define_op! {
            name: index_analysis_daily_sw,
            params: {
                symbol: str,
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: index_bei_cx,
            params: {
            },
        }
        define_op! {
            name: index_option_300etf_min_qvix,
            params: {
            },
        }
        define_op! {
            name: stock_zh_index_hist_csindex,
            params: {
                start_date: str,
                symbol: str,
                end_date: str,
            },
        }
        define_op! {
            name: index_detail_cni,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: index_price_cflp,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_option_50etf_min_qvix,
            params: {
            },
        }
        define_op! {
            name: index_detail_hist_adjust_cni,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_sugar_msweet,
            params: {
            },
        }
        define_op! {
            name: index_analysis_monthly_sw,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: index_hist_sw,
            params: {
                period: str,
                symbol: str,
            },
        }
        define_op! {
            name: index_inner_quote_sugar_msweet,
            params: {
            },
        }
        define_op! {
            name: stock_hk_index_spot_em,
            params: {
            },
        }
        define_op! {
            name: drewry_wci_index,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_fi_cx,
            params: {
            },
        }
        define_op! {
            name: index_kq_fashion,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_eri,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: stock_zh_index_value_csindex,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_si_cx,
            params: {
            },
        }
        define_op! {
            name: index_news_sentiment_scope,
            params: {
            },
        }
        define_op! {
            name: index_stock_cons,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_hist_cni,
            params: {
                symbol: str,
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: index_pmi_man_cx,
            params: {
            },
        }
        define_op! {
            name: index_detail_hist_cni,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: index_option_1000index_min_qvix,
            params: {
            },
        }
        define_op! {
            name: index_option_300etf_qvix,
            params: {
            },
        }
        define_op! {
            name: index_option_500etf_min_qvix,
            params: {
            },
        }
        define_op! {
            name: index_outer_quote_sugar_msweet,
            params: {
            },
        }
        define_op! {
            name: stock_zh_index_spot_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: index_zh_a_hist_min_em,
            params: {
                end_date: str,
                symbol: str,
                period: str,
                start_date: str,
            },
        }
        define_op! {
            name: index_pmi_ser_cx,
            params: {
            },
        }
        define_op! {
            name: index_option_300index_qvix,
            params: {
            },
        }
    }
}
pub mod fx {
    pub mod fx {
        use akshare_macros::define_op;
        define_op! {
            name: macro_fx_sentiment,
            params: {
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: fx_swap_quote,
            params: {
            },
        }
        define_op! {
            name: currency_boc_sina,
            params: {
                start_date: str,
                end_date: str,
                symbol: str,
            },
        }
        define_op! {
            name: currency_boc_safe,
            params: {
            },
        }
        define_op! {
            name: fx_spot_quote,
            params: {
            },
        }
        define_op! {
            name: fx_pair_quote,
            params: {
            },
        }
        define_op! {
            name: currency_pair_map,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fx_quote_baidu,
            params: {
                symbol: str,
            },
        }
    }
}
pub mod r#macro {
    pub mod r#macro {
        use akshare_macros::define_op;
        define_op! {
            name: macro_canada_core_cpi_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_euro_industrial_production_mom,
            params: {
            },
        }
        define_op! {
            name: macro_china_pmi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_cftc_merchant_goods_holding,
            params: {
            },
        }
        define_op! {
            name: macro_canada_new_house_rate,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_market_info,
            params: {
            },
        }
        define_op! {
            name: macro_uk_trade,
            params: {
            },
        }
        define_op! {
            name: macro_china_lpi_index,
            params: {
            },
        }
        define_op! {
            name: macro_swiss_gdp_quarterly,
            params: {
            },
        }
        define_op! {
            name: macro_uk_retail_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_lmci,
            params: {
            },
        }
        define_op! {
            name: macro_usa_phs,
            params: {
            },
        }
        define_op! {
            name: macro_usa_durable_goods_orders,
            params: {
            },
        }
        define_op! {
            name: macro_euro_gdp_yoy,
            params: {
            },
        }
        define_op! {
            name: macro_china_qyspjg,
            params: {
            },
        }
        define_op! {
            name: macro_china_lpr,
            params: {
            },
        }
        define_op! {
            name: macro_china_gdp,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_ppi,
            params: {
            },
        }
        define_op! {
            name: macro_china_gdp_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_japan_bank_rate,
            params: {
            },
        }
        define_op! {
            name: macro_usa_initial_jobless,
            params: {
            },
        }
        define_op! {
            name: macro_germany_zew,
            params: {
            },
        }
        define_op! {
            name: macro_euro_trade_balance,
            params: {
            },
        }
        define_op! {
            name: macro_uk_rightmove_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_australia_ppi_quarterly,
            params: {
            },
        }
        define_op! {
            name: macro_china_retail_price_index,
            params: {
            },
        }
        define_op! {
            name: macro_cnbs,
            params: {
            },
        }
        define_op! {
            name: macro_cons_silver,
            params: {
            },
        }
        define_op! {
            name: macro_australia_bank_rate,
            params: {
            },
        }
        define_op! {
            name: macro_usa_crude_inner,
            params: {
            },
        }
        define_op! {
            name: macro_china_bond_public,
            params: {
            },
        }
        define_op! {
            name: macro_china_new_house_price,
            params: {
                city_first: str,
                city_second: str,
            },
        }
        define_op! {
            name: macro_euro_cpi_yoy,
            params: {
            },
        }
        define_op! {
            name: macro_australia_retail_rate_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_non_farm,
            params: {
            },
        }
        define_op! {
            name: macro_china_cpi,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_gbp,
            params: {
            },
        }
        define_op! {
            name: macro_usa_current_account,
            params: {
            },
        }
        define_op! {
            name: macro_china_pmi,
            params: {
            },
        }
        define_op! {
            name: macro_usa_exist_home_sales,
            params: {
            },
        }
        define_op! {
            name: macro_uk_halifax_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_cftc_merchant_currency_holding,
            params: {
            },
        }
        define_op! {
            name: macro_usa_core_cpi_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_china_wbck,
            params: {
            },
        }
        define_op! {
            name: macro_euro_ppi_mom,
            params: {
            },
        }
        define_op! {
            name: macro_china_market_margin_sz,
            params: {
            },
        }
        define_op! {
            name: macro_swiss_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_building_volume,
            params: {
            },
        }
        define_op! {
            name: macro_uk_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_germany_gdp,
            params: {
            },
        }
        define_op! {
            name: macro_china_supply_of_money,
            params: {
            },
        }
        define_op! {
            name: macro_canada_core_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_canada_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_china_bank_financing,
            params: {
            },
        }
        define_op! {
            name: macro_china_daily_energy,
            params: {
            },
        }
        define_op! {
            name: macro_uk_core_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_euro_lme_holding,
            params: {
            },
        }
        define_op! {
            name: macro_usa_adp_employment,
            params: {
            },
        }
        define_op! {
            name: macro_china_ppi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_china_yw_electronic_index,
            params: {
            },
        }
        define_op! {
            name: macro_germany_trade_adjusted,
            params: {
            },
        }
        define_op! {
            name: macro_china_national_tax_receipts,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_gbp_ratio,
            params: {
            },
        }
        define_op! {
            name: macro_usa_nfib_small_business,
            params: {
            },
        }
        define_op! {
            name: macro_swiss_gbd_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_china_exports_yoy,
            params: {
            },
        }
        define_op! {
            name: macro_japan_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_uk_cpi_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_cons_opec_month,
            params: {
            },
        }
        define_op! {
            name: macro_euro_unemployment_rate_mom,
            params: {
            },
        }
        define_op! {
            name: macro_china_central_bank_balance,
            params: {
            },
        }
        define_op! {
            name: macro_canada_unemployment_rate,
            params: {
            },
        }
        define_op! {
            name: macro_canada_retail_rate_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_info_ws,
            params: {
                date: str,
            },
        }
        define_op! {
            name: news_economic_baidu,
            params: {
                date: str,
            },
        }
        define_op! {
            name: macro_china_new_financial_credit,
            params: {
            },
        }
        define_op! {
            name: macro_usa_personal_spending,
            params: {
            },
        }
        define_op! {
            name: macro_china_real_estate,
            params: {
            },
        }
        define_op! {
            name: macro_china_consumer_goods_retail,
            params: {
            },
        }
        define_op! {
            name: macro_china_insurance_income,
            params: {
            },
        }
        define_op! {
            name: macro_china_czsr,
            params: {
            },
        }
        define_op! {
            name: macro_rmb_deposit,
            params: {
            },
        }
        define_op! {
            name: macro_japan_head_indicator,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_trade_diff_ratio,
            params: {
            },
        }
        define_op! {
            name: macro_usa_industrial_production,
            params: {
            },
        }
        define_op! {
            name: macro_usa_michigan_consumer_sentiment,
            params: {
            },
        }
        define_op! {
            name: macro_euro_manufacturing_pmi,
            params: {
            },
        }
        define_op! {
            name: macro_china_m2_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_swiss_svme,
            params: {
            },
        }
        define_op! {
            name: macro_uk_unemployment_rate,
            params: {
            },
        }
        define_op! {
            name: macro_china_non_man_pmi,
            params: {
            },
        }
        define_op! {
            name: macro_china_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_china_nbs_nation,
            params: {
                kind: str,
                path: str,
                period: str,
            },
        }
        define_op! {
            name: macro_uk_gdp_quarterly,
            params: {
            },
        }
        define_op! {
            name: macro_euro_retail_sales_mom,
            params: {
            },
        }
        define_op! {
            name: macro_china_au_report,
            params: {
            },
        }
        define_op! {
            name: macro_china_enterprise_boom_index,
            params: {
            },
        }
        define_op! {
            name: macro_china_construction_index,
            params: {
            },
        }
        define_op! {
            name: macro_china_reserve_requirement_ratio,
            params: {
            },
        }
        define_op! {
            name: macro_euro_zew_economic_sentiment,
            params: {
            },
        }
        define_op! {
            name: macro_euro_sentix_investor_confidence,
            params: {
            },
        }
        define_op! {
            name: macro_usa_house_starts,
            params: {
            },
        }
        define_op! {
            name: macro_usa_import_price,
            params: {
            },
        }
        define_op! {
            name: macro_usa_ism_non_pmi,
            params: {
            },
        }
        define_op! {
            name: macro_china_hgjck,
            params: {
            },
        }
        define_op! {
            name: macro_usa_gdp_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_china_fdi,
            params: {
            },
        }
        define_op! {
            name: macro_uk_bank_rate,
            params: {
            },
        }
        define_op! {
            name: macro_australia_trade,
            params: {
            },
        }
        define_op! {
            name: macro_australia_unemployment_rate,
            params: {
            },
        }
        define_op! {
            name: macro_euro_lme_stock,
            params: {
            },
        }
        define_op! {
            name: macro_china_cx_services_pmi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_germany_retail_sale_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_shipping_bcti,
            params: {
            },
        }
        define_op! {
            name: macro_usa_pending_home_sales,
            params: {
            },
        }
        define_op! {
            name: macro_china_gyzjz,
            params: {
            },
        }
        define_op! {
            name: macro_china_postal_telecommunicational,
            params: {
            },
        }
        define_op! {
            name: macro_china_fx_reserves_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_china_urban_unemployment,
            params: {
            },
        }
        define_op! {
            name: macro_china_society_traffic_volume,
            params: {
            },
        }
        define_op! {
            name: macro_china_foreign_exchange_gold,
            params: {
            },
        }
        define_op! {
            name: macro_usa_ppi,
            params: {
            },
        }
        define_op! {
            name: macro_usa_core_ppi,
            params: {
            },
        }
        define_op! {
            name: macro_swiss_trade,
            params: {
            },
        }
        define_op! {
            name: macro_usa_trade_balance,
            params: {
            },
        }
        define_op! {
            name: macro_japan_core_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_cb_consumer_confidence,
            params: {
            },
        }
        define_op! {
            name: macro_euro_services_pmi,
            params: {
            },
        }
        define_op! {
            name: macro_swiss_gbd_bank_rate,
            params: {
            },
        }
        define_op! {
            name: macro_uk_rightmove_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_cpi_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_china_bsi_index,
            params: {
            },
        }
        define_op! {
            name: macro_china_freight_index,
            params: {
            },
        }
        define_op! {
            name: macro_china_rmb,
            params: {
            },
        }
        define_op! {
            name: macro_china_imports_yoy,
            params: {
            },
        }
        define_op! {
            name: macro_china_nbs_region,
            params: {
                indicator: str,
                region: str,
                path: str,
                kind: str,
                period: str,
            },
        }
        define_op! {
            name: macro_china_hk_cpi_ratio,
            params: {
            },
        }
        define_op! {
            name: macro_china_cpi_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_china_passenger_load_factor,
            params: {
            },
        }
        define_op! {
            name: macro_usa_factory_orders,
            params: {
            },
        }
        define_op! {
            name: macro_china_shibor_all,
            params: {
            },
        }
        define_op! {
            name: macro_usa_pmi,
            params: {
            },
        }
        define_op! {
            name: macro_china_trade_balance,
            params: {
            },
        }
        define_op! {
            name: macro_usa_cpi_yoy,
            params: {
            },
        }
        define_op! {
            name: macro_australia_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_core_pce_price,
            params: {
            },
        }
        define_op! {
            name: macro_usa_cme_merchant_goods_holding,
            params: {
            },
        }
        define_op! {
            name: macro_china_fx_gold,
            params: {
            },
        }
        define_op! {
            name: macro_japan_unemployment_rate,
            params: {
            },
        }
        define_op! {
            name: macro_australia_cpi_quarterly,
            params: {
            },
        }
        define_op! {
            name: macro_china_whxd,
            params: {
            },
        }
        define_op! {
            name: macro_usa_rig_count,
            params: {
            },
        }
        define_op! {
            name: macro_usa_spcs20,
            params: {
            },
        }
        define_op! {
            name: macro_canada_trade,
            params: {
            },
        }
        define_op! {
            name: macro_china_money_supply,
            params: {
            },
        }
        define_op! {
            name: macro_shipping_bdi,
            params: {
            },
        }
        define_op! {
            name: macro_stock_finance,
            params: {
            },
        }
        define_op! {
            name: macro_china_society_electricity,
            params: {
            },
        }
        define_op! {
            name: macro_china_insurance,
            params: {
            },
        }
        define_op! {
            name: macro_china_international_tourism_fx,
            params: {
            },
        }
        define_op! {
            name: macro_shipping_bpi,
            params: {
            },
        }
        define_op! {
            name: macro_china_shrzgm,
            params: {
            },
        }
        define_op! {
            name: macro_usa_retail_sales,
            params: {
            },
        }
        define_op! {
            name: macro_china_industrial_production_yoy,
            params: {
            },
        }
        define_op! {
            name: macro_usa_ism_pmi,
            params: {
            },
        }
        define_op! {
            name: macro_usa_business_inventories,
            params: {
            },
        }
        define_op! {
            name: macro_usa_new_home_sales,
            params: {
            },
        }
        define_op! {
            name: macro_germany_ifo,
            params: {
            },
        }
        define_op! {
            name: macro_usa_eia_crude_rate,
            params: {
            },
        }
        define_op! {
            name: macro_canada_bank_rate,
            params: {
            },
        }
        define_op! {
            name: macro_usa_cftc_nc_holding,
            params: {
            },
        }
        define_op! {
            name: macro_china_commodity_price_index,
            params: {
            },
        }
        define_op! {
            name: macro_china_mobile_number,
            params: {
            },
        }
        define_op! {
            name: macro_china_gdzctz,
            params: {
            },
        }
        define_op! {
            name: macro_usa_unemployment_rate,
            params: {
            },
        }
        define_op! {
            name: macro_cons_gold,
            params: {
            },
        }
        define_op! {
            name: macro_usa_cftc_c_holding,
            params: {
            },
        }
        define_op! {
            name: macro_china_xfzxx,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_building_amount,
            params: {
            },
        }
        define_op! {
            name: macro_usa_nahb_house_market_index,
            params: {
            },
        }
        define_op! {
            name: macro_china_energy_index,
            params: {
            },
        }
        define_op! {
            name: macro_usa_house_price_index,
            params: {
            },
        }
        define_op! {
            name: macro_euro_cpi_mom,
            params: {
            },
        }
        define_op! {
            name: macro_germany_cpi_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_services_pmi,
            params: {
            },
        }
        define_op! {
            name: macro_uk_halifax_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_china_stock_market_cap,
            params: {
            },
        }
        define_op! {
            name: macro_china_agricultural_product,
            params: {
            },
        }
        define_op! {
            name: macro_china_swap_rate,
            params: {
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: macro_china_market_margin_sh,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_cpi,
            params: {
            },
        }
        define_op! {
            name: macro_rmb_loan,
            params: {
            },
        }
        define_op! {
            name: macro_china_bdti_index,
            params: {
            },
        }
        define_op! {
            name: macro_china_hk_rate_of_unemployment,
            params: {
            },
        }
        define_op! {
            name: macro_china_ppi,
            params: {
            },
        }
        define_op! {
            name: macro_euro_current_account_mom,
            params: {
            },
        }
        define_op! {
            name: macro_germany_cpi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_uk_gdp_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_china_cx_pmi_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_global_sox_index,
            params: {
            },
        }
        define_op! {
            name: macro_germany_retail_sale_yearly,
            params: {
            },
        }
        define_op! {
            name: macro_uk_core_cpi_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_uk_retail_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_canada_cpi_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_china_vegetable_basket,
            params: {
            },
        }
        define_op! {
            name: macro_china_agricultural_index,
            params: {
            },
        }
        define_op! {
            name: macro_usa_api_crude_stock,
            params: {
            },
        }
        define_op! {
            name: macro_shipping_bci,
            params: {
            },
        }
        define_op! {
            name: macro_china_construction_price_index,
            params: {
            },
        }
        define_op! {
            name: macro_usa_real_consumer_spending,
            params: {
            },
        }
        define_op! {
            name: macro_euro_employment_change_qoq,
            params: {
            },
        }
        define_op! {
            name: macro_canada_gdp_monthly,
            params: {
            },
        }
        define_op! {
            name: macro_usa_export_price,
            params: {
            },
        }
        define_op! {
            name: macro_usa_job_cuts,
            params: {
            },
        }
        define_op! {
            name: macro_usa_building_permits,
            params: {
            },
        }
    }
}
pub mod qhkc {
    pub mod fund {
        use akshare_macros::define_op;
        define_op! {
            name: commodity_flow_long,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_flow_short,
            params: {
                date: str,
            },
        }
        define_op! {
            name: stock_flow_long,
            params: {
                date: str,
            },
        }
        define_op! {
            name: money_in_out,
            params: {
                date: str,
            },
        }
        define_op! {
            name: commodity_flow_short,
            params: {
                date: str,
            },
        }
    }
    pub mod commodity {
        use akshare_macros::define_op;
        define_op! {
            name: variety_quotes,
            params: {
                date: str,
                code: str,
            },
        }
        define_op! {
            name: variety_total_money,
            params: {
                date: str,
                code: str,
            },
        }
        define_op! {
            name: variety_no_futures,
            params: {
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: variety_longhu_top,
            params: {
                date: str,
            },
        }
        define_op! {
            name: variety_all,
            params: {
                date: str,
                csymbolode: str,
            },
        }
        define_op! {
            name: variety_net_money_chge,
            params: {
                code: str,
                date: str,
            },
        }
        define_op! {
            name: variety_net_positions,
            params: {
                broker: str,
                symbol: str,
                date: str,
            },
        }
        define_op! {
            name: variety_net_money,
            params: {
                date: str,
                code: str,
            },
        }
        define_op! {
            name: variety_bbr,
            params: {
                date: str,
                code: str,
            },
        }
        define_op! {
            name: variety_profit,
            params: {
                symbol: str,
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: variety_niuxiong_top,
            params: {
                date: str,
            },
        }
        define_op! {
            name: variety_reports,
            params: {
                date: str,
                csymbolode: str,
            },
        }
        define_op! {
            name: variety_list,
            params: {
                date: str,
            },
        }
        define_op! {
            name: variety_positions,
            params: {
                date: str,
                code: str,
                fields: str,
            },
        }
        define_op! {
            name: variety_strategies,
            params: {
                code: str,
                date: str,
            },
        }
        define_op! {
            name: variety_all_positions,
            params: {
                fields: str,
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: variety_money,
            params: {
                date: str,
                symbol: str,
            },
        }
    }
    pub mod index_data {
        use akshare_macros::define_op;
        define_op! {
            name: index_money,
            params: {
                date: str,
                index_id: str,
            },
        }
        define_op! {
            name: index_profit,
            params: {
                start_date: str,
                index_id: str,
                end_date: str,
            },
        }
        define_op! {
            name: index_official,
            params: {
            },
        }
        define_op! {
            name: index_weights,
            params: {
                date: str,
                index_id: str,
            },
        }
        define_op! {
            name: index_trend,
            params: {
                index_id: str,
                date: str,
            },
        }
        define_op! {
            name: index_mine,
            params: {
            },
        }
        define_op! {
            name: index_quotes,
            params: {
                date: str,
                index_id: str,
            },
        }
        define_op! {
            name: index_info,
            params: {
                index_id: str,
            },
        }
    }
    pub mod broker {
        use akshare_macros::define_op;
        define_op! {
            name: broker_flow,
            params: {
                broker: str,
                date: str,
                offset: str,
            },
        }
        define_op! {
            name: broker_total_money,
            params: {
                date: str,
                broker: str,
            },
        }
        define_op! {
            name: broker_profit,
            params: {
                start_date: str,
                end_date: str,
                broker: str,
            },
        }
        define_op! {
            name: broker_all,
            params: {
                offset_days: str,
            },
        }
        define_op! {
            name: broker_in_profit_list,
            params: {
                count: str,
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: broker_positions,
            params: {
                broker: str,
                date: str,
            },
        }
        define_op! {
            name: broker_pk,
            params: {
                symbol: str,
                broker1: str,
                broker2: str,
            },
        }
        define_op! {
            name: broker_net_money,
            params: {
                date: str,
                broker: str,
            },
        }
        define_op! {
            name: broker_in_loss_list,
            params: {
                count: str,
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: broker_bbr,
            params: {
                broker: str,
                date: str,
            },
        }
        define_op! {
            name: broker_net_money_chge,
            params: {
                date: str,
                broker: str,
            },
        }
        define_op! {
            name: broker_positions_process,
            params: {
                broker: str,
                end_date: str,
                start_date: str,
                code: str,
            },
        }
        define_op! {
            name: broker_calendar,
            params: {
                broker: str,
                start_date: str,
                end_date: str,
            },
        }
    }
    pub mod tools {
        use akshare_macros::define_op;
        define_op! {
            name: long_pool,
            params: {
                date: str,
            },
        }
        define_op! {
            name: short_pool,
            params: {
                date: str,
            },
        }
    }
    pub mod fundamental {
        use akshare_macros::define_op;
        define_op! {
            name: free_spread,
            params: {
                code1: str,
                date: str,
                variety1: str,
                variety2: str,
                code2: str,
            },
        }
        define_op! {
            name: virtual_real,
            params: {
                code: str,
                variety: str,
                date: str,
            },
        }
        define_op! {
            name: warehouse_receipt,
            params: {
                date: str,
            },
        }
        define_op! {
            name: term_structure,
            params: {
                date: str,
                variety: str,
            },
        }
        define_op! {
            name: trader_prices,
            params: {
                date: str,
                variety: str,
            },
        }
        define_op! {
            name: inventory,
            params: {
                year: str,
                variety: str,
                week_number: str,
            },
        }
        define_op! {
            name: intertemporal_arbitrage,
            params: {
                code2: str,
                date: str,
                variety: str,
                code1: str,
            },
        }
        define_op! {
            name: free_ratio,
            params: {
                code2: str,
                variety1: str,
                variety2: str,
                date: str,
                code1: str,
            },
        }
        define_op! {
            name: basis,
            params: {
                variety: str,
                date: str,
            },
        }
        define_op! {
            name: profit,
            params: {
                variety: str,
                date: str,
            },
        }
    }
}
pub mod fund {
    pub mod fund_private {
        use akshare_macros::define_op;
        define_op! {
            name: amac_fund_account_info,
            params: {
            },
        }
        define_op! {
            name: amac_person_bond_org_list,
            params: {
            },
        }
        define_op! {
            name: amac_aoin_info,
            params: {
            },
        }
        define_op! {
            name: amac_manager_classify_info,
            params: {
            },
        }
        define_op! {
            name: amac_fund_info,
            params: {
                start_page: str,
                end_page: str,
            },
        }
        define_op! {
            name: amac_person_fund_org_list,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: amac_fund_sub_info,
            params: {
            },
        }
        define_op! {
            name: amac_fund_abs,
            params: {
            },
        }
        define_op! {
            name: amac_member_sub_info,
            params: {
            },
        }
        define_op! {
            name: amac_manager_info,
            params: {
            },
        }
        define_op! {
            name: amac_futures_info,
            params: {
            },
        }
        define_op! {
            name: amac_manager_cancelled_info,
            params: {
            },
        }
        define_op! {
            name: amac_securities_info,
            params: {
            },
        }
        define_op! {
            name: amac_member_info,
            params: {
            },
        }
    }
    pub mod fund_public {
        use akshare_macros::define_op;
        define_op! {
            name: reits_realtime_em,
            params: {
            },
        }
        define_op! {
            name: fund_open_fund_info_em,
            params: {
                symbol: str,
                period: str,
                indicator: str,
            },
        }
        define_op! {
            name: fund_hk_rank_em,
            params: {
            },
        }
        define_op! {
            name: fund_etf_hist_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_purchase_em,
            params: {
            },
        }
        define_op! {
            name: fund_financial_fund_daily_em,
            params: {
            },
        }
        define_op! {
            name: fund_individual_detail_hold_xq,
            params: {
                timeout: float,
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: fund_scale_open_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_info_index_em,
            params: {
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: fund_aum_trend_em,
            params: {
            },
        }
        define_op! {
            name: fund_etf_dividend_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_individual_detail_info_xq,
            params: {
                timeout: float,
                symbol: str,
            },
        }
        define_op! {
            name: fund_aum_em,
            params: {
            },
        }
        define_op! {
            name: fund_fee_em,
            params: {
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: fund_report_stock_cninfo,
            params: {
                date: str,
            },
        }
        define_op! {
            name: fund_aum_hist_em,
            params: {
                year: str,
            },
        }
        define_op! {
            name: fund_rating_ja,
            params: {
                date: str,
            },
        }
        define_op! {
            name: fund_lof_hist_em,
            params: {
                adjust: str,
                end_date: str,
                start_date: str,
                symbol: str,
                period: str,
            },
        }
        define_op! {
            name: fund_scale_change_em,
            params: {
            },
        }
        define_op! {
            name: fund_cf_em,
            params: {
            },
        }
        define_op! {
            name: fund_lof_hist_min_em,
            params: {
                period: str,
                symbol: str,
                start_date: str,
                end_date: str,
                adjust: str,
            },
        }
        define_op! {
            name: fund_new_found_em,
            params: {
            },
        }
        define_op! {
            name: fund_etf_fund_daily_em,
            params: {
            },
        }
        define_op! {
            name: fund_graded_fund_info_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_manager_em,
            params: {
            },
        }
        define_op! {
            name: fund_rating_sh,
            params: {
                date: str,
            },
        }
        define_op! {
            name: fund_scale_structured_sina,
            params: {
            },
        }
        define_op! {
            name: fund_money_fund_info_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_lcx_rank_em,
            params: {
            },
        }
        define_op! {
            name: fund_financial_fund_info_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_rating_all,
            params: {
            },
        }
        define_op! {
            name: fund_report_asset_allocation_cninfo,
            params: {
            },
        }
        define_op! {
            name: fund_etf_spot_em,
            params: {
            },
        }
        define_op! {
            name: fund_stock_position_lg,
            params: {
            },
        }
        define_op! {
            name: fund_etf_spot_ths,
            params: {
                date: str,
            },
        }
        define_op! {
            name: fund_etf_category_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_balance_position_lg,
            params: {
            },
        }
        define_op! {
            name: fund_graded_fund_daily_em,
            params: {
            },
        }
        define_op! {
            name: fund_fh_em,
            params: {
            },
        }
        define_op! {
            name: fund_exchange_rank_em,
            params: {
            },
        }
        define_op! {
            name: fund_lof_spot_em,
            params: {
            },
        }
        define_op! {
            name: fund_etf_fund_info_em,
            params: {
                fund: str,
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: fund_money_rank_em,
            params: {
            },
        }
        define_op! {
            name: fund_name_em,
            params: {
            },
        }
        define_op! {
            name: fund_scale_close_sina,
            params: {
            },
        }
        define_op! {
            name: fund_value_estimation_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_open_fund_rank_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_portfolio_change_em,
            params: {
                date: str,
                indicator: str,
                symbol: str,
            },
        }
        define_op! {
            name: fund_hold_structure_em,
            params: {
            },
        }
        define_op! {
            name: fund_open_fund_daily_em,
            params: {
            },
        }
        define_op! {
            name: fund_fh_rank_em,
            params: {
            },
        }
        define_op! {
            name: fund_linghuo_position_lg,
            params: {
            },
        }
        define_op! {
            name: fund_rating_zs,
            params: {
                date: str,
            },
        }
        define_op! {
            name: fund_report_industry_allocation_cninfo,
            params: {
                date: str,
            },
        }
        define_op! {
            name: fund_etf_hist_em,
            params: {
                symbol: str,
                adjust: str,
                period: str,
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: fund_portfolio_hold_em,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: fund_portfolio_bond_hold_em,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: fund_individual_profit_probability_xq,
            params: {
                symbol: str,
                timeout: float,
            },
        }
        define_op! {
            name: fund_individual_basic_info_xq,
            params: {
                timeout: float,
                symbol: str,
            },
        }
        define_op! {
            name: fund_individual_achievement_xq,
            params: {
                symbol: str,
                timeout: float,
            },
        }
        define_op! {
            name: fund_hk_fund_hist_em,
            params: {
                symbol: str,
                code: str,
            },
        }
        define_op! {
            name: fund_money_fund_daily_em,
            params: {
            },
        }
        define_op! {
            name: fund_portfolio_industry_allocation_em,
            params: {
                date: str,
                symbol: str,
            },
        }
        define_op! {
            name: fund_individual_analysis_xq,
            params: {
                timeout: float,
                symbol: str,
            },
        }
        define_op! {
            name: fund_announcement_personnel_em,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: fund_etf_hist_min_em,
            params: {
                end_date: str,
                start_date: str,
                adjust: str,
                symbol: str,
                period: str,
            },
        }
    }
}
pub mod bond {
    pub mod bond {
        use akshare_macros::define_op;
        define_op! {
            name: bond_deal_summary_sse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: bond_cov_stock_issue_cninfo,
            params: {
            },
        }
        define_op! {
            name: bond_corporate_issue_cninfo,
            params: {
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: bond_cb_summary_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: bond_zh_cov_value_analysis,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: bond_new_composite_index_cbond,
            params: {
                indicator: str,
                period: str,
            },
        }
        define_op! {
            name: bond_zh_hs_cov_daily,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: bond_cov_comparison,
            params: {
            },
        }
        define_op! {
            name: bond_cb_jsl,
            params: {
                cookie: str,
            },
        }
        define_op! {
            name: bond_cb_adj_logs_jsl,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: bond_zh_us_rate,
            params: {
                start_date: str,
            },
        }
        define_op! {
            name: bond_zh_hs_cov_spot,
            params: {
            },
        }
        define_op! {
            name: bond_zh_cov,
            params: {
            },
        }
        define_op! {
            name: bond_zh_hs_cov_pre_min,
            params: {
            },
        }
        define_op! {
            name: bond_zh_hs_daily,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: bond_spot_deal,
            params: {
            },
        }
        define_op! {
            name: bond_spot_quote,
            params: {
            },
        }
        define_op! {
            name: bond_info_detail_cm,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: bond_zh_hs_spot,
            params: {
                end_page: str,
                start_page: str,
            },
        }
        define_op! {
            name: bond_zh_hs_cov_min,
            params: {
                period: str,
                end_date: str,
                symbol: str,
                start_date: str,
                adjust: str,
            },
        }
        define_op! {
            name: bond_cash_summary_sse,
            params: {
                date: str,
            },
        }
        define_op! {
            name: bond_zh_cov_info,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: bond_china_close_return,
            params: {
                symbol: str,
                period: str,
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: bond_treasure_issue_cninfo,
            params: {
                start_date: str,
                end_date: str,
            },
        }
        define_op! {
            name: bond_cov_issue_cninfo,
            params: {
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: bond_composite_index_cbond,
            params: {
                indicator: str,
                period: str,
            },
        }
        define_op! {
            name: bond_cb_profile_sina,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: bond_china_yield,
            params: {
                end_date: str,
                start_date: str,
            },
        }
        define_op! {
            name: bond_cb_redeem_jsl,
            params: {
            },
        }
        define_op! {
            name: bond_cb_index_jsl,
            params: {
            },
        }
        define_op! {
            name: bond_zh_cov_info_ths,
            params: {
            },
        }
        define_op! {
            name: bond_info_cm,
            params: {
                underwriter: str,
                issue_year: str,
                bond_name: str,
                bond_issue: str,
                bond_type: str,
                grade: str,
                bond_code: str,
                coupon_type: str,
            },
        }
        define_op! {
            name: bond_debt_nafmii,
            params: {
                page: str,
            },
        }
        define_op! {
            name: bond_local_government_issue_cninfo,
            params: {
                start_date: str,
                end_date: str,
            },
        }
    }
}
pub mod energy {
    pub mod energy {
        use akshare_macros::define_op;
        define_op! {
            name: energy_carbon_hb,
            params: {
            },
        }
        define_op! {
            name: energy_carbon_domestic,
            params: {
                symbol: str,
            },
        }
        define_op! {
            name: energy_carbon_sz,
            params: {
            },
        }
        define_op! {
            name: energy_oil_hist,
            params: {
            },
        }
        define_op! {
            name: energy_oil_detail,
            params: {
                date: str,
            },
        }
        define_op! {
            name: energy_carbon_bj,
            params: {
            },
        }
        define_op! {
            name: energy_carbon_eu,
            params: {
            },
        }
        define_op! {
            name: energy_carbon_gz,
            params: {
            },
        }
    }
}
pub mod hf {
    pub mod hf {
        use akshare_macros::define_op;
        define_op! {
            name: hf_sp_500,
            params: {
                year: str,
            },
        }
    }
}

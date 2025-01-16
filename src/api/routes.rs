use actix_web::{web};
use crate::api::add_wallets_to_wl::add_wallets_to_wl;
use crate::api::auth_api::{get_session, login, logout, request_nonce};
use crate::api::check_wallet_wl::check_wallet;
use crate::api::create_community::create_community;
use crate::api::generate_txn::generate_transaction;
use crate::api::get_community_data::fetch_community_info;
use crate::api::get_members::fetch_all_community_members;
use crate::api::get_memberships::get_memberships;
use crate::api::get_ownerships::get_ownerships;
use crate::api::get_wallets::fetch_all_allowed_wallets;
use crate::api::process_payment::process_payment;
use crate::api::remove_wallet_from_wl::remove_wallet_from_wl;
use crate::api::update_community_data::update_community_data;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login)
            .service(request_nonce)
            .service(get_session)
            .service(get_ownerships)
            .service(get_memberships)
            .service(logout)
            .service(create_community)
            .service(fetch_community_info)
            .service(check_wallet)
            .service(fetch_all_community_members)
            .service(fetch_all_allowed_wallets)
            .service(add_wallets_to_wl)
            .service(generate_transaction)
            .service(update_community_data)
            .service(remove_wallet_from_wl)
            .service(process_payment)
    );
}
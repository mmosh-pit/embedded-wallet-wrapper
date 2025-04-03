pub mod api_req_model;
pub mod api_res_model;

use actix_web::{get, post, web, HttpResponse, Responder};
use api_req_model::{RecoverRequestModel, SignRequestModel, VerifyRequestModel};
use api_res_model::{CreateWalletResponse, ResponseModel};

use crate::{frost::{frost_create::frost_create_wallet, frost_model::{CreateWalletModel, RecoveryModel, SignModel, VerifyModel}, frost_sign::frost_create_signature, frost_verify::frost_verify_signature}, rocksdb::RocksDbPool};

#[get("/")]
async fn init() -> impl Responder {
    HttpResponse::Ok().body("Welcome to wallet wrapper")
}


#[post("/create")]
async fn create_wallet(pool: web::Data<RocksDbPool>) -> impl Responder {
    let result = frost_create_wallet().unwrap();
    let conn = pool.get().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();
    let _ = conn.put(result[2].clone().address.as_bytes(), serde_json::to_vec(&result[2]).unwrap());

    let mut data = Vec::new();
    for i in 0..2 {
        data.push(result[i].clone().key_package);
    }
   
    HttpResponse::Ok().body(serde_json::to_string(&ResponseModel{
        status: true,
        message: "wallet created successfully".to_string(),
        data: Some(serde_json::to_string(&CreateWalletResponse{
            key_package: data,
            address: result[0].clone().address
        }).unwrap())
    }).unwrap())
}

#[post("/sign")]
async fn sign_message(pool: web::Data<RocksDbPool>, body: web::Json<SignRequestModel>) -> impl Responder {
    let request = body.clone();

    let conn = pool.get().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();
    let keys = conn.get(request.address.as_bytes()).unwrap();

    let mut data = None;
    if keys.is_some() {
        let key_data = serde_json::from_slice::<CreateWalletModel>(&keys.unwrap()) .unwrap();
        let sign_obj = SignModel {
            message: request.clone().message,
            public_key_package: key_data.clone().public_key_package,
            key_package: vec![key_data.key_package,request.clone().key]
        };

        let signature = frost_create_signature(sign_obj).unwrap();
        data = Some(signature);
    }


    HttpResponse::Ok().body(serde_json::to_string(&ResponseModel{
        status: true,
        message: "message signed successfully".to_string(),
        data
    }).unwrap())
}

#[post("/recover")]
async fn recover(pool: web::Data<RocksDbPool>, body: web::Json<RecoverRequestModel>) -> impl Responder {
    let request = body.clone();
    let conn = pool.get().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();
    let keys = conn.get(request.address.as_bytes()).unwrap();


    if keys.is_some() {
        let key_data = serde_json::from_slice::<CreateWalletModel>(&keys.unwrap()) .unwrap();
        let recover_obj = RecoveryModel {
            public_key_package: key_data.clone().public_key_package,
            key_package: vec![key_data.key_package,request.clone().key]
        };
    }
    HttpResponse::Ok().body(serde_json::to_string(&ResponseModel{
        status: true,
        message: "wallet recovery successfull".to_string(),
        data: None
    }).unwrap())
}

#[post("/verify")]
async fn verify(pool: web::Data<RocksDbPool>, body: web::Json<VerifyRequestModel>) -> impl Responder {
    let request = body.clone();

    let conn = pool.get().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();
    let keys = conn.get(request.address.as_bytes()).unwrap();

    let mut is_valid = false;
    if keys.is_some() {
        let key_data = serde_json::from_slice::<CreateWalletModel>(&keys.unwrap()) .unwrap();
        let verify_obj = VerifyModel {
            message: request.clone().message,
            public_key_package: key_data.clone().public_key_package,
            signature: request.clone().signature
        };
        is_valid = frost_verify_signature(verify_obj);
    }

    HttpResponse::Ok().body(serde_json::to_string(&ResponseModel{
        status: is_valid,
        message: "signature verify successfull".to_string(),
        data: None
    }).unwrap())
}
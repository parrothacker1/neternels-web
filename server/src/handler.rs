#![allow(non_camel_case_types)]
use actix_web::{http::StatusCode, web, Responder,HttpRequest};
use serde::Serialize as se;
#[path = "./db.rs"]
mod db;

#[derive(se)]
enum Kernel {
    devices(db::KernelList),
    error(db::ResultRet),
}
#[derive(se)]
enum Request {
    requests(db::RequestList),
    error(db::ResultRet),
}

pub async fn add_kernel(kernel: web::Json<db::Kernel>,req:HttpRequest) -> impl Responder {
    match req.headers().get("Authorization") {
        Some(val) => {
            let auth=val.to_str().unwrap();
            let _split:Vec<&str>=auth.split("Bearer").collect();
            let token=_split[1].trim().to_string();
            let status: StatusCode;
            let result: db::ResultRet;
            match db::add_kernel(kernel,token).await {
                Ok(val) => {
                    result = val.0;
                    status = val.1;
                },
                Err(_) => {
                    result = db::ResultRet {
                        result: false,
                        note: "sql_error".to_string(),
                    };
                    status = StatusCode::from_u16(500).unwrap();
                }
            }
            (web::Json(result),status)
        },
        None => {
            let result=db::ResultRet {
                result:false,
                note:"invalid_request".to_string(),
            };
            let status=StatusCode::from_u16(400).unwrap();
            (web::Json(result),status)
        },
    }
}
pub async fn list_kernel() -> impl Responder {
    let status: StatusCode;
    let ret: Kernel;
    match db::list_kernels().await {
        Ok(val) => {
            ret = Kernel::devices(val.0);
            status = val.1;
        }
        Err(_) => {
            ret = Kernel::error(db::ResultRet {
                result: false,
                note: "sql_error".to_string(),
            });
            status = StatusCode::from_u16(500).unwrap();
        }
    };
    (web::Json(ret), status)
}
pub async fn update_kernel(kernel: web::Json<db::Kernel>,req:HttpRequest) -> impl Responder {
    match req.headers().get("Authorization") {
        Some(val) => {
            let auth=val.to_str().unwrap();
            let _split:Vec<&str>=auth.split("Bearer").collect();
            let token=_split[1].trim().to_string();
            let status: StatusCode;
            let result: db::ResultRet;
            match db::update_kernel(kernel,token).await {
                Ok(val) => {
                    result = val.0;
                    status = val.1;
                },
                Err(_) => {
                    result = db::ResultRet {
                        result: false,
                        note: "sql_error".to_string(),
                    };
                    status = StatusCode::from_u16(500).unwrap();
                },
            }
            (web::Json(result), status)
        },
        None => {
            let result=db::ResultRet {
                result:false,
                note:"invalid_request".to_string(),
            };
            let status=StatusCode::from_u16(400).unwrap();
            (web::Json(result),status)
        }
    }
}
pub async fn delete_kernel(id: web::Path<i32>,req:HttpRequest) -> impl Responder {
    match req.headers().get("Authorization") {
        Some(val) => {
            let auth=val.to_str().unwrap();
            let _split:Vec<&str>=auth.split("Bearer").collect();
            let token=_split[1].trim().to_string();
            let status: StatusCode;
            let result: db::ResultRet;
            match db::delete_kernel(*id,token).await {
                Ok(val) => {
                    result = val.0;
                    status = val.1;
                },
                Err(_) => {
                    result = db::ResultRet {
                        result: false,
                        note: "sql_error".to_string(),
                    };
                    status = StatusCode::from_u16(500).unwrap();
                }
            }
            (web::Json(result), status)
        },
        None => {
            let result=db::ResultRet {
                result:false,
                note:"invalid_request".to_string(),
            };
            let status=StatusCode::from_u16(400).unwrap();
            (web::Json(result),status)
        },
    }
}

pub async fn add_request(request: web::Json<db::Request>) -> impl Responder {
    let status: StatusCode;
    let result: db::ResultRet;
    match db::add_request(request).await {
        Ok(val) => {
            result = val.0;
            status = val.1;
        }
        Err(_) => {
            result = db::ResultRet {
                result: false,
                note: "sql_error".to_string(),
            };
            status = StatusCode::from_u16(500).unwrap();
        }
    }
    (web::Json(result), status)
}
pub async fn list_request() -> impl Responder {
    let status: StatusCode;
    let ret: Request;
    match db::list_requests().await {
        Ok(val) => {
            ret = Request::requests(val.0);
            status = val.1;
        }
        Err(_) => {
            ret = Request::error(db::ResultRet {
                result: false,
                note: "sql_error".to_string(),
            });
            status = StatusCode::from_u16(500).unwrap();
        },
    };
    (web::Json(ret), status)
}
pub async fn update_request(request: web::Json<db::Request>,req:HttpRequest) -> impl Responder {
    match req.headers().get("Authorization") {
        Some(val) => {
            let auth=val.to_str().unwrap();
            let _split:Vec<&str>=auth.split("Bearer").collect();
            let token=_split[1].trim().to_string();
            let status: StatusCode;
            let result: db::ResultRet;
            match db::update_request(request,token).await {
                Ok(val) => {
                    result = val.0;
                    status = val.1;
                },
                Err(_) => {
                    result = db::ResultRet {
                        result: false,
                        note: "sql_error".to_string(),
                    };
                    status = StatusCode::from_u16(500).unwrap();
                },
            }
            (web::Json(result), status)
        },
        None => {
            let result=db::ResultRet {
                result:false,
                note:"invalid_request".to_string(),
            };
            let status=StatusCode::from_u16(400).unwrap();
            (web::Json(result),status)
        }
    }
}
pub async fn delete_request(id: web::Path<i32>,req:HttpRequest) -> impl Responder {
    match req.headers().get("Authorization") {
        Some(val) => {
            let auth=val.to_str().unwrap();
            let _split:Vec<&str>=auth.split("Bearer").collect();
            let token=_split[1].trim().to_string();
            let status: StatusCode;
            let result: db::ResultRet;
            match db::delete_request(*id,token).await {
                Ok(val) => {
                    result = val.0;
                    status = val.1;
                },
                Err(_) => {
                    result = db::ResultRet {
                        result: false,
                        note: "sql_error".to_string(),
                    };
                    status = StatusCode::from_u16(500).unwrap();
                }
            }
            (web::Json(result), status)
        },
        None => {
            let result=db::ResultRet {
                result:false,
                note:"invalid_request".to_string(),
            };
            let status=StatusCode::from_u16(400).unwrap();
            (web::Json(result),status)
        },
    }
}

pub async fn login(info: web::Json<db::Login>) -> impl Responder {
    let ret = db::login(info).await;
    (web::Json(ret.0), ret.1)
}

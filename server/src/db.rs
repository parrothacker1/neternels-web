#![allow(unused_parens)]
#![allow(dead_code)]
use serde::{Serialize as se,Deserialize as dse};
use std::collections::HashMap;
use postgres::{Client,Error,NoTls};
use rand::{thread_rng,Rng};
// These imports are for those postgresql servers which need SSL
//use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
//use postgres_openssl::MakeTlsConnector;
use actix_rt::task;
use std::{io::{Read,Write},fs::File};
use actix_web::{http::StatusCode,web::Json};
use reqwest::Error as ReqError;

pub type KernelList=HashMap<String,Vec<Kernel>>;
pub type RequestList=Vec<Request>;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
const PASSWORD_LEN: usize = 15;

#[derive(se,dse,Debug,Clone)]
pub struct Kernel {
    id:i32,
    name:String,
    codename:String,
    company:String,
    kernel_version:String,
    last_updated:String,
    developer:String,
    download_link:String,
}
#[derive(se,dse,Debug,Clone)]
pub struct Request {
    id:i32,
    device_name:String,
    device_codename:String,
    current_kernel_version:String,
    requested_android_version:i32,
    kernel_source:String,
}
#[derive(se)]
pub struct ResultRet {
    pub result:bool,
    pub note:String,
}
#[derive(dse)]
pub struct Login {
    username:String,
    password:String,
}

fn connect() -> Result<Client,Error> {
    // Program for postgres servers which need SSL (heroku etc)
    //let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    //builder.set_verify(SslVerifyMode::NONE);
    //let tls=MakeTlsConnector::new(builder.build());
    //Client::connect(&std::env::var("DATABASE_URL").unwrap(),tls)
    Client::connect(&std::env::var("DATABASE_URL").unwrap(),NoTls)
}


#[derive(dse,Debug,Clone)]
struct User {
    login:String,
}
async fn request() -> Result<Vec<String>,ReqError> {
    let mut ret:Vec<String>=Vec::new();
    let list:Result<Vec<User>,ReqError>=task::spawn_blocking(|| {
        use reqwest::blocking::Client;
        let list_ret=Client::new()
            .get("https://api.github.com/orgs/Neternels/public_members")
            .header("User-Agent","Test")
            .send()
            .unwrap()
            .json::<Vec<User>>()
            .unwrap();
        Ok(list_ret)
    }).await.unwrap();
    for user in list.unwrap().iter() {
        let login=&user.login;
        ret.push(login.to_string());
    }
    Ok(ret)
}

fn validate_token(token:String) -> bool {
    let ret:bool;
    let mut secret_file=File::open("secret.key").unwrap();
    let mut key=String::new();
    secret_file.read_to_string(&mut key).unwrap();
    if (key==token) {
        ret=true;
    } else {
        ret=false;
    }
    ret
}

pub async fn list_kernels() -> Result<(KernelList,StatusCode),Error> {
    let status:StatusCode;
    let mut hash:KernelList=HashMap::new();
    let companies:Result<Vec<String>,Error>=task::spawn_blocking(move || {
        let mut client=connect()?;
        let mut ret:Vec<String>=Vec::new();
        for row in client.query("SELECT company FROM kernels GROUP BY company",&[])? {
            ret.push(row.get("company"));

        }
        Ok(ret)
    }).await.unwrap();
    for company in companies.unwrap() {
        let cmpny=company.clone();
        let kernels:Result<Vec<Kernel>,Error>=task::spawn_blocking(move || {
            let mut kernels:Vec<Kernel>=Vec::new();
            let mut client=connect()?;
            let cmpny=company.clone();
            for kernel in client.query("SELECT * FROM kernels WHERE company=$1",&[&company])? {
                let krnl=Kernel {
                    id:kernel.get("id"),
                    name:kernel.get("name"),
                    codename:kernel.get("codename"),
                    company:kernel.get("company"),
                    kernel_version:kernel.get("kernel_version"),
                    last_updated:kernel.get("last_updated"),
                    developer:kernel.get("developer"),
                    download_link:kernel.get("download_link"),
                };
                kernels.push(krnl);
             }
            Ok(kernels)
        }).await.unwrap();
        hash.insert(cmpny,kernels.unwrap());
    };
    status=StatusCode::from_u16(200).unwrap();
    Ok((hash,status))
}
pub async fn add_kernel(kernel:Json<Kernel>,token:String) -> Result<(ResultRet,StatusCode),Error> {
    let status:StatusCode;
    let result:ResultRet;
    let new_kernel=kernel.clone();
    let kernels:Result<Vec<String>,Error>=task::spawn_blocking(move || {
        let mut client=connect()?;
        let mut ret:Vec<String>=Vec::new();
        for row in client.query("SELECT name FROM kernels WHERE id=$1",&[&new_kernel.id])? {
            ret.push(row.get("name"));
        }
        Ok(ret)
    }).await.unwrap();
    if (validate_token(token)) {
        if (!kernels.unwrap().is_empty()) {
            result=ResultRet {
                result:false,
                note:"already_exists".to_string(),
            };
            status=StatusCode::from_u16(409).unwrap();
        } else {
            let _:Result<(),Error>=task::spawn_blocking(move || {
                let mut client=connect()?;
                client.execute("INSERT INTO kernels (id,name,codename,company,kernel_version,last_updated,developer,download_link) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",&[&new_kernel.id,&new_kernel.name,&new_kernel.codename,&new_kernel.company,&new_kernel.kernel_version,&new_kernel.last_updated,&new_kernel.developer,&new_kernel.download_link])?;
                Ok(())
            }).await.unwrap();
            result=ResultRet {
                result:true,
                note:"success".to_string(),
            };
            status=StatusCode::from_u16(200).unwrap();
        }
    } else {
        result=ResultRet {
            result:false,
            note:"auth_error".to_string(),
        };
        status=StatusCode::from_u16(401).unwrap();
    }
    Ok((result,status))
}
pub async fn update_kernel(kernel:Json<Kernel>,token:String) -> Result<(ResultRet,StatusCode),Error> {
    let status:StatusCode;
    let result:ResultRet;
    let old_kernel=kernel.clone();
    let kernels:Result<Vec<String>,Error>=task::spawn_blocking(move || {
        let mut ret:Vec<String>=Vec::new();
        let mut client=connect()?;
        for row in client.query("SELECT name FROM kernels WHERE id=$1",&[&old_kernel.id])? {
            ret.push(row.get("name"));
        }
        Ok(ret)
    }).await.unwrap();
    if (validate_token(token)) {
        if (!kernels.unwrap().is_empty()) {
            let _:Result<(),Error>=task::spawn_blocking(move || {
                let mut client=connect()?;
                client.execute("UPDATE kernels SET name=$2,codename=$3,company=$4,kernel_version=$5,last_updated=$6,developer=$7,download_link=$8 WHERE id=$1",&[&old_kernel.id,&old_kernel.name,&old_kernel.codename,&old_kernel.company,&old_kernel.kernel_version,&old_kernel.last_updated,&old_kernel.developer,&old_kernel.download_link])?;
                Ok(())
            }).await.unwrap();
            result=ResultRet {
                result:true,
                note:"success".to_string(),
            };
            status=StatusCode::from_u16(200).unwrap();
        } else {
            result=ResultRet {
                result:false,
                note:"do_not_exists".to_string(),
            };
            status=StatusCode::from_u16(404).unwrap();
        }
    } else {
        result=ResultRet {
            result:false,
            note:"auth_error".to_string(),
        };
        status=StatusCode::from_u16(401).unwrap();
    }
    Ok((result,status))
}
pub async fn delete_kernel(kernel_id:i32,token:String) -> Result<(ResultRet,StatusCode),Error> {
    let status:StatusCode;
    let result:ResultRet;
    let kernels:Result<Vec<Kernel>,Error>=task::spawn_blocking(move || {
        let mut client=connect()?;
        let mut ret:Vec<Kernel>=Vec::new();
        for kernel in client.query("SELECT * FROM kernels WHERE id=$1",&[&kernel_id])? {
            let krnl=Kernel {
                id:kernel.get("id"),
                name:kernel.get("name"),
                codename:kernel.get("codename"),
                company:kernel.get("company"),
                kernel_version:kernel.get("kernel_version"),
                last_updated:kernel.get("last_updated"),
                developer:kernel.get("developer"),
                download_link:kernel.get("download_link"),
            };
            ret.push(krnl);
        }
        Ok(ret)
    }).await.unwrap();
    if (validate_token(token)) {
        if (!kernels.unwrap().is_empty()) {
            let _:Result<(),Error>=task::spawn_blocking(move || {
                let mut client=connect()?;
                client.execute("DELETE FROM kernels WHERE id=$1",&[&kernel_id])?;
                Ok(())
            }).await.unwrap();
            result=ResultRet {
                result:true,
                note:"success".to_string(),
            };
            status=StatusCode::from_u16(200).unwrap();
        } else {
            result=ResultRet {
                    result:false,
                note:"do_not_exists".to_string(),
            };
            status=StatusCode::from_u16(404).unwrap();
        }
    } else {
        result=ResultRet {
            result:false,
            note:"auth_error".to_string(),
        };
        status=StatusCode::from_u16(401).unwrap();
    }
    Ok((result,status))
}


pub async fn list_requests() -> Result<(RequestList,StatusCode),Error> {
    let status:StatusCode;
    let list:Result<RequestList,Error>=task::spawn_blocking(|| {
        let mut client=connect()?;
        let mut ret:RequestList=Vec::new();
        for row in client.query("SELECT * FROM requests",&[])? {
            let request=Request {
                id:row.get("id"),
                device_name:row.get("device_name"),
                device_codename:row.get("device_codename"),
                current_kernel_version:row.get("current_kernel_version"),
                requested_android_version:row.get("requested_android_version"),
                kernel_source:row.get("kernel_source"),
            };
            ret.push(request);
        };
        Ok(ret)
    }).await.unwrap();
    status=StatusCode::from_u16(200).unwrap();
    Ok((list.unwrap(),status))
}
pub async fn add_request(request:Json<Request>) -> Result<(ResultRet,StatusCode),Error> {
    let status:StatusCode;
    let result:ResultRet;
    let new_request=request.clone();
    let requests:Result<Vec<i32>,Error>=task::spawn_blocking(move || {
        let mut client=connect()?;
        let mut ret:Vec<i32>=Vec::new();
        for row in client.query("SELECT id FROM requests WHERE id=$1",&[&new_request.id])? {
            ret.push(row.get("id"));
        }
        Ok(ret)
    }).await.unwrap();
    if (!requests.unwrap().is_empty()) {
        result=ResultRet {
            result:false,
            note:"already_exists".to_string(),
        };
        status=StatusCode::from_u16(409).unwrap();
    } else {
        let _:Result<(),Error>=task::spawn_blocking(move || {
            let mut client=connect()?;
            client.execute("INSERT INTO requests(id,device_name,device_codename,current_kernel_version,requested_android_version,kernel_source) VALUES ($1,$2,$3,$4,$5,$6)",&[&new_request.id,&new_request.device_name,&new_request.device_codename,&new_request.current_kernel_version,&new_request.requested_android_version,&new_request.kernel_source])?;
            Ok(())
        }).await.unwrap();
        result=ResultRet {
            result:true,
            note:"success".to_string(),
        };
        status=StatusCode::from_u16(200).unwrap();
    }
    Ok((result,status))
}
pub async fn update_request(request:Json<Request>,token:String) -> Result<(ResultRet,StatusCode),Error> {
    let status:StatusCode;
    let result:ResultRet;
    let old_request=request.clone();
    let requests:Result<Vec<String>,Error>=task::spawn_blocking(move || {
        let mut client=connect()?;
        let mut ret:Vec<String>=Vec::new();
        for row in client.query("SELECT device_name FROM requests WHERE id=$1",&[&old_request.id])? {
            ret.push(row.get("device_name"));
        }
        Ok(ret)
    }).await.unwrap();
    if (validate_token(token)) {
        if (!requests.unwrap().is_empty()) {
            let _:Result<(),Error>=task::spawn_blocking(move || {
                let mut client=connect()?;
                client.execute("UPDATE requests SET device_name=$2,device_codename=$3,current_kernel_version=$4,requested_android_version=$5,kernel_source=$6 WHERE id=$1",&[&old_request.id,&old_request.device_name,&old_request.device_codename,&old_request.current_kernel_version,&old_request.requested_android_version,&old_request.kernel_source])?;
                Ok(())
            }).await.unwrap();
            result=ResultRet {
                result:true,
                note:"success".to_string(),
            };
            status=StatusCode::from_u16(200).unwrap();
        } else {
            result=ResultRet {
                result:false,
                note:"do_not_exists".to_string(),
            };
            status=StatusCode::from_u16(404).unwrap();
        }
    } else {
        result=ResultRet {
            result:false,
            note:"auth_error".to_string(),
        };
        status=StatusCode::from_u16(401).unwrap();
    }
    Ok((result,status))
}
pub async fn delete_request(request_id:i32,token:String) -> Result<(ResultRet,StatusCode),Error> {
    let status:StatusCode;
    let result:ResultRet;
    let requests:Result<Vec<String>,Error>=task::spawn_blocking(move || {
        let mut client=connect()?;
        let mut ret:Vec<String>=Vec::new();
        for row in client.query("SELECT device_name FROM requests WHERE id=$1",&[&request_id])?{
            ret.push(row.get("device_name"));
        }
        Ok(ret)
    }).await.unwrap();
    if (validate_token(token)) {
        if (!requests.unwrap().is_empty()) {
            let _:Result<(),Error>=task::spawn_blocking(move || {
                let mut client=connect()?;
                client.execute("DELETE FROM requests WHERE id=$1",&[&request_id])?;
                Ok(())
            }).await.unwrap();
            result=ResultRet {
                result:true,
                note:"success".to_string(),
            };
            status=StatusCode::from_u16(200).unwrap();
        } else {
            result=ResultRet {
                result:false,
                note:"do_not_exists".to_string(),
            };
            status=StatusCode::from_u16(404).unwrap();
        }
    } else {
        result=ResultRet {
            result:true,
            note:"auth_error".to_string(),
        };
        status=StatusCode::from_u16(401).unwrap();
    }
    Ok((result,status))
}

pub async fn login(info:Json<Login>) -> (ResultRet,StatusCode) {
    let status:StatusCode;
    let result:ResultRet;
    match request().await  {
        Ok(val) => {
            if (val.contains(&info.username)) {
                if (info.password==String::from("test")) {
                    let mut rng=thread_rng();
                    let token:String=(0..PASSWORD_LEN)
                        .map(|_| {
                            let idx = rng.gen_range(0..CHARSET.len());
                            CHARSET[idx] as char
                        })
                    .collect();
                    let mut secret_file=File::create("secret.key").unwrap();
                    secret_file.write_all(token.as_bytes()).unwrap();
                    result=ResultRet {
                        result:true,
                        note:token,
                    };
                    status=StatusCode::from_u16(200).unwrap();
                } else {
                    result=ResultRet {
                        result:false,
                        note:"not_authenticated".to_string(),
                    };
                    status=StatusCode::from_u16(401).unwrap();
                }
            } else {
                result=ResultRet {
                    result:false,
                    note:"user_not_exists".to_string(),
                };
                status=StatusCode::from_u16(404).unwrap();
            }
        },
        Err(_) => {
            result=ResultRet {
                result:false,
                note:"request_error".to_string(),
            };
            status=StatusCode::from_u16(500).unwrap();
        },
    }
    (result,status)
}

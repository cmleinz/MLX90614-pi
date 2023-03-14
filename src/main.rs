use actix_web::{error, get, web, App, HttpServer, Responder};
use rppal::i2c::I2c;
use serde::Serialize;
use thiserror::Error;

const MLX90614_TA: u8 = 0x06;
const MLX90614_J1: u8 = 0x07;

#[derive(Serialize)]
struct Response {
    sky_temperature: f32,
    ambient_temperature: f32,
}

#[derive(Serialize, Debug, Error)]
enum Error {
    #[error("1: Failed to create I2C connection")]
    Connection,
    #[error("2: Failed to read/write to I2C bus")]
    Read,
}

impl error::ResponseError for Error {}

fn read_kelvin() -> actix_web::Result<(f32, f32), Error> {
    let i2c = match I2c::new() {
        Ok(i2c) => i2c,
        Err(_) => return Err(Error::Connection),
    };
    let mut sky_buf: [u8; 2] = [0; 2];
    if let Err(_) = i2c.write_read(&[MLX90614_J1], &mut sky_buf) {
        return Err(Error::Read);
    }
    let sky_temperature = convert(sky_buf);

    let mut ambient_buf: [u8; 2] = [0; 2];
    if let Err(_) = i2c.write_read(&[MLX90614_TA], &mut ambient_buf) {
        return Err(Error::Read);
    }
    let ambient_temperature = convert(ambient_buf);

    Ok((sky_temperature, ambient_temperature))
}

fn convert(val: [u8; 2]) -> f32 {
    let v1 = (val[1] as u16) << 8;
    let v2 = val[0] as u16;
    (v1 | v2) as f32 * 0.02
}

fn kelvin_to_celcius(k: f32) -> f32 {
    k - 273.15
}

fn kelvin_to_fahrenheit(k: f32) -> f32 {
    1.8 * kelvin_to_celcius(k) + 32.0
}

#[get("/api/kelvin")]
async fn get_kelvin() -> actix_web::Result<impl Responder, Error> {
    let (sky_temperature, ambient_temperature) = read_kelvin()?;

    let resp = Response {
        sky_temperature,
        ambient_temperature,
    };

    Ok(web::Json(resp))
}

#[get("/api/celcius")]
async fn get_celcius() -> actix_web::Result<impl Responder, Error> {
    let (sky_temperature, ambient_temperature) = read_kelvin()?;

    let resp = Response {
        sky_temperature: kelvin_to_celcius(sky_temperature),
        ambient_temperature: kelvin_to_celcius(ambient_temperature),
    };

    Ok(web::Json(resp))
}

#[get("/api/fahrenheit")]
async fn get_fahrenheit() -> actix_web::Result<impl Responder, Error> {
    let (sky_temperature, ambient_temperature) = read_kelvin()?;

    let resp = Response {
        sky_temperature: kelvin_to_fahrenheit(sky_temperature),
        ambient_temperature: kelvin_to_fahrenheit(ambient_temperature),
    };

    Ok(web::Json(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_kelvin)
            .service(get_celcius)
            .service(get_fahrenheit)
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}

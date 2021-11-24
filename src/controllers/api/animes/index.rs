extern crate diesel;

use log::{error, info};

use crate::models::Anime;
use crate::types::animes::{StrictAnime, StrictAnimes};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ResponseBody {
    animes: StrictAnimes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathParams {
    year: i32,
    season: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BodyParams {
    animes: StrictAnimes,
}

#[get("/animes")]
pub async fn get() -> impl Responder {
    let mut animes = StrictAnime::new_by_animes(Anime::all());
    animes.sort();
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}

#[get("/animes/{year}")]
pub async fn get_by_year(path_params: web::Path<PathParams>) -> impl Responder {
    let mut animes = StrictAnime::new_by_animes(Anime::find_by_year(path_params.year));
    animes.sort();
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}

#[get("/animes/{year}/{season}")]
pub async fn get_by_season(path_params: web::Path<PathParams>) -> impl Responder {
    let season = &path_params.season.clone().unwrap();
    let animes = StrictAnime::new_by_animes(Anime::find_by_season(path_params.year, &season));
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}

#[post("/animes")]
pub async fn post(body_params: web::Json<BodyParams>) -> impl Responder {
    let new_animes = &body_params.animes;
    info!("Try create new_animes: {:?}", new_animes);

    let target_animes = StrictAnime::to_new_animes(&new_animes.clone());

    if target_animes.clone().into_iter().any(|a| a.is_none()) {
        error!("Failed to convert animes: {:?}", new_animes);
        return HttpResponse::BadRequest().finish();
    }

    let created_animes = Anime::create(&target_animes.into_iter().map(|a| a.unwrap()).collect());

    if created_animes.is_err() {
        error!("Failed to create new animes: {:?}", created_animes);
        return HttpResponse::BadRequest().finish();
    }

    let animes = StrictAnime::new_by_animes(created_animes.unwrap());
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}

#[put("/animes")]
pub async fn put(body_params: web::Json<BodyParams>) -> impl Responder {
    let animes = &body_params.animes;
    info!("Try update animes: {:?}", animes);

    let mut updated_animes = vec![];

    for anime in animes {
        let target_anime = anime.clone().to_anime();

        if target_anime.is_none() {
            error!("Failed to convert an anime: {:?}", anime);
            let animes = StrictAnime::new_by_animes(updated_animes);
            return HttpResponse::BadRequest().json(ResponseBody { animes: animes });
        }

        let updated_anime = Anime::update(&target_anime.unwrap());

        if let Ok(a) = updated_anime {
            info!("Succeeded to update an anime: {:?}", anime);
            updated_animes.push(a);
        } else {
            error!(
                "Failed to update an anime: {:?} => {:?}",
                anime, updated_anime
            );
            let animes = StrictAnime::new_by_animes(updated_animes);
            return HttpResponse::BadRequest().json(ResponseBody { animes: animes });
        }
    }

    let animes = StrictAnime::new_by_animes(updated_animes);
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}

#[delete("/animes")]
pub async fn delete(body_params: web::Json<BodyParams>) -> impl Responder {
    let animes = &body_params.animes;
    info!("Try delete animes: {:?}", animes);

    let mut deleted_animes = vec![];

    for anime in animes {
        let target_anime = anime.clone().to_anime();

        if target_anime.is_none() {
            error!("Failed to convert an anime: {:?}", anime);
            let animes = StrictAnime::new_by_animes(deleted_animes);
            return HttpResponse::BadRequest().json(ResponseBody { animes: animes });
        }

        let deleted_anime = Anime::delete(&target_anime.unwrap());

        if let Ok(a) = deleted_anime {
            info!("Succeeded to delete an anime: {:?}", anime);
            deleted_animes.push(a);
        } else {
            error!(
                "Failed to delete an anime: {:?} => {:?}",
                anime, deleted_anime
            );
            let animes = StrictAnime::new_by_animes(deleted_animes);
            return HttpResponse::BadRequest().json(ResponseBody { animes: animes });
        }
    }

    let animes = StrictAnime::new_by_animes(deleted_animes);
    return HttpResponse::Ok().json(ResponseBody { animes: animes });
}